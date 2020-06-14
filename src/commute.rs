use datetime::LocalDateTime;

pub struct Route {
    name: String,
    bounds: Vec<Bound>,
}

struct Bound {
    from: String,
    to: String,
    is_special: bool,
    stops: Vec<Stop>,
    bound: u8,
    service_type: u8,
}

struct Stop {
    name: String,
    index: u8,
}

pub struct ArrivalTime {
    is_scheduled: bool,
    time: LocalDateTime,
}
