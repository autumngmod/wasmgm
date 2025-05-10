use std::{fs, path::PathBuf, sync::LazyLock};
use anyhow::Result;

static MODULE_STORAGE: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("garrysmod/data/wasmgm"));

pub struct WasmModule {
  pub path: PathBuf,
}

pub fn get_module_list() -> Result<Vec<WasmModule>> {
  if !MODULE_STORAGE.is_dir() {
    fs::create_dir_all(&*MODULE_STORAGE)?;

    log::debug!("module directory has been created");

    return Ok(Vec::new());
  }

  let mut modules = Vec::new();

  let files = fs::read_dir(&*MODULE_STORAGE)?;
  for entry in files {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
      modules.push(WasmModule { path });
    }
  }

  Ok(modules)
}

pub fn get_module(name: &str) -> Result<WasmModule> {
  let module_path = MODULE_STORAGE.join(&name);
  if module_path.is_file() {
    Ok(WasmModule { path: module_path })
  } else {
    Err(anyhow::anyhow!("Module with name '{}' not found", name))
  }
}