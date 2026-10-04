//! Source: `Analysis/src/TypePath.cpp` `struct RenderTypePath` + `renderTypePath`
//! （840-1293，新式类型路径短语渲染器）。
//!
//! 与 [`to_string_human`]（cpp `toStringHuman_DEPRECATED`）互斥：本渲染器仅在
//! fflag `LuauNewTypePathErrorMessages` 开启时经 `explain_reasonings_generic`
//! 调用，输出 [`RenderedTypePath`] 的 `subject`/`prefix` 供子类型原因拼装。
//!
//! 关键语义：`Index` 分量仅 `Variant::Pack` 参与渲染，`Union`/`Intersection`
//! 分量直接跳过（cpp `render(const Index&)` 首行早退），故联合/交集分量子路径
//! 渲染出空 `prefix`，原因落回紧凑的 `baseReason`（"`X` is not a subtype of `Y`"）。
//!
//! 偏差登记：cpp 的 `returnTypePacks` 单复数判定与 `enclosingNegation` 由
//! metadata 版 `traverse` 在遍历期填充；本端口暂未接该 metadata 采集面，故
//! `RenderMetadata` 恒为空 —— 返回包单值与取反（negation）类 flags-on 原因
//! 仍不可达，留待后续与 negation 渲染专项对齐。
use alloc::{format, string::String};
use core::mem::take;

use crate::{
  enums::{pack_field::PackField, type_field::TypeField, variant::Variant},
  functions::to_human_readable_index::to_human_readable_index,
  records::{index::Index, path::Path, property_type_path::Property},
  type_aliases::component::Component,
};

/// cpp `RenderTypePath::Kind`。
///
/// 偏差登记：cpp 另有 `ReturnType`（`ReturnTypes` + metadata 单值返回包时置位）
/// 与从未被任何 cpp 分支写入的 `IndexMember`；二者在空 metadata 端口下不可达，
/// 故省略（对应读取臂随之删除，输出等价）。
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
  None,
  Property,
  Metatable,
  Table,
  PackEntry,
  Parameter,
  ReturnValue,
  ParameterTypes,
  ReturnTypes,
  Tail,
  Variadic,
  PackSlice,
  Reduction,
  MappedPack,
  IndexerResult,
  Other,
}

/// cpp `RenderTypePath::Phrase`。
struct Phrase {
  kind: Kind,
  text: String,
  nested_text: String,
  property_is_read: bool,
  property_prefix: String,
  property_name: String,
  function_owner: String,
  tail_owner: Option<PackField>,
  mapped_pack_entry_owner: String,
  mapped_pack_is_tail: bool,
  plural: bool,
  terminal_override: String,
}

impl Default for Phrase {
  fn default() -> Self {
    Self {
      kind: Kind::None,
      text: String::new(),
      nested_text: String::new(),
      property_is_read: true,
      property_prefix: String::new(),
      property_name: String::new(),
      function_owner: String::new(),
      tail_owner: None,
      mapped_pack_entry_owner: String::new(),
      mapped_pack_is_tail: false,
      plural: false,
      terminal_override: String::new(),
    }
  }
}

/// cpp `RenderedTypePath`。`enclosing_negation` 恒 `None`（metadata 未接）。
pub struct RenderedTypePath {
  pub subject: String,
  pub prefix: String,
}

/// 空 metadata 占位：与 cpp `TypePathRenderMetadata` 默认构造等价。
pub struct RenderMetadata;

/// 单个分量渲染到 `current` 短语态上。
fn render(current: &mut Phrase, component: &Component) {
  match component {
    Component::Property(property) => render_property(current, property),
    Component::Index(index) => render_index(current, index),
    Component::TypeField(field) => render_type_field(current, *field),
    Component::PackField(field) => render_pack_field(current, *field),
    Component::PackSlice(slice) => render_pack_slice(current, slice.start_index),
    Component::Reduction(_) => render_reduction(current),
    Component::GenericPackMapping(_) => render_generic_pack_mapping(current),
  }
}

