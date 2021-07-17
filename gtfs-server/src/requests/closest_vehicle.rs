//! Request to find the closest service to a set of coordinates from a collection of FeedEntities.

use crate::gtfs::gtfs_real_time::FeedEntity;

pub fn find_closest(entities: &Vec<FeedEntity>, lat: f32, lon: f32) -> Result<&FeedEntity, ()> {
    let mut closest_entity = None;
    let mut distance: Option<f32> = None;

    for entity in entities {
        // get entity coordinates, or skip if no coordinates
        let position = match entity.vehicle.as_ref().and_then(|x| x.position.as_ref()) {
            Some(x) => x,
            _ => continue,
        };

        let entity_lat = position.latitude;
        let entity_lon = position.longitude;

        // if entity is closer, then update var
        let entity_dist =
            get_distance_kms_between_gps_coordinates_haversine(entity_lat, entity_lon, lat, lon);

        match distance {
            None => distance = Some(entity_dist),
            Some(d) => {
                if entity_dist < d {
                    distance = Some(entity_dist);
                    closest_entity = Some(entity);
                }
            }
        }
    }

    match closest_entity {
        Some(entity) => Ok(entity),
        None => Err(()),
    }
}

/// return the naive distance in kilometres for a latitude/longitude difference.
/// naive distance implementation with scaling factor for conversion.
fn get_distance_kms_between_gps_coordinates_naive(
    lat_1: f32,
    lon_1: f32,
    lat_2: f32,
    lon_2: f32,
) -> f32 {
    // todo: figure out constant coefficient what it should be

    let d_lat = lat_2 - lat_1;
    let d_lon = lon_2 - lon_1;
    (d_lat.powi(2) + d_lon.powi(2)).sqrt() * 98.459_288_21
}

/// return the haversine distance in kilometres for a latitude/longitude difference
fn get_distance_kms_between_gps_coordinates_haversine(
    lat_1: f32,
    lon_1: f32,
    lat_2: f32,
    lon_2: f32,
) -> f32 {
    const EARTH_RADIUS_KM: f32 = 6371.0;

    let lat_1_rad = lat_1 * std::f32::consts::PI / 180.0;
    let lat_2_rad = lat_2 * std::f32::consts::PI / 180.0;

    let d_lat = (lat_2 - lat_1) * std::f32::consts::PI / 180.0;
    let d_lon = (lon_2 - lon_1) * std::f32::consts::PI / 180.0;

    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + (d_lon / 2.0).sin() * (d_lon / 2.0).sin() * lat_1_rad.cos() * lat_2_rad.cos();

    let sqrt_a = a.sqrt();
    let sqrt_1_minus_a = (1.0 - a).sqrt(); // test todo
    let c = 2.0 * sqrt_a.atan2(sqrt_1_minus_a);

    EARTH_RADIUS_KM * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_kms_1() {
        assert_eq!(
            get_distance_kms_between_gps_coordinates_naive(3.0, 4.0, 2.0, 1.0),
            5.0
        )
    }
}
