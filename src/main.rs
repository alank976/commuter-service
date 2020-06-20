pub mod commute;
mod kmb;

fn main() {
    println!("Hello, world!");
    try_run();
}

fn try_run() {
    let kmb_client = kmb::provider_kmb_client();
    let route = kmb_client
        .get_route("269c".to_owned())
        .expect("KMB API fails at the moment");
    println!("route={:?}", route);
}