fn render_property(current: &mut Phrase, property: &Property) {
  let mut next = Phrase {
    kind: Kind::Property,
    property_is_read: property.is_read,
    ..Default::default()
  };

  if current.kind == Kind::Property && current.property_is_read {
    next.property_prefix = take(&mut current.property_prefix);
    next.property_name = format!("{}.{}", current.property_name, property.name);
  } else {
    next.property_name = property.name.clone();
    match current.kind {
      Kind::Metatable | Kind::IndexerResult => {
        next.property_prefix = format!("in {}", current.text)
      }
      Kind::None => {}
      _ => {
        let base = if current.nested_text.is_empty() {
          &current.text
        } else {
          &current.nested_text
        };
        next.property_prefix = format!("of {base}");
      }
    }
  }

  next.text = format!("property `{}`", next.property_name);
  if !next.property_prefix.is_empty() {
    next.text = format!("{} {}", next.text, next.property_prefix);
  }

  next.nested_text = next.text.clone();
  if !next.property_is_read {
    next.nested_text = format!("a value assigned to {}", next.text);
  }

  *current = next;
}

fn render_index(current: &mut Phrase, index: &Index) {
  // cpp：非 Pack 变体（Union / Intersection）跳过，保持 current 不变。
  if index.variant != Variant::Pack {
    return;
  }

  let mut next = Phrase::default();
  let position = to_human_readable_index(index.index);
  let context = if current.nested_text.is_empty() {
    current.text.clone()
  } else {
    current.nested_text.clone()
  };

  match current.kind {
    Kind::ParameterTypes => {
      next.kind = Kind::Parameter;
      next.text = format!("the {position} parameter");
      if !current.function_owner.is_empty() {
        next.text = format!("{} of {}", next.text, current.function_owner);
      }
    }
    Kind::ReturnTypes => {
      // metadata.returnTypePacks 单值判定未接，恒走“第 N 个返回值”。
      next.kind = Kind::ReturnValue;
      next.text = format!("the {position} return value");
      if !current.function_owner.is_empty() {
        next.text = format!("{} of {}", next.text, current.function_owner);
      }
    }
    Kind::MappedPack => {
      next.kind = Kind::PackEntry;
      next.text = format!(
        "the {position} entry of {}",
        current.mapped_pack_entry_owner
      );
    }
    _ => {
      next.kind = Kind::PackEntry;
      next.text = format!("the {position} type pack entry");
      if !context.is_empty() {
        next.text = format!("{} of {context}", next.text);
      }
    }
  }

  next.nested_text = next.text.clone();
  *current = next;
}

fn render_type_field(current: &mut Phrase, field: TypeField) {
  let mut next = Phrase::default();
  let context = if current.nested_text.is_empty() {
    current.text.clone()
  } else {
    current.nested_text.clone()
  };
  let suffix = if context.is_empty() {
    String::new()
  } else {
    format!(" of {context}")
  };

  match field {
    TypeField::Table => {
      next.kind = Kind::Table;
      next.text = format!("the table portion{suffix}");
    }
    TypeField::Metatable => {
      next.kind = Kind::Metatable;
      next.text = format!("the metatable{suffix}");
      if current.kind == Kind::Property {
        let verb = if current.property_is_read {
          " has metatable "
        } else {
          " accepts values with metatable "
        };
        next.terminal_override = format!("{}{verb}", current.text);
      }
    }
    TypeField::LowerBound => {
      next.kind = Kind::Other;
      next.text = format!("the lower bound{suffix}");
    }
    TypeField::UpperBound => {
      next.kind = Kind::Other;
      next.text = format!("the upper bound{suffix}");
    }
    TypeField::IndexLookup => {
      next.kind = Kind::Other;
      next.text = format!("the indexer key type{suffix}");
    }
    TypeField::IndexResult => {
      next.kind = Kind::IndexerResult;
      next.text = format!("the indexer result{suffix}");
    }
    TypeField::Negated => {
      next.kind = Kind::Other;
      let in_suffix = if context.is_empty() {
        String::new()
      } else {
        format!(" in {context}")
      };
      next.text = format!("the negated type{in_suffix}");
    }
    TypeField::Variadic => {
      next.kind = Kind::Variadic;
      next.function_owner = current.function_owner.clone();
      next.text = match (current.kind, current.tail_owner) {
        (Kind::Tail, Some(PackField::Arguments)) => String::from("the variadic parameter"),
        (Kind::Tail, Some(PackField::Returns)) => String::from("the variadic return value"),
        _ => String::from("the variadic tail"),
      };
      if !next.function_owner.is_empty() {
        next.text = format!("{} of {}", next.text, next.function_owner);
      }
    }
  }

  next.nested_text = next.text.clone();
  *current = next;
}

