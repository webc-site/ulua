//! `normalized_type` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::fflag;

use crate::{
  enums::normalized_part::{ALL_PARTS, NormalizedPart},
  functions::{
    get_singleton_type::get_singleton_type,
    get_type,
    is_prim::{is_buffer, is_integer, is_number, is_prim, is_thread},
  },
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton, extern_type::ExternType,
    never_type::NeverType, normalized_type::NormalizedType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, unknown_type::UnknownType,
  },
};

impl NormalizedType {
  fn has_booleans(&self) -> bool {
    get_type::get::<NeverType>(self.booleans).is_none()
  }
}

impl NormalizedType {
  fn has_buffers(&self) -> bool {
    get_type::get::<NeverType>(self.buffers).is_none()
  }
}

impl NormalizedType {
  pub fn has_errors(&self) -> bool {
    get_type::get::<NeverType>(self.errors).is_none()
  }
}

impl NormalizedType {
  pub fn has_extern_types(&self) -> bool {
    !self.extern_types.is_never()
  }
}

impl NormalizedType {
  pub fn has_functions(&self) -> bool {
    !self.functions.is_never()
  }
}

impl NormalizedType {
  fn has_integers(&self) -> bool {
    if fflag::LuauIntegerType2.get() {
      get_type::get::<NeverType>(self.integers).is_none()
    } else {
      false
    }
  }
}

impl NormalizedType {
  fn has_nils(&self) -> bool {
    get_type::get::<NeverType>(self.nils).is_none()
  }
}

impl NormalizedType {
  fn has_numbers(&self) -> bool {
    get_type::get::<NeverType>(self.numbers).is_none()
  }
}

impl NormalizedType {
  /// 单个部件的非空判定：把 [`NormalizedPart`] 派发到对应的 `has_x()` 访问器，
  /// 使名单化判定（[`NormalizedType::has_parts_other_than`]）与逐条书写等价。
  fn has_part(&self, part: NormalizedPart) -> bool {
    match part {
      NormalizedPart::Tops => self.has_tops(),
      NormalizedPart::Booleans => self.has_booleans(),
      NormalizedPart::ExternTypes => self.has_extern_types(),
      NormalizedPart::Errors => self.has_errors(),
      NormalizedPart::Nils => self.has_nils(),
      NormalizedPart::Numbers => self.has_numbers(),
      NormalizedPart::Integers => self.has_integers(),
      NormalizedPart::Strings => self.has_strings(),
      NormalizedPart::Threads => self.has_threads(),
      NormalizedPart::Buffers => self.has_buffers(),
      NormalizedPart::Tables => self.has_tables(),
      NormalizedPart::Functions => self.has_functions(),
      NormalizedPart::Tyvars => self.has_tyvars(),
    }
  }
}

impl NormalizedType {
  /// cpp 各 `isX()` 谓词与多个 `types.*` 归约入口的共同形状：`allowed` 名单之外
  /// 的部件全部为空即为 `true`。名单内的正部件（如 `is_nil` 的 nils）由调用方
  /// 另行正判，与原判定链的短路顺序一致。
  pub fn has_parts_other_than(&self, allowed: &[NormalizedPart]) -> bool {
    ALL_PARTS
      .iter()
      .any(|part| !allowed.contains(part) && self.has_part(*part))
  }
}

impl NormalizedType {
  fn has_strings(&self) -> bool {
    !self.strings.is_never()
  }
}

impl NormalizedType {
  pub fn has_tables(&self) -> bool {
    !self.tables.is_never()
  }
}

impl NormalizedType {
  fn has_threads(&self) -> bool {
    get_type::get::<NeverType>(self.threads).is_none()
  }
}

impl NormalizedType {
  pub fn has_top_table(&self) -> bool {
    if !self.has_tables() {
      return false;
    }

    for &ty in &self.tables.order {
      if let Some(prim_ref) = get_type::get::<PrimitiveType>(ty)
        && prim_ref.r#type == PrimitiveType::TABLE
      {
        return true;
      }
    }

    false
  }
}

impl NormalizedType {
  pub fn has_tops(&self) -> bool {
    get_type::get::<NeverType>(self.tops).is_none()
  }
}

impl NormalizedType {
  fn has_tyvars(&self) -> bool {
    !self.tyvars.is_empty()
  }
}

