use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{luai_numidiv::luai_numidiv, luai_nummod::luai_nummod},
};

// IrUtils.cpp: `constexpr double kDoubleMaxExactInteger = 9007199254740992.0;`
use crate::enums::ir_condition::IrCondition;
use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    byteswap::byteswap, compare_ir_utils::compare_f64_f64_ir_condition,
    compare_ir_utils_alt_b::compare_i32_i32_ir_condition,
    compare_ir_utils_alt_c::compare_i64_i64_ir_condition, condition_op::condition_op,
    countlz_bit_utils::countlz_u32, countlz_bit_utils_alt_b::countlz_u64,
    countrz_bit_utils::countrz_u32, countrz_bit_utils_alt_b::countrz_u64,
    get_op_ir_data::get_op_mut, kill_ir_utils::kill_ir_function_ir_inst, lrotate::lrotate,
    replace_ir_utils::replace_ir_function_ir_op_ir_op,
    replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst, rrotate::rrotate,
    substitute::substitute, substitute_with_truncated_uint::substitute_with_truncated_uint,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp,
  },
  type_aliases::ir_ops::IrOps,
};
const K_DOUBLE_MAX_EXACT_INTEGER: f64 = 9007199254740992.0;

const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;

fn make_inst(cmd: IrCmd, ops: &[IrOp]) -> IrInst {
  let mut v = IrOps::new();
  for &o in ops {
    v.push(o);
  }
  IrInst {
    cmd,
    ops: v,
    ..Default::default()
  }
}