fn render_pack_field(current: &mut Phrase, field: PackField) {
  let mut next = Phrase::default();
  let context = if current.nested_text.is_empty() {
    current.text.clone()
  } else {
    current.nested_text.clone()
  };

  match field {
    PackField::Tail => {
      next.kind = Kind::Tail;
      next.tail_owner = match current.kind {
        Kind::ParameterTypes => Some(PackField::Arguments),
        Kind::ReturnTypes => Some(PackField::Returns),
        _ => None,
      };
      next.function_owner = current.function_owner.clone();
      next.text = match next.tail_owner {
        Some(PackField::Arguments) => {
          let owner_suffix = if next.function_owner.is_empty() {
            String::new()
          } else {
            format!(" of {}", next.function_owner)
          };
          format!("the parameter type pack tail{owner_suffix}")
        }
        Some(PackField::Returns) => {
          let owner_suffix = if next.function_owner.is_empty() {
            String::new()
          } else {
            format!(" of {}", next.function_owner)
          };
          format!("the return type pack tail{owner_suffix}")
        }
        _ => {
          let context_suffix = if context.is_empty() {
            String::new()
          } else {
            format!(" of {context}")
          };
          format!("the type pack's tail{context_suffix}")
        }
      };
    }
    PackField::Arguments => {
      next.kind = Kind::ParameterTypes;
      next.text = String::from("the parameter types");
    }
    PackField::Returns => {
      next.kind = Kind::ReturnTypes;
      next.text = String::from("the return types");
    }
  }

  if field != PackField::Tail {
    next.function_owner = match current.kind {
      Kind::Metatable => String::from("the metatable function"),
      Kind::Property if !current.property_is_read => {
        let mut owner = format!("a function assigned to `{}`", current.property_name);
        if !current.property_prefix.is_empty() {
          owner = format!("{} {}", owner, current.property_prefix);
        }
        owner
      }
      Kind::None => String::new(),
      _ => context.clone(),
    };
    if !next.function_owner.is_empty() {
      next.text = format!("{} of {}", next.text, next.function_owner);
    }
  }

  next.nested_text = next.text.clone();
  *current = next;
}

fn render_pack_slice(current: &mut Phrase, start_index: usize) {
  let mut next = Phrase {
    kind: Kind::PackSlice,
    ..Default::default()
  };
  let position = to_human_readable_index(start_index);

  match current.kind {
    Kind::ParameterTypes => {
      next.plural = true;
      next.text = if current.function_owner == "the metatable function" {
        format!("the metatable function's parameters from the {position} onward")
      } else {
        let mut text = String::from("the parameters");
        if !current.function_owner.is_empty() {
          text = format!("{text} of {}", current.function_owner);
        }
        format!("{text} from the {position} onward")
      };
    }
    Kind::ReturnTypes => {
      next.plural = true;
      next.text = if current.function_owner == "the metatable function" {
        format!("the metatable function's return values from the {position} onward")
      } else {
        let mut text = String::from("the return values");
        if !current.function_owner.is_empty() {
          text = format!("{text} of {}", current.function_owner);
        }
        format!("{text} from the {position} onward")
      };
    }
    Kind::MappedPack => {
      next.text = match (current.tail_owner, current.mapped_pack_is_tail) {
        (Some(PackField::Arguments), _) => String::from("the substituted parameter tail"),
        (Some(PackField::Returns), _) => String::from("the substituted return tail"),
        (_, true) => String::from("the substituted type pack tail"),
        _ => String::from("the substituted type pack"),
      };
      if !current.function_owner.is_empty() {
        next.text = format!("{} for {}", next.text, current.function_owner);
      }
      next.text = format!("{}, from its {position} entry onward,", next.text);
    }
    _ => {
      next.text = format!("the type pack from its {position} entry onward");
    }
  }

  next.nested_text = next.text.clone();
  *current = next;
}

