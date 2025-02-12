//--> Imports & Modules <--

mod libs;

use std;

use clap::{Command, Arg};

use tokio;

use mlua::{Lua, Compiler, StdLib, LuaOptions};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Functions & Impls <--

fn main() {
    // DO CLAP THINGS
    let matches = Command::new("rere")
        .get_matches();

    // DO MLUA THINGS
    let mut luau = Lua::new_with(StdLib::NONE, LuaOptions::default()).unwrap();
    luau.set_compiler(Compiler::new()
        .set_optimization_level(1)
        .set_coverage_level(2));

    // DO TOKIO THINGS
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {})
}
