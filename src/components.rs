/// The only one who knows both domain and "details" world
use crate::{commute, kmb};
pub struct Components {
    pub kmb_client: Box<dyn commute::KmbClient + Send + Sync>,
}

impl Components {
    pub fn new() -> Self {
        Components {
            kmb_client: Box::new(kmb::HttpClient::new()),
        }
    }
}
