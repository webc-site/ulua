//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Function:Luau.UnitTest:tests/main.cpp:316:set_fast_value`
//! Source: `tests/main.cpp`
//! Graph edges:
//! - declared_by: source_file tests/main.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGenCommon.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/main.cpp
//!   - calls <- function setFastFlags (tests/main.cpp)
//! - outgoing:
//!   - type_ref -> record FValue (Common/include/Luau/Common.h)
//!   - translates_to -> rust_item setFastValue

use ulua_common::records::f_value::{FValue, FValueList};

/// C++ `template<typename T> void setFastValue(const std::string& name, T value)`
/// (tests/main.cpp:316): walk the per-type `FValue<T>::list` and set the value of
/// the flag whose `name` matches. Runs once at harness startup before test
/// threads start, matching the C++ contract.
pub fn set_fast_value<T: FValueList + Copy + 'static>(name: &str, value: T) {
  FValue::<T>::set_value_by_name(name, value);
}
