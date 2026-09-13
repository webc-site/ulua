//! C++ `extern const LuauFastFunction luauF_table[256]` (lbuiltins.h:9).
//! Source: `VM/src/lbuiltins.cpp:2495-2508,2739-2742` (hand-ported fallback)
use crate::{
  functions::luau_f_missing::luau_f_missing, type_aliases::luau_fast_function::LuauFastFunction,
};

#[unsafe(export_name = "ulua_luauF_table")]
pub static LUAU_F_TABLE: [LuauFastFunction; 256] = [Some(luau_f_missing); 256];

pub use LUAU_F_TABLE as luauF_table;
