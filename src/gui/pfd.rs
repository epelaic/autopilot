/**
 * PFD for Primary Flight Display
 * Display graphically : 
 * - speed (IAS in knots)
 * - Altitude (Feets)
 * - Bank angle (deg)
 * - Pitch angle (deg)
 * - Vertical speed (feets/min)
 */
pub mod pfd {

    pub fn pfd_init() {

        println!("Start init pfd module");

        println!("End init pfd module");
    }
}

pub use pfd::pfd_init;