use super::commute::{ArrivalTime, Route};

trait KmbHttpClient {
    fn get_route(route: String) -> Route;
    fn get_arrival_times(
        route: String,
        bound: u8,
        service_type: u8,
        stop_index: u8,
    ) -> Vec<ArrivalTime>;
}
