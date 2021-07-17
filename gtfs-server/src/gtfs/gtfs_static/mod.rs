// GTFS static manager, maintaining and validating the static database and querying the database

pub mod models;
pub mod schema;

use crate::gtfs::gtfs_static::models::Calendar;
use chrono::prelude::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io::{BufRead, BufReader, Error};
use std::num::ParseIntError;

/// GTFS-static associated errors.
#[derive(Debug)]
pub enum GtfsStaticError {
    ExpiredDataset(Date<Local>),
    MissingDatabase,
    StaticFileError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
    ParseNoneError,
    DatabaseConnectionError(diesel::ConnectionError),
}

impl std::error::Error for GtfsStaticError {}

impl std::fmt::Display for GtfsStaticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GtfsStaticError::*;
        match self {
            ExpiredDataset(expiry_date) => {
                let now = Local::now().date();
                write!(
                    f,
                    "Static database expired {:}, currently {:}",
                    expiry_date.to_string(),
                    now.to_string()
                )
            }
            MissingDatabase => write!(f, "Could not find database!"),
            StaticFileError(io_error) => write!(f, "Error with static file!\n{:}", io_error),
            ParseIntError(err) => write!(f, "Unable to parse GTFS-static file!\n{:}", err),
            ParseNoneError => write!(f, "Unable to parse GTFS-static file!\nParsed None"),
            DatabaseConnectionError(err) => write!(f, "Unable to connect to database!\n{:}", err),
        }
    }
}

impl From<std::io::Error> for GtfsStaticError {
    fn from(e: Error) -> Self {
        GtfsStaticError::StaticFileError(e)
    }
}

impl From<ParseIntError> for GtfsStaticError {
    fn from(e: ParseIntError) -> Self {
        GtfsStaticError::ParseIntError(e)
    }
}

// impl From<NoneError> for GtfsStaticError {
//
// }

/// Validate the current static database.
///     - Ensure the static database has not expired.
///     - Ensure a connection can be established to the static database.
/// If the static database cannot be validated for any of these reasons, an appropriate error is
/// returned. Users may then wish to respond to these errors by downloading fresh static data and/or
/// regenerating the database.
pub fn validate_static_database(
    static_file_path: &str,
    static_database_path: &str,
) -> Result<(), GtfsStaticError> {
    // check current time on static files and compare it to expiry
    // if expired, download new files and regenerate database.
    Ok(())
}

/// Rebuild the database from a directory containing (either the zip file or unzipped files),
/// overwriting the old (if any) database completely.
/// todo! could rename to .old?
pub fn generate_database(static_file_path: &str) -> Result<(), GtfsStaticError> {
    use crate::gtfs::gtfs_static::schema::calendar::dsl::*;

    let conn = establish_connection()?;

    let mut calendar_dir = String::from(static_file_path);
    calendar_dir.push_str("f");
    let calendar_path = std::path::Path::new(&calendar_dir);
    let calendar_file = std::fs::File::open(calendar_path)?;
    let reader = BufReader::new(calendar_file);

    for line in reader.lines().skip(1) {
        use GtfsStaticError::ParseNoneError;
        let line = line?;

        let mut split = line.split(",");
        let entry = Calendar {
            service_id: String::from(split.nth(0).ok_or_else(|| ParseNoneError)?),
            monday: split.nth(1).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            tuesday: split.nth(2).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            wednesday: split.nth(3).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            thursday: split.nth(4).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            friday: split.nth(5).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            saturday: split.nth(6).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            sunday: split.nth(7).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            start_date: split.nth(8).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
            end_date: split.nth(9).ok_or_else(|| ParseNoneError)?.parse::<i32>()?,
        };


    }

    Ok(())
}

/// Connect to the database.
fn establish_connection() -> Result<PgConnection, GtfsStaticError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(GtfsStaticError::DatabaseConnectionError(e)),
    }
}
