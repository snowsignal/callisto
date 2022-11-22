use mlua::*;

pub const METATABLE_REGISTRY: &str = "reg_metatable";
pub const LOADED_MOD_REGISTRY: &str = "_LOADED";

pub fn reserve_registry(lua: &Lua) -> Result<()> {
    set_registry(lua, METATABLE_REGISTRY, true)?;
    Ok(())
}

pub fn set_registry<'lua, T>(lua: &'lua Lua, key: &str, value: T) -> Result<()> where T: ToLua<'lua>  {
    lua.set_named_registry_value(key, value)?;
    Ok(())
}

pub fn get_registry<'lua, T>(lua: &'lua Lua, key: &str) -> Result<T> where T: FromLua<'lua> {
    lua.named_registry_value(key)
}