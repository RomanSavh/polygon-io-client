use my_web_socket_client::WsClientSettings;

pub struct PolygonWsSettings{
    pub url: String,
    pub token_key: String
}

#[async_trait::async_trait]
impl WsClientSettings for PolygonWsSettings{
    async fn get_url(&self) -> String {
        return self.url.clone();
    }
}