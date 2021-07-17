table! {
    calendar (service_id) {
        service_id -> Text,
        monday -> Int4,
        tuesday -> Int4,
        wednesday -> Int4,
        thursday -> Int4,
        friday -> Int4,
        saturday -> Int4,
        sunday -> Int4,
        start_date -> Int4,
        end_date -> Int4,
    }
}

table! {
    calendar_dates (service_id) {
        service_id -> Text,
        date -> Int4,
        exception_type -> Int4,
    }
}

table! {
    routes (route_id) {
        route_id -> Text,
        route_short_name -> Int4,
        route_long_name -> Text,
        route_desc -> Nullable<Text>,
        route_type -> Int4,
        route_url -> Text,
        route_color -> Text,
        route_text_color -> Text,
    }
}

table! {
    stop_times (trip_id) {
        trip_id -> Text,
        arrival_time -> Text,
        departure_time -> Text,
        stop_id -> Int4,
        stop_sequence -> Int4,
        pickup_type -> Int4,
        drop_off_type -> Int4,
    }
}

table! {
    stops (stop_id) {
        stop_id -> Int4,
        stop_code -> Nullable<Int4>,
        stop_name -> Text,
        stop_desc -> Nullable<Text>,
        stop_lat -> Float4,
        stop_lon -> Float4,
        zone_id -> Nullable<Int4>,
        stop_url -> Nullable<Text>,
        location_type -> Int4,
        parent_station -> Nullable<Text>,
        platform_code -> Nullable<Text>,
    }
}

table! {
    trips (trip_id) {
        route_id -> Text,
        service_id -> Text,
        trip_id -> Text,
        trip_headsign -> Text,
        direction_id -> Int4,
        block_id -> Nullable<Text>,
        shape_id -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(calendar, calendar_dates, routes, stop_times, stops, trips,);
