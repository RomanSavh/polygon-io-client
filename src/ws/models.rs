use serde::{Deserialize, Serialize};
use serde_json::Error;

use super::error::PolygonWsError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsDataEvent {
    Status(StatusMessage),
    ForexQuoteTick(ForexQuoteTickMessage),
    StockQuoteTick(StockQuoteTickMessage),
}

impl WsDataEvent {
    pub fn serialize_chunk(data: &str) -> Vec<Result<Self, PolygonWsError>>{
        let mut result = vec![];
        let parse_result: Result<Vec<String>, Error> = serde_json::from_str(data);
        
        if let Ok(messages) = parse_result {
            for message in messages {
                let message_result = Self::serialize(&message);
                result.push(message_result)
            }
        }

        return result;
    }

    fn serialize(data: &str) -> Result<Self, PolygonWsError> {
        let parse_result: Result<serde_json::Value, Error> = serde_json::from_str(data);

        if let Ok(data) = parse_result {
            let evnt = data.get("ev").unwrap().to_string();

            let result = match evnt.as_str(){
                "Q" => {
                    let message = serde_json::from_value::<StockQuoteTickMessage>(data).unwrap();
                    Ok(WsDataEvent::StockQuoteTick(message))
                },
                "C" => {
                    let message = serde_json::from_value::<ForexQuoteTickMessage>(data).unwrap();
                    Ok(WsDataEvent::ForexQuoteTick(message))
                },
                "status" => {
                    let message = serde_json::from_value::<StatusMessage>(data).unwrap();
                    Ok(WsDataEvent::Status(message))
                },
                _ => Err(PolygonWsError::UnknownEventFromSocket(data.to_string()))
            };

            return result;
        }

        return Err(PolygonWsError::SerializeError(data.to_string()));
    }
}

pub enum SendEventMessage {
    Login(String),
    ForexQuotesSubscribe(Option<Vec<String>>),
    StockQuotesSubscribe(Option<Vec<String>>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocketMessage {
    pub action: String,
    pub params: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusMessage {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForexQuoteTickMessage {
    #[serde(rename = "p")]
    pub symbol: String,
    #[serde(rename = "x")]
    pub exchange_id: String,
    #[serde(rename = "a")]
    pub ask: f64,
    #[serde(rename = "b")]
    pub bid: f64,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockQuoteTickMessage {
    #[serde(rename = "sym")]
    pub symbol: String,
    #[serde(rename = "bx")]
    pub bid_exchange_id: u32,
    #[serde(rename = "bp")]
    pub bid: f64,
    #[serde(rename = "bs")]
    pub bid_size: u64,
    #[serde(rename = "ax")]
    pub ask_exchange_id: u32,
    #[serde(rename = "ap")]
    pub ask: f64,
    #[serde(rename = "as")]
    pub ask_size: u64,
    #[serde(rename = "c")]
    pub condition: u32,
    #[serde(rename = "i")]
    pub indicators: Vec<i32>,
    #[serde(rename = "q")]
    pub sequance: u64,
    #[serde(rename = "z")]
    pub tape: i32,
    #[serde(rename = "t")]
    pub timestamp: u64,
}
