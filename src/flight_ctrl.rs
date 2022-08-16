pub mod flight_ctrls {

    pub trait  FlightCtrlsProvider {

        fn send(&self);
    }
}

pub use flight_ctrls::FlightCtrlsProvider;
