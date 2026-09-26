use crate::enums::luau_opcode::LuauOpcode;

/// 全部操作码变体数：`LOP__COUNT` 是末位哨兵，其判别式 + 1 即枚举总长度，
/// 恰好覆盖 `0..=LOP__COUNT` 全部取值，令查表无需越界分支。
const OP_COUNT: usize = LuauOpcode::LOP__COUNT as usize + 1;

/// 需占 2 个字长的操作码列表，逐条对应 cpp `getOpLength` switch 里那组
/// `return 2` 的 `case`（`Common/include/Luau/BytecodeUtils.h`）。
const WIDE: [LuauOpcode; 30] = [
  LuauOpcode::LOP_GETGLOBAL,
  LuauOpcode::LOP_SETGLOBAL,
  LuauOpcode::LOP_GETIMPORT,
  LuauOpcode::LOP_GETTABLEKS,
  LuauOpcode::LOP_SETTABLEKS,
  LuauOpcode::LOP_NAMECALL,
  LuauOpcode::LOP_JUMPIFEQ,
  LuauOpcode::LOP_JUMPIFLE,
  LuauOpcode::LOP_JUMPIFLT,
  LuauOpcode::LOP_JUMPIFNOTEQ,
  LuauOpcode::LOP_JUMPIFNOTLE,
  LuauOpcode::LOP_JUMPIFNOTLT,
  LuauOpcode::LOP_NEWTABLE,
  LuauOpcode::LOP_SETLIST,
  LuauOpcode::LOP_FORGLOOP,
  LuauOpcode::LOP_LOADKX,
  LuauOpcode::LOP_FASTCALL2,
  LuauOpcode::LOP_FASTCALL2K,
  LuauOpcode::LOP_FASTCALL3,
  LuauOpcode::LOP_JUMPXEQKNIL,
  LuauOpcode::LOP_JUMPXEQKB,
  LuauOpcode::LOP_JUMPXEQKN,
  LuauOpcode::LOP_JUMPXEQKS,
  LuauOpcode::LOP_GETUDATAKS,
  LuauOpcode::LOP_SETUDATAKS,
  LuauOpcode::LOP_NAMECALLUDATA,
  LuauOpcode::LOP_NEWCLASSMEMBER,
  LuauOpcode::LOP_CALLFB,
  LuauOpcode::LOP_CMPPROTO,
  LuauOpcode::LOP_NEWCLASS,
];

/// 编译期定表：`OP_LEN[op as usize]` 即该指令占用的字长（1 或 2）。
const OP_LEN: [i8; OP_COUNT] = build_op_len();

/// 构造 `OP_LEN`：先对 `ALL`（全部 `OP_COUNT` 槽位）逐个赋默认 `1`（对应 cpp
/// `getOpLength` 的 `default: return 1`），再把 `WIDE` 列表命中项改写为 `2`。
const fn build_op_len() -> [i8; OP_COUNT] {
  let mut table = [1i8; OP_COUNT];
  let mut i = 0;
  while i < WIDE.len() {
    table[WIDE[i] as usize] = 2;
    i += 1;
  }
  table
}

// 定表自检：编译期锁定与 cpp oracle 一致的关键采样，漂表即编译失败。
const _: () = {
  assert!(WIDE.len() == 30);
  assert!(OP_LEN[LuauOpcode::LOP_NAMECALL as usize] == 2);
  assert!(OP_LEN[LuauOpcode::LOP_NEWCLASS as usize] == 2);
  assert!(OP_LEN[LuauOpcode::LOP_FASTCALL3 as usize] == 2);
  assert!(OP_LEN[LuauOpcode::LOP_ADD as usize] == 1);
  assert!(OP_LEN[LuauOpcode::LOP_FASTPCALL as usize] == 1);
  // 哨兵 `LOP__COUNT` 与越界钳位后的 `NOP` 均落 default 分支，长度为 1。
  assert!(OP_LEN[LuauOpcode::LOP__COUNT as usize] == 1);
  assert!(OP_LEN[LuauOpcode::LOP_NOP as usize] == 1);
};

/// 操作码指令长度（占用的 32 位字数）。
///
/// 出处：cpp `Common/include/Luau/BytecodeUtils.h` 的 `getOpLength`——命中 2 字
/// 长指令列表返回 `2`，其余 `default` 返回 `1`。原为 30 臂 `|` 合并 `match`，
/// 现改为编译期定表 `OP_LEN`，调用侧退化为零分支查表。
pub const fn get_op_length(op: LuauOpcode) -> i32 {
  OP_LEN[op as usize] as i32
}
