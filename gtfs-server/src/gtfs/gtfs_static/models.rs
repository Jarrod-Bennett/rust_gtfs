#[derive(Queryable)]
pub struct Calendar {
    pub service_id: String,
    pub monday: i32,
    pub tuesday: i32,
    pub wednesday: i32,
    pub thursday: i32,
    pub friday: i32,
    pub saturday: i32,
    pub sunday: i32,
    pub start_date: i32,
    pub end_date: i32,
}

#[derive(Queryable)]
pub struct CalendarDate {
    service_id: String,
    date: i32,
    exception_type: i32,
}

#[derive(Queryable)]
pub struct Route {
    route_id: String,
    route_short_name: i32,
    route_long_name: String,
    route_desc: Option<String>,
    route_type: i32,
    route_url: String,
    route_color: String,
    route_text_color: String,
}

#[derive(Queryable)]
pub struct StopTime {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: i32,
    stop_sequence: i32,
    pickup_type: i32,
    drop_off_type: i32,
}

#[derive(Queryable)]
pub struct Stop {
    stop_id: i32,
    stop_code: Option<i32>,
    stop_name: String,
    stop_desc: Option<String>,
    stop_lat: f32,
    stop_lon: f32,
    zone_id: Option<i32>,
    stop_url: Option<String>,
    location_type: i32,
    parent_station: Option<String>,
    platform_code: Option<String>,
}

#[derive(Queryable)]
pub struct Trip {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    direction_id: i32,
    block_id: Option<String>,
    shape_id: Option<String>,
}
