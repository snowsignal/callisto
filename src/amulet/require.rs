use mlua::*;

use super::registry::{get_registry, LOADED_MOD_REGISTRY};

pub fn bind_require(lua: &mlua::Lua) {
    let func = lua.create_function(|lua: &Lua, package: std::string::String| -> Result<mlua::Value> {
        let loaded_modules: mlua::Table = get_registry(lua, LOADED_MOD_REGISTRY)?;
        let package = package.replace(".", "/") + ".lua";
        match loaded_modules.get(package.as_str())? {
            mlua::Nil => {
                println!("Loading");

                let export = lua.create_table()?;

                loaded_modules.set(package.as_str(), export.clone())?;

                let contents = std::fs::read_to_string(package.clone()).unwrap();

                // Call module and pass in export table
                let pkg: mlua::Value = lua.load(&contents).call(export.clone())?;
                match pkg {
                    mlua::Nil => {},
                    pkg => loaded_modules.set(package.as_str(), pkg)?,
                };
                loaded_modules.get(package.as_str())
            },
            val => {
                println!("Already loaded");
                Ok(val)
            }
        }
    }).unwrap();

    lua.globals().set("require", func).unwrap();
}