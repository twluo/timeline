use crate::models::Coordinate;

const EARTH_RADIUS_METERS: f64 = 6_371_000.0;

/// Returns the distance in meters between two coordinates using the Haversine formula.
pub fn get_distance(c1: &Coordinate, c2: &Coordinate) -> f64 {
    let lat1 = c1.latitude.to_radians();
    let lat2 = c2.latitude.to_radians();

    let delta_lat = (c2.latitude - c1.latitude).to_radians();
    let delta_lng = (c2.longitude - c1.longitude).to_radians();

    let a =
        (delta_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (delta_lng / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_METERS * c
}

/// Given a source coordinate, returns the coordinate reached by travelling
/// `distance` metres at `angle` degrees (bearing clockwise from north).
pub fn get_coordinate_from_source(source: &Coordinate, distance: f64, angle: f64) -> Coordinate {
    let lat1 = source.latitude.to_radians();
    let lng1 = source.longitude.to_radians();
    let bearing = angle.to_radians();
    let angular_dist = distance / EARTH_RADIUS_METERS;

    let lat2 =
        (lat1.sin() * angular_dist.cos() + lat1.cos() * angular_dist.sin() * bearing.cos()).asin();

    let lng2 = lng1
        + (bearing.sin() * angular_dist.sin() * lat1.cos())
            .atan2(angular_dist.cos() - lat1.sin() * lat2.sin());

    Coordinate {
        latitude: lat2.to_degrees(),
        longitude: lng2.to_degrees(),
    }
}

/// Returns the (north, south, east, west) bounding coordinates `distance` metres
/// from `source`.
pub fn get_bounding_box_from_source(
    source: &Coordinate,
    distance: f64,
) -> (Coordinate, Coordinate, Coordinate, Coordinate) {
    let lat_delta = (distance / EARTH_RADIUS_METERS).to_degrees();
    let cos_lat = source.latitude.to_radians().cos();
    // At the poles cos is zero; fall back to the full longitude range.
    let lng_delta = if cos_lat.abs() < f64::EPSILON {
        180.0
    } else {
        (distance / (EARTH_RADIUS_METERS * cos_lat)).to_degrees()
    };

    let north = Coordinate {
        latitude: source.latitude + lat_delta,
        longitude: source.longitude,
    };
    let south = Coordinate {
        latitude: source.latitude - lat_delta,
        longitude: source.longitude,
    };
    let east = Coordinate {
        latitude: source.latitude,
        longitude: source.longitude + lng_delta,
    };
    let west = Coordinate {
        latitude: source.latitude,
        longitude: source.longitude - lng_delta,
    };

    (north, south, east, west)
}
