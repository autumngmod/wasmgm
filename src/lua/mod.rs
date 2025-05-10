use gmod::{lua::State, lua_string};

use crate::engine;

pub unsafe extern "C-unwind" fn load(lua: State) -> i32 {
  let module_name = lua.check_string(1).to_string();

  match engine::modules::get_module(&module_name) {
    Ok(module) => {
      // todo
      #[allow(unused)]
      engine::load(&module);
    },
    Err(e) => {
      lua.error(format!("unknown module {module_name}: {e}"));
    }
  }

  0
}

pub unsafe fn init_lua(lua: State) {
  log::info!("setup lua functions");

  lua.get_global(lua_string!("wasmgm"));

  if lua.is_nil(-1) {
    lua.pop();

    lua.new_table();
  }

  lua.push_function(load);
  lua.set_field(-2, lua_string!("load"));

  lua.set_global(lua_string!("wasmgm"));
}