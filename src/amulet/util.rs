const AMULET_LUA_MODULE_NAME: &str = "am";
use mlua::*;

pub fn bind_function<'lua, A, R, F>(lua: &'lua Lua, name: &str, function: F) -> Result<()>
where
    A: FromLuaMulti<'lua>,
    R: ToLuaMulti<'lua>,
    F: 'static + Fn(&'lua Lua, A) -> Result<R>,
{
    let am_module = get_am_module(lua)?;

    let function = lua.create_function(function)?;

    am_module.set(name, function)?;

    Ok(())
}

fn get_am_module(lua: &Lua) -> Result<Table> {
    let maybe_am_module = lua.globals().get(AMULET_LUA_MODULE_NAME)?;
    if let Value::Table(table) = maybe_am_module {
        Ok(table)
    } else {
        init_am_module(lua)
    }
}

fn init_am_module(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;
    lua.globals().set(AMULET_LUA_MODULE_NAME, table)?;
    lua.globals().get(AMULET_LUA_MODULE_NAME)
}