use core::ptr::null_mut;

use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;
pub trait IFragmentAutocompleteReporter {
  fn report_waypoint(&mut self, waypoint: FragmentAutocompleteWaypoint);
  fn report_fragment_string(&mut self, fragment: &str);
}

/// A zero-sized concrete implementor, used only to manufacture a null
/// `*mut dyn IFragmentAutocompleteReporter` (the C++ `nullptr` reporter default).
/// `core::mem::zeroed` cannot be used for a trait object pointer — it is rejected
/// as an invalid zero-initialization — so we build the null fat pointer by casting
/// a thin null pointer of this concrete type.
pub struct NullFragmentAutocompleteReporter;

impl IFragmentAutocompleteReporter for NullFragmentAutocompleteReporter {
  fn report_waypoint(&mut self, _waypoint: FragmentAutocompleteWaypoint) {}
  fn report_fragment_string(&mut self, _fragment: &str) {}
}

/// Returns a null `*mut dyn IFragmentAutocompleteReporter` equivalent to C++ `nullptr`.
pub fn null_reporter() -> *mut dyn IFragmentAutocompleteReporter {
  null_mut::<NullFragmentAutocompleteReporter>() as *mut dyn IFragmentAutocompleteReporter
}