pub fn fold_constants(
  build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &mut IrBlock,
  index: u32,
) {
  let inst_ptr: *mut IrInst = &mut function.instructions[index as usize];
  let cmd = unsafe { (*inst_ptr).cmd };

  // OP_A(inst)..OP_F(inst): read the n-th operand (copy; IrOp is Copy)
  let read = move |idx: usize| -> IrOp {
    let s = unsafe { (*inst_ptr).ops.as_slice() };
    if idx < s.len() {
      s[idx]
    } else {
      IrOp::default()
    }
  };
  let is_const = move |idx: usize| -> bool {
    let s = unsafe { (*inst_ptr).ops.as_slice() };
    let op = if idx < s.len() {
      s[idx]
    } else {
      IrOp::default()
    };
    op.kind() == IrOpKind::Constant
  };

  match cmd {
    IrCmd::AddInt => {
      if is_const(0) && is_const(1) {
        // Add as unsigned to force two's complement evaluation (avoid signed overflow UB)
        let lhs = function.int_op(read(0));
        let rhs = function.int_op(read(1));
        let sum = lhs.wrapping_add(rhs);
        let c = build.const_int(sum);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SubInt => {
      if is_const(0) && is_const(1) {
        let lhs = function.int_op(read(0));
        let rhs = function.int_op(read(1));
        let sum = lhs.wrapping_sub(rhs);
        let c = build.const_int(sum);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::Sexti8Int => {
      if is_const(0) {
        let value = function.int_op(read(0)) as i8 as i32;
        let c = build.const_int(value);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::Sexti16Int => {
      if is_const(0) {
        let value = function.int_op(read(0)) as i16 as i32;
        let c = build.const_int(value);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::AddNum => {
      if is_const(0) && is_const(1) {
        let v = function.double_op(read(0)) + function.double_op(read(1));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SubNum => {
      if is_const(0) && is_const(1) {
        let v = function.double_op(read(0)) - function.double_op(read(1));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MulNum => {
      if is_const(0) && is_const(1) {
        let v = function.double_op(read(0)) * function.double_op(read(1));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::DivNum => {
      if is_const(0) && is_const(1) {
        let v = function.double_op(read(0)) / function.double_op(read(1));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::IdivNum => {
      if is_const(0) && is_const(1) {
        let v = luai_numidiv(function.double_op(read(0)), function.double_op(read(1)));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::ModNum => {
      if is_const(0) && is_const(1) {
        let v = luai_nummod(function.double_op(read(0)), function.double_op(read(1)));
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MinNum => {
      if is_const(0) && is_const(1) {
        let a1 = function.double_op(read(0));
        let a2 = function.double_op(read(1));
        let c = build.const_double(if a1 < a2 { a1 } else { a2 });
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MaxNum => {
      if is_const(0) && is_const(1) {
        let a1 = function.double_op(read(0));
        let a2 = function.double_op(read(1));
        let c = build.const_double(if a1 > a2 { a1 } else { a2 });
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::UnmNum => {
      if is_const(0) {
        let c = build.const_double(-function.double_op(read(0)));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::FloorNum => {
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)).floor());
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CeilNum => {
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)).ceil());
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::RoundNum => {
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)).round());
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SqrtNum => {
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)).sqrt());
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::AbsNum => {
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)).abs());
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SignNum => {
      if is_const(0) {
        let v = function.double_op(read(0));
        let r = if v > 0.0 {
          1.0
        } else if v < 0.0 {
          -1.0
        } else {
          0.0
        };
        let c = build.const_double(r);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::AddFloat => {
      if is_const(0) && is_const(1) {
        let v =
          ((function.double_op(read(0)) as f32) + (function.double_op(read(1)) as f32)) as f64;
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SubFloat => {
      if is_const(0) && is_const(1) {
        let v =
          ((function.double_op(read(0)) as f32) - (function.double_op(read(1)) as f32)) as f64;
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MulFloat => {
      if is_const(0) && is_const(1) {
        let v =
          ((function.double_op(read(0)) as f32) * (function.double_op(read(1)) as f32)) as f64;
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::DivFloat => {
      if is_const(0) && is_const(1) {
        let v =
          ((function.double_op(read(0)) as f32) / (function.double_op(read(1)) as f32)) as f64;
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MinFloat => {
      if is_const(0) && is_const(1) {
        let a1 = function.double_op(read(0)) as f32;
        let a2 = function.double_op(read(1)) as f32;
        let c = build.const_double((if a1 < a2 { a1 } else { a2 }) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MaxFloat => {
      if is_const(0) && is_const(1) {
        let a1 = function.double_op(read(0)) as f32;
        let a2 = function.double_op(read(1)) as f32;
        let c = build.const_double((if a1 > a2 { a1 } else { a2 }) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::UnmFloat => {
      if is_const(0) {
        let c = build.const_double((-(function.double_op(read(0)) as f32)) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::FloorFloat => {
      if is_const(0) {
        let c = build.const_double((function.double_op(read(0)) as f32).floor() as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CeilFloat => {
      if is_const(0) {
        let c = build.const_double((function.double_op(read(0)) as f32).ceil() as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SqrtFloat => {
      if is_const(0) {
        let c = build.const_double((function.double_op(read(0)) as f32).sqrt() as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::AbsFloat => {
      if is_const(0) {
        let c = build.const_double((function.double_op(read(0)) as f32).abs() as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SignFloat => {
      if is_const(0) {
        let v = function.double_op(read(0)) as f32;
        let r: f32 = if v > 0.0 {
          1.0
        } else if v < 0.0 {
          -1.0
        } else {
          0.0
        };
        let c = build.const_double(r as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SelectNum => {
      if is_const(2) && is_const(3) {
        let c = function.double_op(read(2));
        let d = function.double_op(read(3));
        let repl = if c == d { read(1) } else { read(0) };
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      } else if read(0) == read(1) {
        // If the values are the same, no need to worry about the condition check
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::SelectVec => {
      if read(0) == read(1) {
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::SelectIfTruthy => {
      if read(1) == read(2) {
        let repl = read(1);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::NotAny => {
      if is_const(0) {
        let a = function.tag_op(read(0));

        if a == LUA_TNIL {
          let c = build.const_int(1);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        } else if a != LUA_TBOOLEAN {
          let c = build.const_int(0);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        } else if is_const(1) {
          let c = build.const_int(if function.int_op(read(1)) == 1 { 0 } else { 1 });
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::CmpInt => {
      if is_const(0) && is_const(1) {
        let res = compare_i32_i32_ir_condition(
          function.int_op(read(0)),
          function.int_op(read(1)),
          condition_op(read(2)),
        );
        let c = build.const_int(if res { 1 } else { 0 });
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CmpInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        let res = compare_i64_i64_ir_condition(lhs, rhs, condition_op(read(2)));
        let c = build.const_int(if res { 1 } else { 0 });
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CmpTag => {
      if is_const(0) && is_const(1) {
        let cond = condition_op(read(2));
        CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);

        let same = function.tag_op(read(0)) == function.tag_op(read(1));
        let val = if cond == IrCondition::Equal {
          if same { 1 } else { 0 }
        } else if !same {
          1
        } else {
          0
        };
        let c = build.const_int(val);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CmpSplitTvalue => {
      CODEGEN_ASSERT!(is_const(1));

      let cond = condition_op(read(4));
      CODEGEN_ASSERT!(cond == IrCondition::Equal || cond == IrCondition::NotEqual);

      if cond == IrCondition::Equal {
        if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
          let c = build.const_int(0);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        } else if is_const(2) && is_const(3) {
          // If the tag is a constant, this means previous condition has failed because tags are the same
          let known_same_tag = is_const(0);
          let tag_b = function.tag_op(read(1));

          let same_value = if tag_b == LUA_TBOOLEAN {
            compare_i32_i32_ir_condition(
              function.int_op(read(2)),
              function.int_op(read(3)),
              IrCondition::Equal,
            )
          } else if tag_b == LUA_TNUMBER {
            compare_f64_f64_ir_condition(
              function.double_op(read(2)),
              function.double_op(read(3)),
              IrCondition::Equal,
            )
          } else if tag_b == LUA_TINTEGER {
            let lhs = function.int64_op(read(2));
            let rhs = function.int64_op(read(3));
            compare_i64_i64_ir_condition(lhs, rhs, IrCondition::Equal)
          } else {
            CODEGEN_ASSERT!(false, "unsupported type");
            false
          };

          if known_same_tag && same_value {
            let c = build.const_int(1);
            substitute(function, unsafe { &mut *inst_ptr }, c);
          } else if same_value {
            let r = make_inst(IrCmd::CmpTag, &[read(0), read(1), read(4)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
          } else {
            let c = build.const_int(0);
            substitute(function, unsafe { &mut *inst_ptr }, c);
          }
        }
      } else {
        if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
          let c = build.const_int(1);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        } else if is_const(2) && is_const(3) {
          let known_same_tag = is_const(0);
          let tag_b = function.tag_op(read(1));

          let different_value = if tag_b == LUA_TBOOLEAN {
            compare_i32_i32_ir_condition(
              function.int_op(read(2)),
              function.int_op(read(3)),
              IrCondition::NotEqual,
            )
          } else if tag_b == LUA_TNUMBER {
            compare_f64_f64_ir_condition(
              function.double_op(read(2)),
              function.double_op(read(3)),
              IrCondition::NotEqual,
            )
          } else if tag_b == LUA_TINTEGER {
            let lhs = function.int64_op(read(2));
            let rhs = function.int64_op(read(3));
            compare_i64_i64_ir_condition(lhs, rhs, IrCondition::NotEqual)
          } else {
            CODEGEN_ASSERT!(false, "unsupported type");
            false
          };

          if different_value {
            let c = build.const_int(1);
            substitute(function, unsafe { &mut *inst_ptr }, c);
          } else if known_same_tag {
            let c = build.const_int(0);
            substitute(function, unsafe { &mut *inst_ptr }, c);
          } else {
            let r = make_inst(IrCmd::CmpTag, &[read(0), read(1), read(4)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
          }
        }
      }
    }
    IrCmd::JumpEqTag => {
      if is_const(0) && is_const(1) {
        if function.tag_op(read(0)) == function.tag_op(read(1)) {
          let r = make_inst(IrCmd::JUMP, &[read(2)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(3)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::JumpCmpInt => {
      if is_const(0) && is_const(1) {
        let res = compare_i32_i32_ir_condition(
          function.int_op(read(0)),
          function.int_op(read(1)),
          condition_op(read(2)),
        );
        let r = if res {
          make_inst(IrCmd::JUMP, &[read(3)])
        } else {
          make_inst(IrCmd::JUMP, &[read(4)])
        };
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      }
    }
    IrCmd::JumpCmpNum => {
      if is_const(0) && is_const(1) {
        let res = compare_f64_f64_ir_condition(
          function.double_op(read(0)),
          function.double_op(read(1)),
          condition_op(read(2)),
        );
        let r = if res {
          make_inst(IrCmd::JUMP, &[read(3)])
        } else {
          make_inst(IrCmd::JUMP, &[read(4)])
        };
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      }
    }
    IrCmd::JumpCmpFloat => {
      if is_const(0) && is_const(1) {
        let a = function.double_op(read(0)) as f32 as f64;
        let b = function.double_op(read(1)) as f32 as f64;
        let res = compare_f64_f64_ir_condition(a, b, condition_op(read(2)));
        let r = if res {
          make_inst(IrCmd::JUMP, &[read(3)])
        } else {
          make_inst(IrCmd::JUMP, &[read(4)])
        };
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      }
    }
    IrCmd::TryNumToIndex => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // To avoid undefined behavior of casting a value not representable in the target type, we check the range
        if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
          let arr_index = value as i32;

          if arr_index as f64 == value {
            let c = build.const_int(arr_index);
            substitute(function, unsafe { &mut *inst_ptr }, c);
          } else {
            let r = make_inst(IrCmd::JUMP, &[read(1)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
          }
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(1)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::IntToNum => {
      if is_const(0) {
        let c = build.const_double(function.int_op(read(0)) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::Int64ToNum => {
      if is_const(0) {
        let v = function.int64_op(read(0)) as f64;
        let c = build.const_double(v);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::UintToNum => {
      if is_const(0) {
        let c = build.const_double((function.int_op(read(0)) as u32) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::UintToFloat => {
      if is_const(0) {
        let c = build.const_double(((function.int_op(read(0)) as u32) as f32) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::NumToInt => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // matches luai_num2int range check
        if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
          let c = build.const_int(value as i32);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::NumToUint => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // matches luai_num2unsigned range check
        if (-K_DOUBLE_MAX_EXACT_INTEGER..=K_DOUBLE_MAX_EXACT_INTEGER).contains(&value) {
          let c = build.const_int((value as i64 as u32) as i32);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::NumToInt64 => {
      if is_const(0) {
        let value = function.double_op(read(0));

        if value >= i64::MIN as f64 && value < i64::MAX as f64 {
          let c = build.const_int_64(value as i64);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::FloatToNum => {
      // float -> double for a constant is a no-op
      if is_const(0) {
        let c = build.const_double(function.double_op(read(0)));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::NumToFloat => {
      // double -> float for a constant just needs to lower precision
      if is_const(0) {
        let c = build.const_double((function.double_op(read(0)) as f32) as f64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::TruncateUint => {
      // Truncating a constant integer is a no-op as constant integers only store 32 bits
      if is_const(0) {
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::CheckTag => {
      if is_const(0) && is_const(1) {
        if function.tag_op(read(0)) == function.tag_op(read(1)) {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::CheckTruthy => {
      if is_const(0) {
        if function.tag_op(read(0)) == LUA_TNIL {
          let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        } else if function.tag_op(read(0)) == LUA_TBOOLEAN {
          if is_const(1) {
            if function.int_op(read(1)) == 0 {
              let r = make_inst(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
              replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
            } else {
              kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
            }
          }
        } else {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        }
      }
    }
    IrCmd::CheckCmpNum => {
      if is_const(0) && is_const(1) {
        if compare_f64_f64_ir_condition(
          function.double_op(read(0)),
          function.double_op(read(1)),
          condition_op(read(2)),
        ) {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(3)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::CheckCmpInt => {
      if is_const(0) && is_const(1) {
        if compare_i32_i32_ir_condition(
          function.int_op(read(0)),
          function.int_op(read(1)),
          condition_op(read(2)),
        ) {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(3)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::AddInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        let c = build.const_int_64(lhs.wrapping_add(rhs));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::SubInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        let c = build.const_int_64(lhs.wrapping_sub(rhs));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::MulInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        let c = build.const_int_64(lhs.wrapping_mul(rhs));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::DivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let c = build.const_int_64(lhs / rhs);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::IdivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let mut q = lhs / rhs;
          // Floored division: adjust if signs differ and there's a remainder
          if (lhs ^ rhs) < 0 && q.wrapping_mul(rhs) != lhs {
            q -= 1;
          }
          let c = build.const_int_64(q);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::UdivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0)) as u64;
        let rhs = function.int64_op(read(1)) as u64;
        if let Some(q) = lhs.checked_div(rhs) {
          let c = build.const_int_64(q as i64);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::RemInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let c = build.const_int_64(lhs % rhs);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::UremInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0)) as u64;
        let rhs = function.int64_op(read(1)) as u64;
        if rhs != 0 {
          let c = build.const_int_64((lhs % rhs) as i64);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::ModInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let mut rem = lhs % rhs;
          // Floored modulus: adjust if remainder != 0 and signs differ
          if rem != 0 && (rem ^ rhs) < 0 {
            rem += rhs;
          }
          let c = build.const_int_64(rem);
          substitute(function, unsafe { &mut *inst_ptr }, c);
        }
      }
    }
    IrCmd::CheckDivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        // guard is satisfied, eliminate it
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(2)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::CheckCmpInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if compare_i64_i64_ir_condition(lhs, rhs, condition_op(read(2))) {
          kill_ir_function_ir_inst(function, unsafe { &mut *inst_ptr });
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(3)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      }
    }
    IrCmd::BitandInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = build.const_int_64(op1 & op2);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let c = build.const_int_64(0);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int64_op(read(0)) == -1 {
        let repl = read(1);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let c = build.const_int_64(0);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int64_op(read(1)) == -1 {
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::BitxorInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = build.const_int_64(op1 ^ op2);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let repl = read(1);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      }
    }
    IrCmd::BitorInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = build.const_int_64(op1 | op2);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let repl = read(1);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      } else if is_const(0) && function.int64_op(read(0)) == -1 {
        let c = build.const_int_64(-1);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let repl = read(0);
        substitute(function, unsafe { &mut *inst_ptr }, repl);
      } else if is_const(1) && function.int64_op(read(1)) == -1 {
        let c = build.const_int_64(-1);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitnotInt64 => {
      if is_const(0) {
        let op1 = function.int64_op(read(0));
        let c = build.const_int_64(!op1);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitlshiftInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let i = function.int64_op(read(1));
        let result = if (-63..=63).contains(&i) {
          (if i < 0 {
            n >> ((-i) as u32)
          } else {
            n << (i as u32)
          }) as i64
        } else {
          0
        };
        let c = build.const_int_64(result);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitrshiftInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let i = function.int64_op(read(1));
        let result = if (-63..=63).contains(&i) {
          (if i < 0 {
            n << ((-i) as u32)
          } else {
            n >> (i as u32)
          }) as i64
        } else {
          0
        };
        let c = build.const_int_64(result);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitarshiftInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0));
        let i = function.int64_op(read(1));
        let result = if (-63..=63).contains(&i) {
          if i < 0 {
            ((n as u64) << ((-i) as u32)) as i64
          } else {
            n >> (i as u32) // arithmetic shift for signed i64
          }
        } else if i < -63 {
          0
        } else if n < 0 {
          -1
        } else {
          0
        };
        let c = build.const_int_64(result);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitlrotateInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
        let r = if s != 0 { n.rotate_left(s) } else { n };
        let c = build.const_int_64(r as i64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitrrotateInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
        let r = if s != 0 { n.rotate_right(s) } else { n };
        let c = build.const_int_64(r as i64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitcountlzInt64 => {
      if is_const(0) {
        let n = function.int64_op(read(0)) as u64;
        let c = build.const_int_64(countlz_u64(n) as i64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitcountrzInt64 => {
      if is_const(0) {
        let n = function.int64_op(read(0)) as u64;
        let c = build.const_int_64(countrz_u64(n) as i64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::ByteswapInt64 => {
      if is_const(0) {
        let a = function.int64_op(read(0)) as u64;
        let result = byteswap(a);
        let c = build.const_int_64(result as i64);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitandUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = build.const_int((op1 & op2) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let c = build.const_int(0);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let op = read(1);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let c = build.const_int(0);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitxorUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = build.const_int((op1 ^ op2) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let op = read(1);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let r = make_inst(IrCmd::BitnotUint, &[read(1)]);
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let r = make_inst(IrCmd::BitnotUint, &[read(0)]);
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      }
    }
    IrCmd::BitorUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = build.const_int((op1 | op2) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let op = read(1);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let c = build.const_int(-1);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let c = build.const_int(-1);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitnotUint => {
      if is_const(0) {
        let c = build.const_int(!(function.int_op(read(0)) as u32) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitlshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1));
        let c = build.const_int((op1 << ((op2 & 31) as u32)) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitrshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1));
        let c = build.const_int((op1 >> ((op2 & 31) as u32)) as i32);
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitarshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0));
        let op2 = function.int_op(read(1));
        // signed arithmetic right shift
        let c = build.const_int(op1 >> ((op2 & 31) as u32));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitlrotateUint => {
      if is_const(0) && is_const(1) {
        let c = build.const_int(lrotate(
          function.int_op(read(0)) as u32,
          function.int_op(read(1)),
        ));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitrrotateUint => {
      if is_const(0) && is_const(1) {
        let c = build.const_int(rrotate(
          function.int_op(read(0)) as u32,
          function.int_op(read(1)),
        ));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint(function, block, unsafe { &mut *inst_ptr }, op);
      }
    }
    IrCmd::BitcountlzUint => {
      if is_const(0) {
        let c = build.const_int(countlz_u32(function.int_op(read(0)) as u32));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::BitcountrzUint => {
      if is_const(0) {
        let c = build.const_int(countrz_u32(function.int_op(read(0)) as u32));
        substitute(function, unsafe { &mut *inst_ptr }, c);
      }
    }
    IrCmd::CheckBufferLen => {
      if is_const(1) && is_const(4) {
        // If base offset and base offset source double value are both constants, we can get rid of that check or fallback
        if (function.int_op(read(1)) as f64) == function.double_op(read(4)) {
          let u = build.undef();
          // This disables equality check at runtime
          replace_ir_function_ir_op_ir_op(function, get_op_mut(unsafe { &mut *inst_ptr }, 4), u);
        } else {
          let r = make_inst(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
        }
      } else if read(1).kind() == IrOpKind::Inst && is_const(4) {
        // If only the base offset source double value is a constant, it means we couldn't constant-fold NUM_TO_INT
        let e_op = read(4);
        let inner = function.inst_op(read(1));
        let ok = inner.cmd == IrCmd::NumToInt
          && inner.ops.as_slice().first().copied().unwrap_or_default() == e_op;
        CODEGEN_ASSERT!(ok);

        let r = make_inst(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
        replace_ir_function_ir_block_u32_ir_inst(function, block, index, r);
      }
    }
    _ => {}
  }
}
