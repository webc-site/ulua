use crate::enums::ir_block_kind::IrBlockKind;

/// cpp `IrData.h` 块排序优先级：Fallback→1、ExitSync→2、其余→0。
/// 以 `IrBlockKind`（`#[repr(u8)]`，判别式 0..=5）为下标的编译期定表；越界
/// （理论上不可达）取 0，与旧 if 链的默认分支一致。
const BLOCK_KIND_PRIORITY: [i32; 6] = [0, 1, 0, 0, 2, 0];

const _: () = {
  assert!(
    BLOCK_KIND_PRIORITY[IrBlockKind::Fallback as usize] == 1,
    "Fallback 优先级应为 1"
  );
  assert!(
    BLOCK_KIND_PRIORITY[IrBlockKind::ExitSync as usize] == 2,
    "ExitSync 优先级应为 2"
  );
  assert!(
    BLOCK_KIND_PRIORITY[IrBlockKind::Bytecode as usize] == 0,
    "Bytecode 优先级应为 0"
  );
};

pub fn get_block_kind_priority(kind: IrBlockKind) -> i32 {
  *BLOCK_KIND_PRIORITY.get(kind as usize).unwrap_or(&0)
}
