
pub mod autopilot {

    use std::sync::Arc;

    use crate::{flight_ctrl::FlightCtrlsProvider};

    pub fn autopilot_init(flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>) -> Autopilot {

        println!("Start init autopilot module");

        println!("End init autopilot module");

        Autopilot{engaged: false, flcs: flcs}
    }

    pub struct Autopilot {

        engaged: bool,
        flcs: Arc::<dyn FlightCtrlsProvider + Send + Sync>,
    }

}

pub use autopilot::autopilot_init;
