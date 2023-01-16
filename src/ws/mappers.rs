use tokio_tungstenite::tungstenite::Message;

use super::models::{SendEventMessage, SocketMessage};

impl SendEventMessage {
    pub fn as_message(self) -> SocketMessage {
        match self {
            SendEventMessage::Login(login) => SocketMessage {
                action: "auth".to_string(),
                params: login,
            },
            SendEventMessage::ForexQuotesSubscribe(subscribe) => {
                let tickers = match subscribe {
                    Some(tickers_list) => tickers_list.join(","),
                    None => "*".to_string(),
                };

                SocketMessage {
                    action: "subscribe".to_string(),
                    params: format!("C.{}", tickers),
                }
            }
            SendEventMessage::StockQuotesSubscribe(subscribe) => {
                let tickers = match subscribe {
                    Some(tickers_list) => tickers_list.join(","),
                    None => "*".to_string(),
                };

                SocketMessage {
                    action: "subscribe".to_string(),
                    params: format!("Q.{}", tickers),
                }
            }
        }
    }
}




impl Into<Message> for SocketMessage {
    fn into(self) -> Message {
        return Message::Text(serde_json::to_string(&self).unwrap());
    }
}