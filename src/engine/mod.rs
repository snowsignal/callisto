use std::fmt::format;
use std::thread::panicking;

use mlua::Error;
use mlua::Lua;
use sdl2::Sdl;

use crate::Args;
use crate::amulet::load_amulet;

pub struct Engine {
    sdl_inst: Sdl,
    lua_inst: Lua,
}

impl Engine {
    pub fn init() -> Self {
        // init SDL first
        let sdl_inst = sdl2::init().unwrap();
        // load the safe subset of lua libs. this includes:
        // - table
        // - io
        // - os
        // - string
        // - math
        // - package
        // additional modules are loaded based on the lua backend.
        // see https://docs.rs/mlua/latest/mlua/struct.StdLib.html for more
        let lua_inst = mlua::Lua::new();

        // load amulet functions
        load_amulet(&lua_inst).unwrap();

        Self { sdl_inst, lua_inst }
    }

    pub fn execute(&mut self, args: Args) {
        // here we go!

        self.run_game(args.file);
    }

    pub fn run_game(&mut self, main: String) {
        use mlua::Error::*;
        let main_exec = format!("require '{main}'");
        let chunk = self.lua_inst.load(&main_exec);
        match chunk.exec() {
            Ok(()) => {}
            Err(
                SyntaxError { message, .. }
                | RuntimeError(message)
                | MemoryError(message)
                | SafetyError(message),
            ) => {
                println!("{message}");
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
