use ulua_vm::records::proto::Proto;

use crate::{
  functions::{proto_views::name_str, try_find_local_name::try_find_local_name},
  traits::LogAppend,
};

/// 打印 IR 函数头（cpp `logFunctionHeader`）：名字、形参表与定义行号。
///
/// 契约：`proto` 指向存活 Proto（编译会话内由 VM 持有）；名字串经 `proto_views` 的安全
/// 视图读取，本函数不再有裸指针解引用。
pub(crate) fn log_function_header<B: LogAppend>(build: &mut B, proto: &Proto) {
  match name_str(proto.debugname.cast_const(), proto) {
    Some(name) => build.log_append(format_args!("; function {name}(")),
    None => build.log_append(format_args!("; function(")),
  }

  let numparams = proto.numparams as i32;
  for i in 0..numparams {
    // 首个参数无分隔符（对齐 cpp `i == 0 ? "" : ", "`）
    let sep = if i == 0 { "" } else { ", " };
    match try_find_local_name(proto, i, 0) {
      Some(name_str) => build.log_append(format_args!("{}{}", sep, name_str)),
      None => build.log_append(format_args!("{}$arg{}", sep, i)),
    }
  }

  if proto.numparams != 0 && proto.is_vararg != 0 {
    build.log_append(format_args!(", ...)"));
  } else {
    build.log_append(format_args!(")"));
  }

  let linedefined = proto.linedefined;
  if linedefined >= 0 {
    build.log_append(format_args!(" line {}\n", linedefined));
  } else {
    build.log_append(format_args!("\n"));
  }
}
