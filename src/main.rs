

struct Waypoint {
    name: String,
    longitude: f64,
    latitude: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self {
            start,
            end
        }
    }
}

fn main() {

    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle = Waypoint {
        name: "KCLE".to_string(),
        longitude: -81.851111,
        latitude: 41.4075
    };
    let kslc = Waypoint {
        name: "KSLC".to_string(),
        longitude: -111.9822,
        latitude: 40.7861
    };

    let kcle_kslc = Segment::new(kcle, kslc);

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in kcle_kslc {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
                    + previous_waypoint_radians.cos() * waypoint_radians.cos()
                    * f64::powi((delta_longitude / 2.0).sin(),2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!("The distance between {} and {} is {:.1} kilometers",
                         previous_waypoint_value.0, waypoint.0, distance);
            }
        }
    }

    println!("\nThe total distance between the two points is {:.1} kilometers", total_distance);
}