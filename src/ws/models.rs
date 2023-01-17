use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};

use super::error::PolygonWsError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsDataEvent {
    Status(StatusMessage),
    ForexQuoteTick(ForexQuoteTickMessage),
    StockQuoteTick(StockQuoteTickMessage),
}

impl WsDataEvent {
    pub fn serialize_chunk(data: &str) -> Vec<Result<Option<Self>, PolygonWsError>>{
        let mut result = vec![];
        let parse_result: Result<Vec<Value>, Error> = serde_json::from_str(data);
        
        if let Ok(messages) = parse_result {
            for message in messages {
                let message_result = Self::serialize(message);
                result.push(message_result)
            }
        }

        return result;
    }

    fn serialize(data: Value) -> Result<Option<Self>, PolygonWsError> {
        let parse_result: Result<serde_json::Value, Error> = serde_json::from_value(data.clone());

        if let Ok(parse_data) = parse_result {
            let evnt = parse_data["ev"].as_str();

            if let None =  evnt{
                return Err(PolygonWsError::UnknownEventFromSocket(data.to_string()));
            }

            let result = match evnt.unwrap(){
                "Q" => {
                    if parse_data["ap"].as_f64().is_none() || parse_data["bp"].as_f64().is_none(){
                        return Ok(None);
                    }

                    let message = serde_json::from_value::<StockQuoteTickMessage>(parse_data);
                    match message{
                        Ok(message) => Ok(Some(WsDataEvent::StockQuoteTick(message))),
                        Err(err) => {
                            println!("Error: {}", err);
                            Err(PolygonWsError::SerializeError(data.to_string()))
                        },
                    }
                    
                },
                "C" => {
                    let message = serde_json::from_value::<ForexQuoteTickMessage>(parse_data);
                    match message{
                        Ok(message) => Ok(Some(WsDataEvent::ForexQuoteTick(message))),
                        Err(_) => Err(PolygonWsError::SerializeError(data.to_string())),
                    }
                },
                "status" => {
                    let message = serde_json::from_value::<StatusMessage>(parse_data);
                    match message{
                        Ok(message) => Ok(Some(WsDataEvent::Status(message))),
                        Err(_) => Err(PolygonWsError::SerializeError(data.to_string())),
                    }
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
    pub exchange_id: u32,
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
    pub bid_exchange_id: Option<u32>,
    #[serde(rename = "bp")]
    pub bid: f64,
    #[serde(rename = "bs")]
    pub bid_size: u64,
    #[serde(rename = "ax")]
    pub ask_exchange_id: Option<u32>,
    #[serde(rename = "ap")]
    pub ask: f64,
    #[serde(rename = "as")]
    pub ask_size: u64,
    #[serde(rename = "c")]
    pub condition: Option<u32>,
    #[serde(rename = "i")]
    pub indicators: Option<Vec<i32>>,
    #[serde(rename = "q")]
    pub sequance: u64,
    #[serde(rename = "z")]
    pub tape: i32,
    #[serde(rename = "t")]
    pub timestamp: u64,
}
