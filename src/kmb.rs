use crate::commute::{ArrivalTime, Bound, KmbClient, Route, Stop};
use crate::Result;
use isahc::prelude::*;
use serde;

pub struct HttpClient {
    basic_info_url: &'static str,
    arrival_time_url: &'static str,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            basic_info_url: "http://search.kmb.hk/KMBWebSite/Function/FunctionRequest.ashx",
            arrival_time_url: "http://etav3.kmb.hk/?action=geteta&lang=en&route={}&bound={}&stop_seq={}&servicetype=01",
        }
    }

    fn get_stops(&self, route: &str, bound: u8, service_type: u8) -> Result<StopsAndBasicInfo> {
        let stops_response = isahc::get(format!(
            "{}?action={}&route={}&bound={}&serviceType={}",
            self.basic_info_url, "getstops", route, bound, service_type
        ))?
        .json::<GetStopResponse>()?;

        Ok(StopsAndBasicInfo {
            origin: stops_response.data.basic_info.ori_e_name,
            destination: stops_response.data.basic_info.dest_e_name,
            is_special: stops_response.data.basic_info.special == "N".to_owned(),
            stops: stops_response
                .data
                .route_stops
                .into_iter()
                .enumerate()
                .map(|(i, stop)| Stop::new(stop.e_name, i as u8))
                .collect(),
        })
    }
}

impl KmbClient for HttpClient {
    fn get_route(&self, route: String) -> Result<Route> {
        let route_bound_response: GetRouteBoundResponse = isahc::get(format!(
            "{}?action={}&route={}",
            self.basic_info_url, "getroutebound", &route
        ))?
        .json::<GetRouteBoundResponse>()?;

        let bounds: Result<Vec<Bound>> = if route_bound_response.result {
            let bounds_result: Result<Vec<Bound>> = route_bound_response
                .data
                .into_iter()
                .map(|bound_data| {
                    let stops_and_basic_info =
                        self.get_stops(&route, bound_data.bound, bound_data.service_type)?;
                    Ok(Bound::new(
                        stops_and_basic_info.origin,
                        stops_and_basic_info.destination,
                        stops_and_basic_info.is_special,
                        stops_and_basic_info.stops,
                        bound_data.bound,
                        bound_data.service_type,
                    ))
                })
                .collect();
            bounds_result
        } else {
            Ok(Vec::new())
        };
        bounds.map(|bounds| Route::new(route, bounds))
    }

    fn get_arrival_times(
        &self,
        _: std::string::String,
        _: u8,
        _: u8,
        _: u8,
    ) -> std::vec::Vec<ArrivalTime> {
        todo!()
    }
}

struct StopsAndBasicInfo {
    origin: String,
    destination: String,
    is_special: bool,
    stops: Vec<Stop>,
}

#[derive(Debug, serde::Deserialize)]
struct GetRouteBoundResponse {
    data: Vec<GetRouteBoundData>,
    result: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct GetRouteBoundData {
    service_type: u8,
    bound: u8,
    route: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetStopResponse {
    data: GetStopResponseData,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetStopResponseData {
    basic_info: BoundStopBasicInfo,
    route_stops: Vec<RouteStop>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BoundStopBasicInfo {
    dest_e_name: String,
    ori_e_name: String,
    special: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RouteStop {
    e_name: String,
}