fn render_reduction(current: &mut Phrase) {
  let mut next = Phrase::default();
  next.kind = Kind::Reduction;
  next.text = if current.nested_text.is_empty() {
    String::from("the reduced type")
  } else {
    format!("the reduced form of {}", current.nested_text)
  };
  next.nested_text = next.text.clone();
  *current = next;
}

fn render_generic_pack_mapping(current: &mut Phrase) {
  let mut next = Phrase {
    kind: Kind::MappedPack,
    ..Default::default()
  };

  match current.kind {
    Kind::Tail => {
      next.mapped_pack_is_tail = true;
      next.tail_owner = current.tail_owner;
      next.function_owner = current.function_owner.clone();
      let owner_suffix = if next.function_owner.is_empty() {
        String::new()
      } else {
        format!(" of {}", next.function_owner)
      };
      match current.tail_owner {
        Some(PackField::Arguments) => {
          next.text = format!("the parameter type pack tail{owner_suffix}");
          next.mapped_pack_entry_owner =
            format!("the substituted parameter type pack tail{owner_suffix}");
        }
        Some(PackField::Returns) => {
          next.text = format!("the return type pack tail{owner_suffix}");
          next.mapped_pack_entry_owner =
            format!("the substituted return type pack tail{owner_suffix}");
        }
        _ => {
          next.text = String::from("the type pack tail");
          next.mapped_pack_entry_owner = String::from("the substituted type pack tail");
        }
      }
    }
    _ => {
      next.text = String::from("the generic type pack");
      next.mapped_pack_entry_owner = String::from("the substituted type pack");
    }
  }

  next.nested_text = next.text.clone();
  *current = next;
}

fn finalize(current: &Phrase) -> RenderedTypePath {
  let mut subject = current.text.clone();
  if current.kind == Kind::Property && !current.property_is_read {
    subject = format!("values assigned to {}", current.text);
  }

  if !current.terminal_override.is_empty() {
    return RenderedTypePath {
      subject,
      prefix: current.terminal_override.clone(),
    };
  }

  let prefix = match current.kind {
    Kind::None => String::new(),
    Kind::Property => {
      let verb = if current.property_is_read {
        " has type "
      } else {
        " accepts values of type "
      };
      format!("{}{verb}", current.text)
    }
    Kind::Parameter | Kind::ReturnValue => format!("{} has type ", current.text),
    Kind::ParameterTypes | Kind::ReturnTypes => format!("{} are ", current.text),
    Kind::PackSlice => {
      let verb = if current.plural { " are " } else { " is " };
      format!("{}{verb}", current.text)
    }
    Kind::Variadic => {
      let verb = if current.text == "the variadic tail" {
        " has element type "
      } else {
        " has type "
      };
      format!("{}{verb}", current.text)
    }
    Kind::MappedPack => format!("{} is substituted with ", current.text),
    _ => format!("{} is ", current.text),
  };

  RenderedTypePath { subject, prefix }
}

/// cpp `renderTypePath(path, options)`。
pub fn render_type_path(path: &Path, _metadata: &RenderMetadata) -> RenderedTypePath {
  let mut current = Phrase::default();
  for component in &path.components {
    render(&mut current, component);
  }
  finalize(&current)
}
