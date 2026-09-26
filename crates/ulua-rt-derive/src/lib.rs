//! Procedural derive macros for [`ulua-rt`](https://docs.rs/ulua-rt) — the
//! safe, mlua-style API of ulua (pure-Rust Luau).
//!
//! Two derives are provided, mirroring `mlua_derive`:
//!
//! - [`macro@UserData`] — `#[derive(UserData)]` generates an `impl UserData`
//!   that exposes the type's named struct fields to Lua (getters/setters),
//!   honoring the `#[lua(skip)]` / `#[lua(get)]` / `#[lua(set)]` /
//!   `#[lua(name = "...")]` field attributes — exactly mlua's field surface.
//! - [`macro@FromLua`] — `#[derive(FromLua)]` generates an `impl FromLua` that
//!   recovers a `T: Clone` value out of a Lua userdata of that type.
//!
//! These are intended to be used via ulua-rt's `macros` feature, which
//! re-exports them so users write `#[derive(ulua_rt::UserData)]` /
//! `#[derive(ulua_rt::FromLua)]`.
//!
//! ## Not implemented (deferred)
//!
//! mlua additionally ships a `#[userdata_impl]` **attribute** macro that turns
//! a whole `impl` block into method/meta/field registrations through an
//! `inventory`-based registry (`UserDataRegistry`) and `Lua::create_proxy`.
//! ulua-rt does not (yet) have that registry / proxy / inventory machinery, so
//! the method side of mlua's derive system is out of scope here; only the
//! field-deriving `#[derive(UserData)]` and `#[derive(FromLua)]` are provided.
//! mlua's `chunk!` and `lua_module` macros are likewise out of scope.

use proc_macro::TokenStream;

mod attr;
mod from_lua;
mod userdata;

/// Derive macro implementing `UserData` for a struct, exposing its named fields
/// to Lua.
///
/// Field attributes (mirroring mlua):
/// - `#[lua(skip)]` — do not expose this field.
/// - `#[lua(get)]` — expose only a getter (read-only field).
/// - `#[lua(set)]` — expose only a setter (write-only field).
/// - `#[lua(name = "...")]` — use a custom Lua-visible name.
/// - bare `#[lua]` or no attribute — getter **and** setter (the default).
///
/// Getters clone the field, so the field type must be `Clone + IntoLua`;
/// setters move the assigned value in, so it must be `FromLua`.
#[proc_macro_derive(UserData, attributes(lua))]
pub fn userdata(item: TokenStream) -> TokenStream {
  userdata::userdata_type(item)
}

/// Derive macro implementing `FromLua` for a `Clone` type, recovering the value
/// out of a Lua userdata of that type.
#[proc_macro_derive(FromLua)]
pub fn from_lua(input: TokenStream) -> TokenStream {
  from_lua::from_lua(input)
}
