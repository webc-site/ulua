use core::slice::from_raw_parts;

use ulua_common::{
  FFlag::{LuauCompilePropagateTableProps2, LuauIntegerType2},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{enums::type_constant_folding::Type, records::constant::Constant};

pub fn constants_equal(la: &Constant, ra: &Constant) -> bool {
  LUAU_ASSERT!(la.r#type != Type::Unknown && ra.r#type != Type::Unknown);

  match la.r#type {
    Type::Nil => ra.r#type == Type::Nil,
    Type::Boolean => {
      ra.r#type == Type::Boolean && unsafe { la.data.value_boolean == ra.data.value_boolean }
    }
    Type::Number => {
      ra.r#type == Type::Number && unsafe { la.data.value_number == ra.data.value_number }
    }
    Type::Vector => {
      ra.r#type == Type::Vector
        && unsafe { la.data.value_vector[0] == ra.data.value_vector[0] }
        && unsafe { la.data.value_vector[1] == ra.data.value_vector[1] }
        && unsafe { la.data.value_vector[2] == ra.data.value_vector[2] }
        && unsafe { la.data.value_vector[3] == ra.data.value_vector[3] }
    }
    Type::String => {
      ra.r#type == Type::String
        && la.string_length == ra.string_length
        && unsafe {
          from_raw_parts(la.data.value_string as *const u8, la.string_length as usize)
            == from_raw_parts(ra.data.value_string as *const u8, ra.string_length as usize)
        }
    }
    Type::Table => {
      if LuauCompilePropagateTableProps2.get() {
        ra.r#type == Type::Table && unsafe { la.data.value_table == ra.data.value_table }
      } else {
        LUAU_ASSERT!(false);
        false
      }
    }
    Type::Integer => {
      if LuauIntegerType2.get() {
        ra.r#type == Type::Integer && unsafe { la.data.value_integer64 == ra.data.value_integer64 }
      } else {
        LUAU_ASSERT!(false);
        false
      }
    }
    _ => {
      LUAU_ASSERT!(false);
      false
    }
  }
}
