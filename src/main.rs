mod commute;
mod components;
mod kmb;

use commute::Route;
use components::{Components, GenComponents};

use std::boxed::Box;

use tide::{Body, Request, Response, Result as TideResult};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    tide::log::start();
    let components_state = components::gen_components();
    let mut app = tide::with_state(components_state);

    /// company -> route number -> direction (in/outbound) -> stop -> eta
    app.at("/companies/:company/routes/:route")
        .get(get_bus_route);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn get_bus_route(req: Request<GenComponents<_>>) -> TideResult<Response> {
    let get_route_func = |req: Request<Components>| -> Result<Route> {
        let kmb_client = &req.state().kmb_client;
        let route_name: String = req.param("route")?;
        let bus_route = kmb_client.get_route(route_name)?;
        Ok(bus_route)
    };
    match get_route_func(req) {
        Ok(route) => {
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&route)?);
            Ok(res)
        }
        Err(e) => {
            let mut res = Response::new(500);
            res.set_body(format!("{}", e));
            Ok(res)
        }
    }
}
