use std::sync::Arc;

use my_web_socket_client::WsConnection;

use super::{error::PolygonWsError, models::WsDataEvent};

#[async_trait::async_trait]
pub trait PolygonEventHandler {
    async fn on_data(&self, event: WsDataEvent, connection: &Arc<WsConnection>);
    async fn on_connected(&self, connection: &Arc<WsConnection>);
    async fn on_disconnected(&self, connection: &Arc<WsConnection>);
    async fn on_error(&self, error: PolygonWsError, connection: &Arc<WsConnection>);
}
