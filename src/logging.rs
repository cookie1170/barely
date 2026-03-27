#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn init() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let env = env_logger::Env::new().default_filter_or("info");
        env_logger::init_from_env(env);
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }
}
