pub mod commute;
mod kmb;
use std::boxed::Box;

use tide::{Body, Request, Response, Result as TideResult};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    tide::log::start();
    let state = State {
        kmb_client: kmb::provide_kmb_client(),
    };
    let mut app = tide::with_state(state);
    app.at("/routes/:name").get(get_bus_route);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

struct State {
    kmb_client: Box<dyn kmb::KmbClient + Send + Sync>,
}

async fn get_bus_route(req: Request<State>) -> TideResult<Response> {
    let get_route_func = |req: Request<State>| -> Result<commute::Route> {
        let kmb_client = &req.state().kmb_client;
        let route_name: String = req.param("name")?;
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
