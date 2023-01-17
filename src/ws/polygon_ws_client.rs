use std::sync::{atomic::AtomicBool, Arc};

use my_web_socket_client::{WebSocketClient, WsCallback, WsConnection};
use tokio_tungstenite::tungstenite::Message;

use super::{socket_settings::{PolygonWsSettings}, error::PolygonWsError, models::{SendEventMessage, WsDataEvent}, event_handler::PolygonEventHandler};

pub struct PolygonWsClient {
    ws_client: WebSocketClient,
    is_started: AtomicBool,
    socket_settings: Arc<PolygonWsSettings>,
    event_handler: Arc<dyn PolygonEventHandler + 'static + Sync + Send>,
}


impl PolygonWsClient {
    pub fn new(
        socket_settings: Arc<PolygonWsSettings>,
        event_handler: Arc<dyn PolygonEventHandler + Sync + Send + 'static>,
    ) -> Self {

        let logger = my_logger::LOGGER.clone();

        Self {
            ws_client: WebSocketClient::new("PolygonWs".to_string(), socket_settings.clone(), logger),
            is_started: AtomicBool::new(false),
            socket_settings,
            event_handler
        }
    }

    pub fn start(polygon_ws_client: Arc<PolygonWsClient>) {
        if !polygon_ws_client
            .is_started
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let ping_message = Message::Ping(vec![]);
            polygon_ws_client
                .ws_client
                .start(ping_message, polygon_ws_client.clone());
            
            polygon_ws_client
                .is_started
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub async fn on_connect(&self, connection: &Arc<WsConnection>){
        self.send_message(SendEventMessage::Login(self.socket_settings.token_key.clone()), connection).await.unwrap();
    }

    pub async fn send_message(&self, message: SendEventMessage, connection: &Arc<WsConnection>) -> Result<(), PolygonWsError>{
        connection.send_message(message.as_message().into()).await;
        Ok(())
    }
}

#[async_trait::async_trait]
impl WsCallback for PolygonWsClient {
    async fn on_connected(&self, connection: Arc<WsConnection>) {
        self.on_connect(&connection).await;
        self.event_handler.on_connected(&connection).await;
    }

    async fn on_disconnected(&self, connection: Arc<WsConnection>) {
        self.event_handler.on_disconnected(&connection).await;
    }

    async fn on_data(&self, connection: Arc<WsConnection>, data: Message) {
        match data {
            Message::Text(msg) => {
                let messages = WsDataEvent::serialize_chunk(&msg);
                for message in messages{
                    match message {
                        Ok(event) => {
                            if let Some(event) = event{
                                self.event_handler.on_data(event, &connection).await;
                            }
                        }
                        Err(err) => {
                            self.event_handler.on_error(err, &connection).await;
                            connection.disconnect().await;
                        }
                    }
                }
            }
            Message::Ping(_) => {
                connection.send_message(Message::Ping(vec![])).await;
            }
            _ => {}
        }
    }
}
