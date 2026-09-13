use std::{env, fs, process};
use tsuki::builtin::{BaseLib, CoroLib, MathLib, StrLib, TableLib, Utf8Lib};
use tsuki::Lua;

fn main() {
    let path = env::args().nth(1).expect("usage: tsuki-run <script>");
    let src = fs::read(&path).expect("read script");

    let lua = Lua::new(());
    lua.use_module(None, true, BaseLib).unwrap();
    lua.use_module(None, true, CoroLib).unwrap();
    lua.use_module(None, true, MathLib).unwrap();
    lua.use_module(None, true, StrLib).unwrap();
    lua.use_module(None, true, TableLib).unwrap();
    lua.use_module(None, true, Utf8Lib).unwrap();

    let chunk = match lua.load(path.clone(), src) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("parse error: {e}");
            process::exit(1);
        }
    };
    let td = lua.create_thread();
    if let Err(e) = td.call::<()>(chunk, ()) {
        eprintln!("runtime error: {e}");
        process::exit(1);
    }
}
