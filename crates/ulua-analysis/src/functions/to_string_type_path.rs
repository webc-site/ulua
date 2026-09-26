//! Source: `Analysis/src/TypePath.cpp:642-740` (hand-ported)
use alloc::string::String;
use core::fmt::Write as _;

use crate::{records::path::Path, type_aliases::component::Component};

pub fn to_string(path: &Path, prefix_dot: bool) -> String {
  let mut result = String::new();
  let mut first = true;

  for component in &path.components {
    match component {
      Component::Property(c) => {
        result.push('[');
        if c.is_read {
          result.push_str("read ");
        } else {
          result.push_str("write ");
        }
        result.push('"');
        result.push_str(&c.name);
        result.push('"');
        result.push(']');
      }
      Component::Index(c) => {
        let _ = write!(result, "[{}]", c.index);
      }
      Component::TypeField(c) => {
        if !first || prefix_dot {
          result.push('.');
        }

        result.push_str(c.into());
        result.push_str("()");
      }
      Component::PackField(c) => {
        if !first || prefix_dot {
          result.push('.');
        }

        result.push_str(c.into());
        result.push_str("()");
      }
      Component::PackSlice(c) => {
        let _ = write!(result, "[{}:]", c.start_index);
      }
      Component::Reduction(_c) => {
        // We need to rework the TypePath system to make subtyping failures
        // easier to understand
        // https://roblox.atlassian.net/browse/CLI-104422
        result.push_str("~~>");
      }
      Component::GenericPackMapping(_c) => {
        result.push('~');
      }
    }

    first = false;
  }

  result
}
