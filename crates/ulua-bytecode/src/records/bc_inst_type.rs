/// 指令包装类型与其 opcode 的映射（自原 methods/bc_function_as.rs 收敛而来）。
pub trait BcInstType {
  const OPCODE: i32;
}
