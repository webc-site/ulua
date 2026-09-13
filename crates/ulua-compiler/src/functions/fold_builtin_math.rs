use core::f64::consts::{E, GOLDEN_RATIO, PI, SQRT_2, TAU};

use ulua_ast::records::ast_name::AstName;

use crate::{
  functions::{cnum::cnum, cvar::cvar},
  records::constant::Constant,
};

pub(crate) const K_PI: f64 = PI;
pub(crate) const K_NAN: f64 = f64::NAN;
pub(crate) const K_E: f64 = E;
pub(crate) const K_PHI: f64 = GOLDEN_RATIO;
pub(crate) const K_SQRT2: f64 = SQRT_2;
pub(crate) const K_TAU: f64 = TAU;

pub(crate) fn fold_builtin_math(index: AstName) -> Constant {
  match index.as_bytes() {
    b"pi" => cnum(K_PI),
    b"huge" => cnum(f64::INFINITY),
    b"nan" => cnum(K_NAN),
    b"e" => cnum(K_E),
    b"phi" => cnum(K_PHI),
    b"sqrt2" => cnum(K_SQRT2),
    b"tau" => cnum(K_TAU),
    _ => cvar(),
  }
}
