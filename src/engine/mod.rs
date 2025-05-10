pub mod modules;

use std::ffi::OsStr;
use std::sync::LazyLock;
use anyhow::{anyhow, Result};
use wasmtime::{Engine, Module, Store, Caller, Linker};

static WASM_ENGINE: LazyLock<Engine> = LazyLock::new(|| Engine::default());

pub fn load(module: &modules::WasmModule) -> Result<()> {
  log::info!("loading module {:?}", module.path.file_name().unwrap());

  let engine = &*WASM_ENGINE;
  let mut store = Store::new(engine, ());
  let mut linker = Linker::new(engine);

  linker.func_wrap("env", "log", |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
    let memory = match caller.get_export("memory") {
      Some(wasmtime::Extern::Memory(m)) => m,
      _ => {
        log::error!("no memory found in module for log");
        return;
      }
    };

    let data = memory
      .data(&caller)
      .get(ptr as usize..(ptr + len) as usize)
      .map(|s| String::from_utf8_lossy(s).to_string());

    match data {
      Some(s) => log::info!("[wasm-runtime] {}", s),
      None => log::error!("log: failed to read memory at {}..{}", ptr, len),
    }
  })?;

  let wasm_module = Module::from_file(engine, &module.path).map_err(|e| {
    anyhow!(
      "unable to open module {:?}: {e}",
      module.path.file_name().unwrap_or_else(|| OsStr::new("Unknown file"))
    )
  })?;

  let instance = linker.instantiate(&mut store, &wasm_module).map_err(|e| {
    anyhow!(
      "failed to instantiate module {:?}: {e}",
      module.path.file_name().unwrap_or_else(|| OsStr::new("Unknown file"))
    )
  })?;

  let func = instance
    .get_typed_func::<(), ()>(&mut store, "main")
    .map_err(|e| {
      anyhow!(
        "function 'main' not found in module {:?}: {e}",
        module.path.file_name().unwrap_or_else(|| OsStr::new("Unknown file"))
      )
    })?;

  func.call(&mut store, ())?;

  Ok(())
}

pub fn init_wasm() -> Result<()> {
  log::info!("starting wasm engine");

  let _store = Store::new(&*WASM_ENGINE, ());

  let modules = modules::get_module_list()
    .map_err(|_| anyhow!("unable to get list of installed modules"))?;

  if modules.is_empty() {
    log::debug!("no modules for loading");
    return Ok(());
  }

  modules.into_iter().try_for_each(|module| load(&module))?;

  Ok(())
}