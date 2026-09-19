# Ulua

Pure Rust Luau compiler, register VM, and static type checker.

```toml
[dependencies]
ulua = { version = "0.1.0", features = ["default"] }
```

The `ulua` crate is the batteries-included package. It bundles the static type checker (`ulua::check`) and the installable `ulua` CLI (script runner / REPL).

Library-only users can slim the build with `default-features = false` and opt back into `typecheck` and/or `cli` as needed.

### Install the CLI

```sh
cargo install ulua
```

## Links

- Docs: https://docs.rs/ulua
- Repository: https://github.com/webc-site/ulua
