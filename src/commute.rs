use crate::Result;
use datetime::LocalDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Route {
    name: String,
    bounds: Vec<Bound>,
}

impl Route {
    pub fn new(name: String, bounds: Vec<Bound>) -> Self {
        Self { name, bounds }
    }
}

#[derive(Debug, Serialize)]
pub struct Bound {
    from: String,
    to: String,
    is_special: bool,
    stops: Vec<Stop>,
    bound: u8,
    service_type: u8,
}

impl Bound {
    pub fn new(
        from: String,
        to: String,
        is_special: bool,
        stops: Vec<Stop>,
        bound: u8,
        service_type: u8,
    ) -> Self {
        Self {
            from,
            to,
            is_special,
            stops,
            bound,
            service_type,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Stop {
    name: String,
    index: u8,
}

impl Stop {
    pub fn new(name: String, index: u8) -> Stop {
        Stop { name, index }
    }
}

#[derive(Debug)]
pub struct ArrivalTime {
    is_scheduled: bool,
    time: LocalDateTime,
}

pub trait KmbClient {
    fn get_route(&self, route: String) -> Result<Route>;
    fn get_arrival_times(
        &self,
        route: String,
        bound: u8,
        service_type: u8,
        stop_index: u8,
    ) -> Vec<ArrivalTime>;
}
