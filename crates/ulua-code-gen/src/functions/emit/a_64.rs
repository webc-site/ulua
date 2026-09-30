use core::mem::offset_of;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  enums::{condition_a_64::ConditionA64, kind_a_64::KindA64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    label::Label,
    register_a_64::{RegisterA64, reg},
  },
  type_aliases::mem::mem,
};

pub fn emit_abort(build: &mut AssemblyBuilderA64, abort: &mut Label) {
  let mut skip = Label::default();
  build.b_label(&mut skip);
  build.set_label_label(abort);
  build.udf();
  build.set_label_label(&mut skip);
}

const R_STATE: RegisterA64 = reg(KindA64::X, 19);
const R_BASE: RegisterA64 = reg(KindA64::X, 25);

pub fn emit_update_base(build: &mut AssemblyBuilderA64) {
  build.ldr(R_BASE, mem(R_STATE, offset_of!(LuaState, base) as i32));
}

#[inline]
pub fn get_inverse_condition(cond: ConditionA64) -> ConditionA64 {
  match cond {
    ConditionA64::Equal => ConditionA64::Equal,
    ConditionA64::NotEqual => ConditionA64::NotEqual,
    ConditionA64::UnsignedGreater => ConditionA64::UNSIGNED_LESS,
    ConditionA64::UnsignedLessEqual => ConditionA64::UNSIGNED_GREATER_EQUAL,
    ConditionA64::GreaterEqual => ConditionA64::LessEqual,
    ConditionA64::Less => ConditionA64::Greater,
    ConditionA64::Greater => ConditionA64::Less,
    ConditionA64::LessEqual => ConditionA64::GreaterEqual,
    ConditionA64::CarryClear => ConditionA64::UnsignedGreater,
    ConditionA64::CarrySet => ConditionA64::UnsignedLessEqual,
    _ => {
      CODEGEN_ASSERT!(false, "invalid ConditionA64 value for getInverseCondition");
      ConditionA64::Count
    }
  }
}
