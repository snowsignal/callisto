mod registry;
mod require;
mod util;
mod window;

use mlua::*;
use registry::*;

use self::require::bind_require;

pub fn load_amulet(lua: &Lua) -> Result<()> {
    // Load the amulet functions into memory, using Callisto backends

    // Initialize basic state
    reserve_registry(lua)?;
    init_metatable_registry(lua)?;

    // load 'require' function
    bind_require(lua);

    Ok(())
}

fn global_newindex() {

}

fn global_index() {

}

fn init_metatable_registry(lua: &Lua) -> Result<()> {
    let table = lua.create_table()?;
    lua.globals().set("_metatable_registry", table.clone())?;
    set_registry(lua, METATABLE_REGISTRY, table)?;
    Ok(())
}