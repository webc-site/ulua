//! Source: `Analysis/include/Luau/JsonEmitter.h` (lines 155-167, hand-ported)
//!
//! C++ template:
//! ```cpp
//! template<typename T>
//! void writePair(std::string_view name, T value)
//! {
//!     if (finished) return;
//!     emitter->writeComma();
//!     write(*emitter, name);
//!     emitter->writeRaw(':');
//!     write(*emitter, value);
//! }
//! ```
//!
//! C++ resolves the two `write(*emitter, ...)` calls by overload resolution over
//! the `write(JsonEmitter&, T)` family. Rust expresses that overload set as the
//! `WriteJson` trait defined here: every value that can be a JSON value impls it
//! by delegating to the corresponding `write_json_emitter_*` free function (the
//! ported C++ `write` overloads). The concrete `impl WriteJson for <T>` blocks
//! are colocated with the function that ports the matching overload:
//!   - scalars / strings / pointers          -> this file
//!   - `Vec<T>`                               -> functions/write_json_emitter.rs
//!   - `Option<T>`                            -> functions/write_json_emitter_alt_b.rs
//!   - `unordered_map<String, T>` / hash maps -> functions/write_json_emitter_alt_c.rs

use ulua_ast::records::location::Location;

use crate::{
  functions::{
    write_dcr_logger_alt_f::write_json_emitter_location,
    write_dcr_logger_alt_g::write_json_emitter_error_snapshot,
    write_dcr_logger_alt_h::write_json_emitter_binding_snapshot,
    write_dcr_logger_alt_i::write_json_emitter_type_binding_snapshot,
    write_dcr_logger_alt_k::write_json_emitter_expr_types_at_location,
    write_dcr_logger_alt_l::write_json_emitter_annotation_types_at_location,
    write_dcr_logger_alt_m::write_json_emitter_constraint_generation_log,
    write_dcr_logger_alt_n::write_json_emitter_scope_snapshot,
    write_dcr_logger_alt_o::write_json_emitter_constraint_block,
    write_dcr_logger_alt_p::write_json_emitter_constraint_snapshot,
    write_dcr_logger_alt_q::write_json_emitter_boundary_snapshot,
    write_dcr_logger_alt_t::write_json_emitter_step_snapshot,
    write_dcr_logger_alt_u::write_json_emitter_type_solve_log,
    write_dcr_logger_alt_v::write_json_emitter_type_check_log,
  },
  records::{
    annotation_types_at_location::AnnotationTypesAtLocation, binding_snapshot::BindingSnapshot,
    boundary_snapshot::BoundarySnapshot, constraint_block::ConstraintBlock,
    constraint_generation_log::ConstraintGenerationLog, constraint_snapshot::ConstraintSnapshot,
    error_snapshot::ErrorSnapshot, expr_types_at_location::ExprTypesAtLocation,
    scope_snapshot::ScopeSnapshot, type_binding_snapshot::TypeBindingSnapshot,
    type_check_log::TypeCheckLog, type_solve_log::TypeSolveLog,
  },
  type_aliases::step_snapshot::StepSnapshot,
};
extern crate alloc;

use alloc::string::{String, ToString};

use crate::{
  functions::{
    write_dcr_logger::write_json_emitter_t,
    write_json_emitter_alt_ae::write_json_emitter_string_view,
    write_json_emitter_alt_w::write_json_emitter_bool,
    write_json_emitter_alt_x::write_json_emitter_f64,
  },
  records::{json_emitter::JsonEmitter, object_emitter::ObjectEmitter},
};

/// Rust shape of the overloaded `void write(JsonEmitter&, T)` family. A type
/// implements this iff there is a `write(JsonEmitter&, T)` overload for it in
/// C++. The single method writes the JSON encoding of `self` into `emitter`.
pub trait WriteJson {
  fn write_json(&self, emitter: &mut JsonEmitter);
}

/// `write(*emitter, value)` where `value` was bound to a reference at the call
/// site (`writePair("k", &field)`); forward through the reference.
impl<T: WriteJson + ?Sized> WriteJson for &T {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    (**self).write_json(emitter);
  }
}

// --- scalar overloads (write(JsonEmitter&, bool/double/int...)) ---

impl WriteJson for bool {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_bool(emitter, *self);
  }
}

impl WriteJson for f64 {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_f64(emitter, *self);
  }
}

macro_rules! write_json_int {
    ($($t:ty),*) => {$(
        impl WriteJson for $t {
            fn write_json(&self, emitter: &mut JsonEmitter) {
                emitter.write_raw_string_view(&self.to_string());
            }
        }
    )*};
}
// NB: no i8/u8 — C++ `char` writes as a one-char STRING, not a JSON integer.
write_json_int!(i32, u32, i64, u64, usize, isize, i16, u16);

// --- string overloads (write(JsonEmitter&, std::string_view / std::string)) ---

impl WriteJson for str {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_string_view(emitter, self);
  }
}

impl WriteJson for String {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_string_view(emitter, self.as_str());
  }
}

// --- pointer overload (write(JsonEmitter&, const T*) via to_pointer_id) ---
// In DcrLogger, raw pointers are written through `write_json_emitter_t`, which
// emits the pointer id. Matches `writePair("currentConstraint", snapshot.ptr)`.

impl<T> WriteJson for *const T {
  fn write_json(&self, emitter: &mut JsonEmitter) {
    write_json_emitter_t(emitter, *self);
  }
}

impl ObjectEmitter {
  /// `writePair(std::string_view name, T value)`
  pub fn write_pair<T: WriteJson>(&mut self, name: &str, value: T) {
    if self.finished {
      return;
    }

    let emitter = unsafe { &mut *self.emitter };
    emitter.write_comma();
    // write(*emitter, name) — `name` is a std::string_view => the string overload.
    write_json_emitter_string_view(emitter, name);
    emitter.write_raw_string_view(":");
    // write(*emitter, value)
    value.write_json(emitter);
  }
}

// --- record overloads (the DcrLogger `write(JsonEmitter&, const X&)` family) ---
// Each delegates to its ported free function; these are the `write` overloads
// that let snapshot/log records be JSON values via writePair/writeValue.
macro_rules! write_json_via {
  ($ty:path => $func:path) => {
    impl WriteJson for $ty {
      fn write_json(&self, emitter: &mut JsonEmitter) {
        $func(emitter, self);
      }
    }
  };
}

write_json_via!(Location
    => write_json_emitter_location);
write_json_via!(ErrorSnapshot
    => write_json_emitter_error_snapshot);
write_json_via!(BindingSnapshot
    => write_json_emitter_binding_snapshot);
write_json_via!(TypeBindingSnapshot
    => write_json_emitter_type_binding_snapshot);
write_json_via!(ExprTypesAtLocation
    => write_json_emitter_expr_types_at_location);
write_json_via!(AnnotationTypesAtLocation
    => write_json_emitter_annotation_types_at_location);
write_json_via!(ConstraintGenerationLog
    => write_json_emitter_constraint_generation_log);
write_json_via!(ScopeSnapshot
    => write_json_emitter_scope_snapshot);
write_json_via!(ConstraintBlock
    => write_json_emitter_constraint_block);
write_json_via!(ConstraintSnapshot
    => write_json_emitter_constraint_snapshot);
write_json_via!(BoundarySnapshot
    => write_json_emitter_boundary_snapshot);
write_json_via!(StepSnapshot
    => write_json_emitter_step_snapshot);
write_json_via!(TypeSolveLog
    => write_json_emitter_type_solve_log);
write_json_via!(TypeCheckLog
    => write_json_emitter_type_check_log);
