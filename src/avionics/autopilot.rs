
pub mod autopilot {

    use crate::{flight_ctrl::FlightCtrlsProvider};

    pub fn autopilot_init(flcs: Box::<dyn FlightCtrlsProvider>) -> Autopilot {

        println!("Start init autopilot module");

        println!("End init autopilot module");

        Autopilot{engaged: false, flcs: flcs}
    }

    pub struct Autopilot {

        engaged: bool,
        flcs: Box::<dyn FlightCtrlsProvider>,
    }

}

pub use autopilot::autopilot_init;
