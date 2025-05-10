mod engine;
mod lua;

use gmod::{gmod13_open, lua::State};

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
  env_logger::init();

  log::info!("wasmgm is initializing...");

  match engine::init_wasm() {
    Ok(_) => log::info!("wasm engine has been initialized"),
    Err(e) => {
      log::error!("unable to initialize wasm engine: {e}");
      return 0
    }
  };

  lua::init_lua(lua);

  0
}