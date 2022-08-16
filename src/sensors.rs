pub mod sensors {

    pub trait  SensorsProvider {

        fn acquire(&self);
    }
}

pub use sensors::SensorsProvider;
