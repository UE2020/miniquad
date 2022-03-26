pub mod graphics;

#[cfg(feature = "log-impl")]
pub mod log;

pub use graphics::*;

use std::ffi::CString;

pub use gl;

pub mod clipboard {
    use crate::graphics::*;

    use clipboard::*;

    static mut CLIPBOARD_CTX: Option<ClipboardContext> = None;

    pub fn ctx() -> &'static mut ClipboardContext {
        unsafe { CLIPBOARD_CTX.as_mut().unwrap_or_else(|| {
            let ctx = ClipboardContext::new().unwrap();
            CLIPBOARD_CTX = Some(ctx);
            CLIPBOARD_CTX.as_mut().unwrap()
        }) }
    }

    pub fn get(_ctx: &mut Context) -> Option<String> {
        ctx().get_contents().ok()
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        ctx().set_contents(data.to_string()).unwrap();
    }
}

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