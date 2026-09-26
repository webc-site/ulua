use ulua_vm::records::proto::Proto;

use crate::functions::proto_views::{locvars, name_str};

/// cpp `tryFindLocalName`：按 (寄存器号, pc) 在局部变量表里查首个命中的变量名。
///
/// 契约：`proto` 指向存活 Proto；返回的 `&str` 寿命随该借用（名字串随 Proto 可达）。
pub(crate) fn try_find_local_name(proto: &Proto, reg: i32, pcpos: i32) -> Option<&str> {
  // 顺序扫描命中首个匹配即返回（对齐 cpp 语义）；表为空时自然迭代零次。
  locvars(proto)
    .iter()
    .find(|local| reg == local.reg as i32 && pcpos >= local.startpc && pcpos < local.endpc)
    .and_then(|local| name_str(local.varname as *const _, proto))
}
