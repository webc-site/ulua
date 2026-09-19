use std::string::String;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::format_append::format_append,
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::luau_insn_a, luau_insn_aux_kv_16::luau_insn_aux_kv16,
    luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c, luau_insn_d::luau_insn_d,
    luau_insn_op::luau_insn_op,
  },
};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn dump_instruction(&self, code: &[u32], result: &mut String, target_label: i32) -> usize {
    let insn = code[0];
    let op = luau_insn_op(insn);
    let op_enum = LuauOpcode::from(op as u8);

    match op_enum {
      LuauOpcode::LOP_LOADNIL => {
        format_append(result, format_args!("LOADNIL R{}\n", luau_insn_a(insn)));
        1
      }
      LuauOpcode::LOP_LOADB => {
        if luau_insn_c(insn) != 0 {
          format_append(
            result,
            format_args!(
              "LOADB R{} {} +{}\n",
              luau_insn_a(insn),
              luau_insn_b(insn),
              luau_insn_c(insn)
            ),
          );
        } else {
          format_append(
            result,
            format_args!("LOADB R{} {}\n", luau_insn_a(insn), luau_insn_b(insn)),
          );
        }
        1
      }
      LuauOpcode::LOP_LOADN => {
        format_append(
          result,
          format_args!("LOADN R{} {}\n", luau_insn_a(insn), luau_insn_d(insn)),
        );
        1
      }
      LuauOpcode::LOP_LOADK => {
        format_append(
          result,
          format_args!("LOADK R{} K{} [", luau_insn_a(insn), luau_insn_d(insn)),
        );
        self.dump_constant(result, luau_insn_d(insn), false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_MOVE => {
        format_append(
          result,
          format_args!("MOVE R{} R{}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_GETGLOBAL => {
        format_append(
          result,
          format_args!("GETGLOBAL R{} K{} [", luau_insn_a(insn), code[1]),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_SETGLOBAL => {
        format_append(
          result,
          format_args!("SETGLOBAL R{} K{} [", luau_insn_a(insn), code[1]),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_GETUPVAL => {
        format_append(
          result,
          format_args!("GETUPVAL R{} {}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_SETUPVAL => {
        format_append(
          result,
          format_args!("SETUPVAL R{} {}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_CLOSEUPVALS => {
        format_append(result, format_args!("CLOSEUPVALS R{}\n", luau_insn_a(insn)));
        1
      }
      LuauOpcode::LOP_GETIMPORT => {
        format_append(
          result,
          format_args!("GETIMPORT R{} {} [", luau_insn_a(insn), luau_insn_d(insn)),
        );
        self.dump_constant(result, luau_insn_d(insn), false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_GETTABLE => {
        format_append(
          result,
          format_args!(
            "GETTABLE R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_SETTABLE => {
        format_append(
          result,
          format_args!(
            "SETTABLE R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_GETTABLEKS => {
        format_append(
          result,
          format_args!(
            "GETTABLEKS R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1]
          ),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_SETTABLEKS => {
        format_append(
          result,
          format_args!(
            "SETTABLEKS R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1]
          ),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_GETTABLEN => {
        format_append(
          result,
          format_args!(
            "GETTABLEN R{} R{} {}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn) + 1
          ),
        );
        1
      }
      LuauOpcode::LOP_SETTABLEN => {
        format_append(
          result,
          format_args!(
            "SETTABLEN R{} R{} {}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn) + 1
          ),
        );
        1
      }
      LuauOpcode::LOP_NEWCLOSURE => {
        format_append(
          result,
          format_args!("NEWCLOSURE R{} P{}\n", luau_insn_a(insn), luau_insn_d(insn)),
        );
        1
      }
      LuauOpcode::LOP_NAMECALL => {
        format_append(
          result,
          format_args!(
            "NAMECALL R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1]
          ),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_CALL => {
        format_append(
          result,
          format_args!(
            "CALL R{} {} {}\n",
            luau_insn_a(insn),
            luau_insn_b(insn) as i32 - 1,
            luau_insn_c(insn) as i32 - 1
          ),
        );
        1
      }
      LuauOpcode::LOP_CALLFB => {
        format_append(
          result,
          format_args!(
            "CALLFB R{} {} {} [{}]\n",
            luau_insn_a(insn),
            luau_insn_b(insn) as i32 - 1,
            luau_insn_c(insn) as i32 - 1,
            code[1] as i32
          ),
        );
        2
      }
      LuauOpcode::LOP_RETURN => {
        format_append(
          result,
          format_args!(
            "RETURN R{} {}\n",
            luau_insn_a(insn),
            luau_insn_b(insn) as i32 - 1
          ),
        );
        1
      }
      LuauOpcode::LOP_JUMP => {
        format_append(result, format_args!("JUMP L{}\n", target_label));
        1
      }
      LuauOpcode::LOP_JUMPIF => {
        format_append(
          result,
          format_args!("JUMPIF R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_JUMPIFNOT => {
        format_append(
          result,
          format_args!("JUMPIFNOT R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_JUMPIFEQ => {
        format_append(
          result,
          format_args!(
            "JUMPIFEQ R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPIFLE => {
        format_append(
          result,
          format_args!(
            "JUMPIFLE R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPIFLT => {
        format_append(
          result,
          format_args!(
            "JUMPIFLT R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPIFNOTEQ => {
        format_append(
          result,
          format_args!(
            "JUMPIFNOTEQ R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPIFNOTLE => {
        format_append(
          result,
          format_args!(
            "JUMPIFNOTLE R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPIFNOTLT => {
        format_append(
          result,
          format_args!(
            "JUMPIFNOTLT R{} R{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_ADD => {
        format_append(
          result,
          format_args!(
            "ADD R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_SUB => {
        format_append(
          result,
          format_args!(
            "SUB R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_MUL => {
        format_append(
          result,
          format_args!(
            "MUL R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_DIV => {
        format_append(
          result,
          format_args!(
            "DIV R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_IDIV => {
        format_append(
          result,
          format_args!(
            "IDIV R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_MOD => {
        format_append(
          result,
          format_args!(
            "MOD R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_POW => {
        format_append(
          result,
          format_args!(
            "POW R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_ADDK => {
        format_append(
          result,
          format_args!(
            "ADDK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_SUBK => {
        format_append(
          result,
          format_args!(
            "SUBK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_MULK => {
        format_append(
          result,
          format_args!(
            "MULK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_DIVK => {
        format_append(
          result,
          format_args!(
            "DIVK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_IDIVK => {
        format_append(
          result,
          format_args!(
            "IDIVK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_MODK => {
        format_append(
          result,
          format_args!(
            "MODK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_POWK => {
        format_append(
          result,
          format_args!(
            "POWK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_SUBRK => {
        format_append(
          result,
          format_args!("SUBRK R{} K{} [", luau_insn_a(insn), luau_insn_b(insn)),
        );
        self.dump_constant(result, luau_insn_b(insn) as i32, false);
        format_append(result, format_args!("] R{}\n", luau_insn_c(insn)));
        1
      }
      LuauOpcode::LOP_DIVRK => {
        format_append(
          result,
          format_args!("DIVRK R{} K{} [", luau_insn_a(insn), luau_insn_b(insn)),
        );
        self.dump_constant(result, luau_insn_b(insn) as i32, false);
        format_append(result, format_args!("] R{}\n", luau_insn_c(insn)));
        1
      }
      LuauOpcode::LOP_AND => {
        format_append(
          result,
          format_args!(
            "AND R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_OR => {
        format_append(
          result,
          format_args!(
            "OR R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_ANDK => {
        format_append(
          result,
          format_args!(
            "ANDK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_ORK => {
        format_append(
          result,
          format_args!(
            "ORK R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, luau_insn_c(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_CONCAT => {
        format_append(
          result,
          format_args!(
            "CONCAT R{} R{} R{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_NOT => {
        format_append(
          result,
          format_args!("NOT R{} R{}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_MINUS => {
        format_append(
          result,
          format_args!("MINUS R{} R{}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_LENGTH => {
        format_append(
          result,
          format_args!("LENGTH R{} R{}\n", luau_insn_a(insn), luau_insn_b(insn)),
        );
        1
      }
      LuauOpcode::LOP_NEWTABLE => {
        format_append(
          result,
          format_args!(
            "NEWTABLE R{} {} {}\n",
            luau_insn_a(insn),
            if luau_insn_b(insn) == 0 {
              0
            } else {
              1 << (luau_insn_b(insn) as i32 - 1)
            },
            code[1]
          ),
        );
        2
      }
      LuauOpcode::LOP_DUPTABLE => {
        format_append(
          result,
          format_args!("DUPTABLE R{} {}\n", luau_insn_a(insn), luau_insn_d(insn)),
        );
        1
      }
      LuauOpcode::LOP_SETLIST => {
        format_append(
          result,
          format_args!(
            "SETLIST R{} R{} {} [{}]\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_c(insn) as i32 - 1,
            code[1]
          ),
        );
        2
      }
      LuauOpcode::LOP_FORNPREP => {
        format_append(
          result,
          format_args!("FORNPREP R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_FORNLOOP => {
        format_append(
          result,
          format_args!("FORNLOOP R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_FORGPREP => {
        format_append(
          result,
          format_args!("FORGPREP R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_FORGLOOP => {
        format_append(
          result,
          format_args!(
            "FORGLOOP R{} L{} {}{}\n",
            luau_insn_a(insn),
            target_label,
            code[1] as u8,
            if (code[1] as i32) < 0 { " [inext]" } else { "" }
          ),
        );
        2
      }
      LuauOpcode::LOP_FORGPREP_INEXT => {
        format_append(
          result,
          format_args!("FORGPREP_INEXT R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_FORGPREP_NEXT => {
        format_append(
          result,
          format_args!("FORGPREP_NEXT R{} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_GETVARARGS => {
        format_append(
          result,
          format_args!(
            "GETVARARGS R{} {}\n",
            luau_insn_a(insn),
            luau_insn_b(insn) as i32 - 1
          ),
        );
        1
      }
      LuauOpcode::LOP_DUPCLOSURE => {
        format_append(
          result,
          format_args!("DUPCLOSURE R{} K{} [", luau_insn_a(insn), luau_insn_d(insn)),
        );
        self.dump_constant(result, luau_insn_d(insn) as i32, false);
        result.push_str("]\n");
        1
      }
      LuauOpcode::LOP_BREAK => {
        format_append(result, format_args!("BREAK\n"));
        1
      }
      LuauOpcode::LOP_JUMPBACK => {
        format_append(result, format_args!("JUMPBACK L{}\n", target_label));
        1
      }
      LuauOpcode::LOP_LOADKX => {
        format_append(
          result,
          format_args!("LOADKX R{} K{} [", luau_insn_a(insn), code[1]),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_JUMPX => {
        format_append(result, format_args!("JUMPX L{}\n", target_label));
        1
      }
      LuauOpcode::LOP_FASTCALL => {
        format_append(
          result,
          format_args!("FASTCALL {} L{}\n", luau_insn_a(insn), target_label),
        );
        1
      }
      LuauOpcode::LOP_FASTCALL1 => {
        format_append(
          result,
          format_args!(
            "FASTCALL1 {} R{} L{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            target_label
          ),
        );
        1
      }
      LuauOpcode::LOP_FASTCALL2 => {
        format_append(
          result,
          format_args!(
            "FASTCALL2 {} R{} R{} L{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_FASTCALL2K => {
        format_append(
          result,
          format_args!(
            "FASTCALL2K {} R{} K{} L{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1],
            target_label
          ),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_FASTCALL3 => {
        format_append(
          result,
          format_args!(
            "FASTCALL3 {} R{} R{} R{} L{}\n",
            luau_insn_a(insn),
            luau_insn_b(insn),
            code[1] & 0xff,
            (code[1] >> 8) & 0xff,
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_COVERAGE => {
        format_append(result, format_args!("COVERAGE\n"));
        1
      }
      LuauOpcode::LOP_CAPTURE => {
        let a = luau_insn_a(insn);
        // LCT_VAL = 0, LCT_REF = 1, LCT_UPVAL = 2 (the port had VAL/UPVAL swapped).
        let capture_name = match a {
          0 => "VAL",
          1 => "REF",
          2 => "UPVAL",
          _ => "",
        };
        format_append(
          result,
          format_args!(
            "CAPTURE {} {}{}\n",
            capture_name,
            if a == 2 { 'U' } else { 'R' },
            luau_insn_b(insn)
          ),
        );
        1
      }
      LuauOpcode::LOP_JUMPXEQKNIL => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKNIL R{} L{}{}\n",
            luau_insn_a(insn),
            target_label,
            if (code[1] >> 31) != 0 { " NOT" } else { "" }
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKB R{} {} L{}{}\n",
            luau_insn_a(insn),
            code[1] & 1,
            target_label,
            if (code[1] >> 31) != 0 { " NOT" } else { "" }
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKN => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKN R{} K{} L{}{} [",
            luau_insn_a(insn),
            code[1] & 0xffffff,
            target_label,
            if (code[1] >> 31) != 0 { " NOT" } else { "" }
          ),
        );
        self.dump_constant(result, (code[1] & 0xffffff) as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_JUMPXEQKS => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKS R{} K{} L{}{} [",
            luau_insn_a(insn),
            code[1] & 0xffffff,
            target_label,
            if (code[1] >> 31) != 0 { " NOT" } else { "" }
          ),
        );
        self.dump_constant(result, (code[1] & 0xffffff) as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_GETUDATAKS => {
        format_append(
          result,
          format_args!(
            "GETUDATAKS R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_aux_kv16(code[1])
          ),
        );
        self.dump_constant(result, luau_insn_aux_kv16(code[1]) as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_SETUDATAKS => {
        format_append(
          result,
          format_args!(
            "SETUDATAKS R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_aux_kv16(code[1])
          ),
        );
        self.dump_constant(result, luau_insn_aux_kv16(code[1]) as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_NAMECALLUDATA => {
        format_append(
          result,
          format_args!(
            "NAMECALLUDATA R{} R{} K{} [",
            luau_insn_a(insn),
            luau_insn_b(insn),
            luau_insn_aux_kv16(code[1])
          ),
        );
        self.dump_constant(result, luau_insn_aux_kv16(code[1]) as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_NEWCLASSMEMBER => {
        format_append(
          result,
          format_args!(
            "NEWCLASSMEMBER R{} R{} [",
            luau_insn_a(insn),
            luau_insn_c(insn)
          ),
        );
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      LuauOpcode::LOP_CMPPROTO => {
        format_append(
          result,
          format_args!(
            "CMPPROTO R{} #{} L{}\n",
            luau_insn_a(insn),
            code[1],
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_FASTPCALL => {
        format_append(
          result,
          format_args!(
            "FASTPCALL {} L{}\n",
            if luau_insn_a(insn) == 0 {
              "pcall"
            } else {
              "xpcall"
            },
            target_label
          ),
        );
        1
      }
      LuauOpcode::LOP_NEWCLASS => {
        let b = luau_insn_b(insn);
        if b == 0xff {
          format_append(
            result,
            format_args!(
              "NEWCLASS R{} no_base K{} {} [",
              luau_insn_a(insn),
              code[1],
              luau_insn_c(insn)
            ),
          );
        } else {
          format_append(
            result,
            format_args!(
              "NEWCLASS R{} R{} K{} {} [",
              luau_insn_a(insn),
              b,
              code[1],
              luau_insn_c(insn)
            ),
          );
        }
        self.dump_constant(result, code[1] as i32, false);
        result.push_str("]\n");
        2
      }
      _ => {
        LUAU_ASSERT!(false);
        1
      }
    }
  }
}
