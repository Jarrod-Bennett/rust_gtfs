CREATE TABLE calendar (
    service_id  TEXT    NOT NULL PRIMARY KEY,
    monday      INTEGER NOT NULL,
    tuesday     INTEGER NOT NULL,
    wednesday   INTEGER NOT NULL,
    thursday    INTEGER NOT NULL,
    friday      INTEGER NOT NULL,
    saturday    INTEGER NOT NULL,
    sunday      INTEGER NOT NULL,
    start_date  INTEGER NOT NULL,
    end_date    INTEGER NOT NULL
);

CREATE TABLE calendar_dates (
    service_id      TEXT    NOT NULL PRIMARY KEY,
    date            INTEGER NOT NULL,
    exception_type  INTEGER NOT NULL
);

CREATE TABLE routes (
      route_id	        TEXT    NOT NULL UNIQUE PRIMARY KEY,
      route_short_name	INTEGER NOT NULL,
      route_long_name	TEXT    NOT NULL,
      route_desc	    TEXT,
      route_type	    INTEGER NOT NULL,
      route_url	        TEXT    NOT NULL,
      route_color	    TEXT    NOT NULL,
      route_text_color	TEXT    NOT NULL
);

CREATE TABLE stop_times (
      trip_id	        TEXT    NOT NULL PRIMARY KEY,
      arrival_time	    TEXT    NOT NULL,
      departure_time	TEXT    NOT NULL,
      stop_id	        INTEGER NOT NULL,
      stop_sequence	    INTEGER NOT NULL,
      pickup_type	    INTEGER NOT NULL,
      drop_off_type	    INTEGER NOT NULL
);

CREATE TABLE stops (
     stop_id	    INTEGER NOT NULL UNIQUE PRIMARY KEY,
     stop_code	    INTEGER UNIQUE,
     stop_name	    TEXT NOT NULL,
     stop_desc	    TEXT,
     stop_lat	    REAL NOT NULL,
     stop_lon	    REAL NOT NULL,
     zone_id	    INTEGER,
     stop_url	    TEXT,
     location_type	INTEGER NOT NULL,
     parent_station	TEXT,
     platform_code	TEXT
);

CREATE TABLE trips (
     route_id       TEXT    NOT NULL,
     service_id	    TEXT    NOT NULL,
     trip_id	    TEXT    NOT NULL UNIQUE PRIMARY KEY,
     trip_headsign	TEXT    NOT NULL,
     direction_id	INTEGER NOT NULL,
     block_id	    TEXT,
     shape_id	    TEXT
);