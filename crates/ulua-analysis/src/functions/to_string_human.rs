//! Source: `Analysis/src/TypePath.cpp:742-967` (hand-ported)
use alloc::string::String;
use core::fmt::Write as _;

use crate::{
  enums::{pack_field::PackField, type_field::TypeField, variant::Variant},
  functions::to_human_readable_index::to_human_readable_index,
  records::path::Path,
  type_aliases::component::Component,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
  Initial,
  Normal,
  Property,
  PendingIs,
  PendingAs,
  PendingWhich,
}

pub fn to_string_human(path: &Path) -> String {
  let mut result = String::new();
  let mut state = State::Initial;
  let mut last = false;

  let mut count: usize = 0;

  for component in &path.components {
    count += 1;
    if count == path.components.len() {
      last = true;
    }

    match component {
      Component::Property(c) => {
        if state == State::PendingIs {
          result.push_str(", ");
        }

        match state {
          State::Initial | State::PendingIs => {
            if c.is_read {
              result.push_str("accessing `");
            } else {
              result.push_str("writing to `");
            }
          }
          State::Property => {
            // if the previous state was a property, then we're doing
            // a sequence of indexing
            result.push('.');
          }
          _ => {}
        }

        result.push_str(&c.name);

        state = State::Property;
      }
      Component::Index(c) => {
        if state == State::Initial && !last {
          result.push_str("in ");
        } else if state == State::PendingIs {
          result.push_str(" has ");
        } else if state == State::Property {
          result.push_str("` has ");
        }

        let _ = write!(result, "the {}", to_human_readable_index(c.index));

        match c.variant {
          Variant::Pack => result.push_str(" entry in the type pack"),
          Variant::Union => result.push_str(" component of the union"),
          Variant::Intersection => result.push_str(" component of the intersection"),
        }

        if state == State::PendingWhich {
          result.push_str(" which");
        }

        if state == State::PendingIs || state == State::Property {
          state = State::PendingAs;
        } else {
          state = State::PendingIs;
        }
      }
      Component::TypeField(c) => {
        if state == State::Initial && !last {
          result.push_str("in ");
        } else if state == State::PendingIs {
          result.push_str(", ");
        } else if state == State::Property {
          result.push_str("` has ");
        }

        match c {
          TypeField::Table => {
            result.push_str("the table portion");
            if state == State::Property {
              state = State::PendingAs;
            } else {
              state = State::PendingIs;
            }
          }
          TypeField::Metatable => {
            result.push_str("the metatable portion");
            if state == State::Property {
              state = State::PendingAs;
            } else {
              state = State::PendingIs;
            }
          }
          TypeField::LowerBound => {
            result.push_str("the lower bound of ");
            state = State::Normal;
          }
          TypeField::UpperBound => {
            result.push_str("the upper bound of ");
            state = State::Normal;
          }
          TypeField::IndexLookup => {
            result.push_str("the index type");
            if state == State::Property {
              state = State::PendingAs;
            } else {
              state = State::PendingIs;
            }
          }
          TypeField::IndexResult => {
            result.push_str("the result of indexing");
            if state == State::Property {
              state = State::PendingAs;
            } else {
              state = State::PendingIs;
            }
          }
          TypeField::Negated => {
            result.push_str("the negation ");
            state = State::Normal;
          }
          TypeField::Variadic => {
            result.push_str("the variadic ");
            state = State::Normal;
          }
        }
      }
      Component::PackField(c) => {
        if state == State::PendingIs {
          result.push_str(", ");
        } else if state == State::Property {
          result.push_str("`, ");
        }

        match c {
          PackField::Arguments => {
            if state == State::Initial {
              result.push_str("it ");
            } else if state == State::PendingIs {
              result.push_str("the function ");
            }
            result.push_str("takes");
          }
          PackField::Returns => {
            if state == State::Initial {
              result.push_str("it ");
            } else if state == State::PendingIs {
              result.push_str("the function ");
            }
            result.push_str("returns");
          }
          PackField::Tail => {
            if state == State::Initial {
              result.push_str("it has ");
            }
            result.push_str("a tail of");
          }
        }

        if state == State::PendingIs {
          result.push(' ');
          state = State::PendingWhich;
        } else {
          result.push(' ');
          state = State::Normal;
        }
      }
      Component::PackSlice(c) => {
        let _ = write!(
          result,
          "the portion of the type pack starting at index {} to the end",
          c.start_index
        );
      }
      Component::Reduction(_c) => {
        if state == State::Initial {
          result.push_str("it ");
        }
        result.push_str("reduces to ");
        state = State::Normal;
      }
      Component::GenericPackMapping(_c) => {
        result.push_str("is a generic pack mapped to ");
      }
    }
  }

  match state {
    State::Property => {
      result.push_str("` results in ");
    }
    State::PendingWhich => {
      // pending `which` becomes `is` if it's at the end
      result.push_str("is ");
    }
    State::PendingIs => {
      result.push_str(" is ");
    }
    State::PendingAs => {
      result.push_str(" as ");
    }
    _ => {}
  }

  result
}
