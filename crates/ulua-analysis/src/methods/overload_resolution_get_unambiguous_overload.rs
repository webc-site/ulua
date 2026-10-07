use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  overload_resolution::OverloadResolution, selected_overload::SelectedOverload,
};

impl OverloadResolution {
  pub fn get_unambiguous_overload(&self) -> SelectedOverload {
    if self.ok.len() == 1 && self.potential_overloads.is_empty() {
      return SelectedOverload {
        overload: Some(self.ok[0]),
        assumed_constraints: vec![],
        should_retry: false,
      };
    }

    if self.ok.is_empty() && self.potential_overloads.len() == 1 {
      return SelectedOverload {
        overload: Some(self.potential_overloads[0].0),
        assumed_constraints: self.potential_overloads[0].1.clone(),
        should_retry: false,
      };
    }

    if self.ok.len() > 1 {
      return SelectedOverload {
        overload: None,
        assumed_constraints: vec![],
        should_retry: false,
      };
    }

    if self.potential_overloads.len() + self.ok.len() > 1 {
      // This is a first case of "ambiguous" overloads: we have at least
      // one overload that matches without constraints, and one that matches
      // with extra constraints.
      if self.ok.is_empty() {
        return SelectedOverload {
          overload: Some(self.potential_overloads[0].0),
          assumed_constraints: self.potential_overloads[0].1.clone(),
          should_retry: true,
        };
      } else {
        LUAU_ASSERT!(self.ok.len() == 1);
        return SelectedOverload {
          overload: Some(self.ok[0]),
          assumed_constraints: vec![],
          should_retry: true,
        };
      }
    }

    LUAU_ASSERT!(self.potential_overloads.len() + self.ok.len() == 0);

    // In this case, no overloads are valid. Let's try to pick the one that
    // will cause us to report the most legible errors.
    if self.incompatible_overloads.len() == 1 {
      // There's exactly one incompatible overload, but it has
      // the right arity, so just use that. We'll fail type checking
      // but that's ok.
      return SelectedOverload {
        overload: Some(self.incompatible_overloads[0].0),
        assumed_constraints: vec![],
        should_retry: false,
      };
    }

    SelectedOverload {
      overload: None,
      assumed_constraints: vec![],
      should_retry: false,
    }
  }
}
