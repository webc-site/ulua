use crate::{
  functions::{
    get_bytecode_type_name::{UserdataTypes, get_bytecode_type_name},
    try_find_local_name::try_find_local_name,
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

/// 打印已构建 IR 的类型信息（实参/上值/局部变量名与类型 tag）。
///
/// 全程安全借用：名字查询经 [`IrFunction::proto_view`] 取得原型只读视图，视图缺省
/// （无原型的合成 IR）时按「无名」分支输出，与原 `proto == nullptr` 短路语义一致。
pub(crate) fn log_function_types<B: LogAppend>(
  build: &mut B,
  function: &IrFunction,
  userdata_types: UserdataTypes<'_>,
) {
  let type_info = &function.bc_type_info;
  let proto = function.proto_view();

  for (i, &ty) in type_info.argument_types.iter().enumerate() {
    let r#type = get_bytecode_type_name(ty, userdata_types);
    let optional = optional_flag(ty);

    if ty != LBC_TYPE_ANY {
      if let Some(name_str) = proto.and_then(|proto| try_find_local_name(proto, i as i32, 0)) {
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
      if let Some(name_str) = proto.and_then(|proto| try_find_upvalue_name(proto, i as i32)) {
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

    // 用最后活跃位置作为 PC：类型信息的 'startpc' 在局部变量初始化之前
    // （el.reg/endpc 取自本函数自身字节码类型表，界内成立）
    if let Some(name_str) =
      proto.and_then(|proto| try_find_local_name(proto, el.reg as i32, el.endpc - 1))
    {
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
