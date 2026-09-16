//! Source: `Analysis/src/TypePath.cpp:642-740` (hand-ported)
use alloc::string::String;
use core::fmt::Write as _;

use crate::{
  enums::{pack_field::PackField, type_field::TypeField},
  records::path::Path,
  type_aliases::component::Component,
};

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

        match c {
          TypeField::Table => result.push_str("table"),
          TypeField::Metatable => result.push_str("metatable"),
          TypeField::LowerBound => result.push_str("lowerBound"),
          TypeField::UpperBound => result.push_str("upperBound"),
          TypeField::IndexLookup => result.push_str("indexer"),
          TypeField::IndexResult => result.push_str("indexResult"),
          TypeField::Negated => result.push_str("negated"),
          TypeField::Variadic => result.push_str("variadic"),
        }

        result.push_str("()");
      }
      Component::PackField(c) => {
        if !first || prefix_dot {
          result.push('.');
        }

        match c {
          PackField::Arguments => result.push_str("arguments"),
          PackField::Returns => result.push_str("returns"),
          PackField::Tail => result.push_str("tail"),
        }
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