impl NormalizedType {
  pub fn is_exactly_number(&self) -> bool {
    // `has_integers()` 自身按 `LuauIntegerType2` 门控（关旗标时恒 `false`），故原先的
    // 旗标双分支——差别仅是尾部多一条 `!hasIntegers()`——坍缩为单条名单化判定。
    self.has_numbers() && !self.has_parts_other_than(&[NormalizedPart::Numbers])
  }
}

impl NormalizedType {
  pub fn is_falsy(&self) -> bool {
    let mut has_a_false = false;
    if let Some(singleton_ptr) = get_type::get::<SingletonType>(self.booleans)
      && let Some(boolean_ptr) = get_singleton_type::<BooleanSingleton>(singleton_ptr)
    {
      has_a_false = !boolean_ptr.value;
    }

    // 允许名单含 booleans：bool 部件只允许存 `false` 单例（上面已判），原判定链
    // 因此不否定 `hasBooleans()`；nils 是正部件，integers 自带旗标门控。
    (has_a_false || self.has_nils())
      && !self.has_parts_other_than(&[NormalizedPart::Nils, NormalizedPart::Booleans])
  }
}

impl NormalizedType {
  pub fn is_nil(&self) -> bool {
    // 原判定链排除名单里没有 errors（含 error 的 nil 联合仍判为 nil）与 nils 自身，
    // 故二者进允许名单；`has_integers()` 自带旗标门控，尾部多一条即可。
    self.has_nils() && !self.has_parts_other_than(&[NormalizedPart::Nils, NormalizedPart::Errors])
  }
}

impl NormalizedType {
  pub fn is_subtype_of_string(&self) -> bool {
    // 同 `is_exactly_number`：`has_integers()` 自带 `LuauIntegerType2` 门控，
    // 原旗标双分支只差尾部一条 `!hasIntegers()`，此处坍缩为单条名单化判定。
    self.has_strings() && !self.has_parts_other_than(&[NormalizedPart::Strings])
  }
}

impl NormalizedType {
  pub fn is_truthy(&self) -> bool {
    !self.is_falsy()
  }
}

impl NormalizedType {
  pub fn is_unknown(&self) -> bool {
    // Check if tops is UnknownType
    let tops_ptr = get_type::get::<UnknownType>(self.tops);
    if tops_ptr.is_some() {
      return true;
    }

    // Check if we have all primitives
    let has_all_primitives = if fflag::LuauIntegerType2.get() {
      is_prim(self.booleans, PrimitiveType::BOOLEAN)
        && is_prim(self.nils, PrimitiveType::NIL_TYPE)
        && is_number(self.numbers)
        && self.strings.is_string()
        && is_thread(self.threads)
        && is_buffer(self.buffers)
        && is_integer(self.integers)
    } else {
      is_prim(self.booleans, PrimitiveType::BOOLEAN)
        && is_prim(self.nils, PrimitiveType::NIL_TYPE)
        && is_number(self.numbers)
        && self.strings.is_string()
        && is_thread(self.threads)
        && is_buffer(self.buffers)
    };

    // Check extern types: we need at least one ExternType that matches builtinTypes->extern_type with empty disjunction
    let mut is_top_extern_type = false;
    for (t, disj) in &self.extern_types.extern_types {
      let extern_type_ptr = get_type::get::<ExternType>(*t);
      if extern_type_ptr.is_some() {
        let builtin_extern_type = self.builtin_types.get_mut().extern_type;
        if *t == builtin_extern_type && disj.empty() {
          is_top_extern_type = true;
          break;
        }
      }
    }

    // Check tables: we need at least one PrimitiveType::TABLE
    let mut is_top_table = false;
    for &t in &self.tables.order {
      if is_prim(t, PrimitiveType::TABLE) {
        is_top_table = true;
        break;
      }
    }

    // any = unknown or error ==> we need to make sure we have all the unknown components, but not errors
    let errors_ptr = get_type::get::<NeverType>(self.errors);
    errors_ptr.is_some()
      && has_all_primitives
      && is_top_extern_type
      && is_top_table
      && self.functions.is_top
  }
}

impl NormalizedType {
  pub fn should_suppress_errors(&self) -> bool {
    self.has_errors() || get_type::get::<AnyType>(self.tops).is_some()
  }
}
