pub mod graphics;

#[cfg(feature = "log-impl")]
pub mod log;

pub use graphics::*;

use std::ffi::CString;

pub mod date {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }
}