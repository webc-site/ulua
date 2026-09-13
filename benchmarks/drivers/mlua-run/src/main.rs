use std::{env, fs, process};

fn main() {
    let path = env::args().nth(1).expect("usage: mlua-run <script>");
    let src = fs::read_to_string(&path).expect("read script");
    let lua = mlua::Lua::new();
    if let Err(e) = lua.load(&src).set_name(path).exec() {
        eprintln!("{e}");
        process::exit(1);
    }
}
