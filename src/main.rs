// #![allow(unused_variables)]

enum NavigationAids {
    NBD,
    VOR,
    VORDME,
    FIX {name: String, latitude: f32, longitude: f32}
}

trait Quack {
    fn quack(&self);
}
fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter == 100 {
            break;
        }
    }
    /*

        println!("NDB:\t{}", NavigationAids::NBD as u8);
        println!("VOR:\t{}", NavigationAids::VOR as u8);
        println!("VORDME:\t{}", NavigationAids::VORDME as u8);

        let phrase= String::from("Duck Airlines");
        let letter = phrase.chars().nth(5);
        const EARTH_RADIUS_IN_KM: f64 = 6371.0;
        let kcle_latitude_degrees: f64 = 41.4075;
        let kcle_longitde_degrees: f64 = -84.851111;

        let kslc_latitude_degrees: f64 = 41.7861;
        let kslc_longitde_degrees: f64 = -111.9822;

        let kcle_latitude_radians = kcle_latitude_degrees.to_radians();
        let kslc_latitude_radians = kslc_latitude_degrees.to_radians();

        let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
        let delta_longitude = (kcle_longitde_degrees - kslc_longitde_degrees).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
            + kcle_latitude_radians.cos()
            * kslc_latitude_radians.cos()
            * f64::powi((delta_longitude / 2.0).sin(),2);

        let central_angle = 2.0 * inner_central_angle.sqrt().asin();

        let distance = EARTH_RADIUS_IN_KM * central_angle;
        println!("The distance between points is {} km", distance);

    let available_aircrafts = "F-26";
    let minimum_crew = 7;
    let available_crew = 7;

    if available_aircrafts == "Boeing" || available_aircrafts == "Airbus" {
        if available_crew >= minimum_crew {
            println!(" The flight is ready to go!")
        } else {
            println!("there is not enough crew members to run flight")
        }
    } else {
        println!("Plane is not ready")
    }

     */
}

