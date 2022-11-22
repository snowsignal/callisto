use mlua::Lua;
use sdl2::Sdl;

pub struct AmuletWindow<'sdl> {
    sdl: &'sdl Sdl,
}

impl<'sdl> AmuletWindow<'sdl> {
    pub fn bind_functions(lua: &Lua) {
        // bind am.window()
    }
}
