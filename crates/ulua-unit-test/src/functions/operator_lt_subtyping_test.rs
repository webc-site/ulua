use alloc::string::String;
use core::fmt::Write;

use ulua_analysis::enums::subtyping_variance::SubtypingVariance;

pub fn operator_lt_ostream_subtyping_variance(
  lhs: &mut String,
  variance: SubtypingVariance,
) -> &mut String {
  let s = match variance {
    SubtypingVariance::Covariant => "covariant",
    SubtypingVariance::Contravariant => "contravariant",
    SubtypingVariance::Invariant => "invariant",
    SubtypingVariance::Invalid => "*invalid*",
  };

  let _ = write!(lhs, "{}", s);
  lhs
}
