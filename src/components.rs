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
/*
Using generic for abstraction + static dispatching i/o dynamic dispatching above
*/
pub struct GenComponents<K>
where
    K: commute::KmbClient,
{
    // cannot use Rc as as it's :x: Send as a shared state of tide
    // Will make kmb client clonable when multiple instances/refs are required
    // pub kmb_client: K,
    pub commuter_service: commute::CommuterService<K>,
}

pub fn gen_components() -> GenComponents<kmb::HttpClient> {
    let k = kmb::HttpClient::new();
    let s = commute::CommuterService::new(k);
    GenComponents {
        commuter_service: s,
    }
}
