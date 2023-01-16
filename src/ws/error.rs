#[derive(Debug)]
pub enum PolygonWsError{
    SocketNotConnected,
    UnknownEventFromSocket(String),
    SerializeError(String)
}