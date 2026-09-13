use core::ffi::c_char;

use ulua_vm::records::proto::Proto;

use crate::{
  functions::{
    get_bytecode_type_name::get_bytecode_type_name, try_find_local_name::try_find_local_name,
    try_find_upvalue_name::try_find_upvalue_name,
  },
  records::{bytecode_reg_type_info::LBC_TYPE_ANY, ir_function::IrFunction},
  traits::LogAppend,
};

pub const LBC_TYPE_OPTIONAL_BIT: u8 = 1 << 7;

/// 类型可空标志：`LBC_TYPE_OPTIONAL_BIT` 置位时输出 `?`
fn optional_flag(ty: u8) -> &'static str {
  if ty & LBC_TYPE_OPTIONAL_BIT != 0 {
    "?"
  } else {
    ""
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn log_function_types(
  build: &mut dyn LogAppend,
  function: &IrFunction,
  userdata_types: *const *const c_char,
) {
  unsafe {
    let type_info = &function.bc_type_info;

    for (i, &ty) in type_info.argument_types.iter().enumerate() {
      let r#type = get_bytecode_type_name(ty, userdata_types);
      let optional = optional_flag(ty);

      if ty != LBC_TYPE_ANY {
        if let Some(name_str) = try_find_local_name(function.proto, i as i32, 0) {
          build.log_append(format_args!(
            "; R{}: {}{} [argument '{}']\n",
            i, r#type, optional, name_str
          ));
        } else {
          build.log_append(format_args!(
            "; R{}: {}{} [argument]\n",
            i, r#type, optional
          ));
        }
      }
    }

    for (i, &ty) in type_info.upvalue_types.iter().enumerate() {
      let r#type = get_bytecode_type_name(ty, userdata_types);
      let optional = optional_flag(ty);

      if ty != LBC_TYPE_ANY {
        if let Some(name_str) = try_find_upvalue_name(function.proto as *const Proto, i as i32) {
          build.log_append(format_args!(
            "; U{}: {}{} ['{}']\n",
            i, r#type, optional, name_str
          ));
        } else {
          build.log_append(format_args!("; U{}: {}{}\n", i, r#type, optional));
        }
      }
    }

    for el in type_info.reg_types.iter() {
      let r#type = get_bytecode_type_name(el.r#type, userdata_types);
      let optional = optional_flag(el.r#type);

      // Using last active position as the PC because 'startpc' for type info is before local is initialized
      if let Some(name_str) = try_find_local_name(function.proto, el.reg as i32, el.endpc - 1) {
        build.log_append(format_args!(
          "; R{}: {}{} from {} to {} [local '{}']\n",
          el.reg, r#type, optional, el.startpc, el.endpc, name_str
        ));
      } else {
        build.log_append(format_args!(
          "; R{}: {}{} from {} to {}\n",
          el.reg, r#type, optional, el.startpc, el.endpc
        ));
      }
    }
  }
}
