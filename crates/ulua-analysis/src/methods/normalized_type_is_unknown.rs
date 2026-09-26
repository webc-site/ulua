use ulua_common::fflag;

use crate::{
  functions::{
    get_type,
    is_prim::{is_buffer, is_integer, is_number, is_prim, is_thread},
  },
  records::{
    extern_type::ExternType, never_type::NeverType, normalized_type::NormalizedType,
    primitive_type::PrimitiveType, unknown_type::UnknownType,
  },
};

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
