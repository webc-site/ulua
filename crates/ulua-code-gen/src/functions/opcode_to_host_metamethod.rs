use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::enums::host_metamethod::HostMetamethod;

#[inline]
pub fn opcode_to_host_metamethod(op: LuauOpcode) -> HostMetamethod {
  match op {
    LuauOpcode::LOP_ADD => HostMetamethod::Add,
    LuauOpcode::LOP_SUB => HostMetamethod::Sub,
    LuauOpcode::LOP_MUL => HostMetamethod::Mul,
    LuauOpcode::LOP_DIV => HostMetamethod::Div,
    LuauOpcode::LOP_IDIV => HostMetamethod::Idiv,
    LuauOpcode::LOP_MOD => HostMetamethod::Mod,
    LuauOpcode::LOP_POW => HostMetamethod::Pow,

    LuauOpcode::LOP_ADDK => HostMetamethod::Add,
    LuauOpcode::LOP_SUBK => HostMetamethod::Sub,
    LuauOpcode::LOP_MULK => HostMetamethod::Mul,
    LuauOpcode::LOP_DIVK => HostMetamethod::Div,
    LuauOpcode::LOP_IDIVK => HostMetamethod::Idiv,
    LuauOpcode::LOP_MODK => HostMetamethod::Mod,
    LuauOpcode::LOP_POWK => HostMetamethod::Pow,

    LuauOpcode::LOP_SUBRK => HostMetamethod::Sub,
    LuauOpcode::LOP_DIVRK => HostMetamethod::Div,

    _ => HostMetamethod::Add,
  }
}
