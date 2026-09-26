use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{luai_numidiv::luai_numidiv, luai_nummod::luai_nummod},
};

// IrUtils.cpp: `constexpr double kDoubleMaxExactInteger = 9007199254740992.0;`
use crate::enums::ir_condition::IrCondition;
use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    compare_ir_utils::{compare_f64_f64_ir_condition, compare_int},
    condition_op::condition_op,
    countlz_bit_utils::{countlz_u32, countlz_u64},
    countrz_bit_utils::{countrz_u32, countrz_u64},
    kill_ir_utils::kill_ir_function_ir_inst_at,
    lrotate::lrotate,
    replace_ir_utils::{
      replace_ir_function_ir_block_u32_ir_inst, replace_ir_function_ir_op_ir_op_at,
    },
    rrotate::rrotate,
    substitute::substitute_at,
    substitute_with_truncated_uint::substitute_with_truncated_uint_at,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_builder::ConstantMap, ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};
const K_DOUBLE_MAX_EXACT_INTEGER: f64 = 9007199254740992.0;

const LUA_TNIL: u8 = LuaType::Nil as u8;
const LUA_TBOOLEAN: u8 = LuaType::Boolean as u8;
const LUA_TNUMBER: u8 = LuaType::Number as u8;
const LUA_TINTEGER: u8 = LuaType::Integer as u8;

/// cpp 上游指令操作数上限为 6（OP_A..OP_F）
const MAX_OPERANDS: usize = 6;

/// 操作数若为常量则返回其值拷贝（IrOp 为 Copy）
#[inline]
fn const_operand(operands: &[IrOp; MAX_OPERANDS], idx: usize) -> Option<IrOp> {
  (operands[idx].kind() == IrOpKind::Constant).then_some(operands[idx])
}

pub fn fold_constants(
  function: &mut IrFunction,
  map: &mut ConstantMap,
  block_idx: u32,
  index: u32,
) {
  // cpp 上游指令操作数上限为 6（OP_A..OP_F）；开头一次性快照到栈上，
  // 后续各分支的只读取值不再逐次解引用指令。写回全部走 _at 索引化变体，
  // 这里只借 &function 读、不留裸指针，故无结构性 unsafe。
  let mut operands = [IrOp::default(); MAX_OPERANDS];
  let (cmd, operand_count) = {
    let inst = &function.instructions[index as usize];
    let s = inst.ops.as_slice();
    let n = s.len().min(MAX_OPERANDS);
    operands[..n].copy_from_slice(&s[..n]);
    (inst.cmd, n)
  };

  // OP_A(inst)..OP_F(inst)：读取第 n 个操作数（拷贝；IrOp 是 Copy）
  let read = move |idx: usize| -> IrOp {
    if idx < operand_count {
      operands[idx]
    } else {
      IrOp::default()
    }
  };
  let is_const = move |idx: usize| -> bool { read(idx).kind() == IrOpKind::Constant };

  macro_rules! fold_two {
    ($getter:ident, $const_fn:ident, $op:expr) => {
      if let (Some(a), Some(b)) = (const_operand(&operands, 0), const_operand(&operands, 1)) {
        let va = function.$getter(a);
        let vb = function.$getter(b);
        let c = function.$const_fn(map, ($op)(va, vb));
        substitute_at(function, index, c);
      }
    };
  }

  macro_rules! fold_one {
    ($getter:ident, $const_fn:ident, $op:expr) => {
      if let Some(a) = const_operand(&operands, 0) {
        let va = function.$getter(a);
        let c = function.$const_fn(map, ($op)(va));
        substitute_at(function, index, c);
      }
    };
  }

  macro_rules! fold_two_float {
    ($op:expr) => {
      if let (Some(a), Some(b)) = (const_operand(&operands, 0), const_operand(&operands, 1)) {
        let va = function.double_op(a) as f32;
        let vb = function.double_op(b) as f32;
        let c = function.const_double(map, ($op)(va, vb) as f64);
        substitute_at(function, index, c);
      }
    };
  }

  macro_rules! fold_one_float {
    ($op:expr) => {
      if let Some(a) = const_operand(&operands, 0) {
        let va = function.double_op(a) as f32;
        let c = function.const_double(map, ($op)(va) as f64);
        substitute_at(function, index, c);
      }
    };
  }

  macro_rules! fold_jump_cmp {
    ($cond:expr) => {
      if is_const(0) && is_const(1) {
        let r = IrInst::ir_inst_new(IrCmd::JUMP, &[if $cond { read(3) } else { read(4) }]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
      }
    };
  }

  macro_rules! fold_check_cmp {
    ($cond:expr) => {
      if is_const(0) && is_const(1) {
        if $cond {
          kill_ir_function_ir_inst_at(function, index);
        } else {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(3)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        }
      }
    };
  }

  match cmd {
    IrCmd::AddInt => fold_two!(int_op, const_int, i32::wrapping_add),
    IrCmd::SubInt => fold_two!(int_op, const_int, i32::wrapping_sub),
    IrCmd::Sexti8Int => fold_one!(int_op, const_int, |v| v as i8 as i32),
    IrCmd::Sexti16Int => fold_one!(int_op, const_int, |v| v as i16 as i32),
    IrCmd::AddNum => fold_two!(double_op, const_double, |a, b| a + b),
    IrCmd::SubNum => fold_two!(double_op, const_double, |a, b| a - b),
    IrCmd::MulNum => fold_two!(double_op, const_double, |a, b| a * b),
    IrCmd::DivNum => fold_two!(double_op, const_double, |a, b| a / b),
    IrCmd::IdivNum => fold_two!(double_op, const_double, luai_numidiv),
    IrCmd::ModNum => fold_two!(double_op, const_double, luai_nummod),
    IrCmd::MinNum => fold_two!(double_op, const_double, |a, b| if a < b { a } else { b }),
    IrCmd::MaxNum => fold_two!(double_op, const_double, |a, b| if a > b { a } else { b }),
    IrCmd::UnmNum => fold_one!(double_op, const_double, |v: f64| -v),
    IrCmd::FloorNum => fold_one!(double_op, const_double, f64::floor),
    IrCmd::CeilNum => fold_one!(double_op, const_double, f64::ceil),
    IrCmd::RoundNum => fold_one!(double_op, const_double, f64::round),
    IrCmd::SqrtNum => fold_one!(double_op, const_double, f64::sqrt),
    IrCmd::AbsNum => fold_one!(double_op, const_double, f64::abs),
    IrCmd::SignNum => fold_one!(double_op, const_double, |v: f64| {
      if v > 0.0 {
        1.0
      } else if v < 0.0 {
        -1.0
      } else {
        0.0
      }
    }),
    IrCmd::AddFloat => fold_two_float!(|a, b| a + b),
    IrCmd::SubFloat => fold_two_float!(|a, b| a - b),
    IrCmd::MulFloat => fold_two_float!(|a, b| a * b),
    IrCmd::DivFloat => fold_two_float!(|a, b| a / b),
    IrCmd::MinFloat => fold_two_float!(|a, b| if a < b { a } else { b }),
    IrCmd::MaxFloat => fold_two_float!(|a, b| if a > b { a } else { b }),
    IrCmd::UnmFloat => fold_one_float!(|v: f32| -v),
    IrCmd::FloorFloat => fold_one_float!(f32::floor),
    IrCmd::CeilFloat => fold_one_float!(f32::ceil),
    IrCmd::SqrtFloat => fold_one_float!(f32::sqrt),
    IrCmd::AbsFloat => fold_one_float!(f32::abs),
    IrCmd::SignFloat => fold_one_float!(|v: f32| {
      if v > 0.0 {
        1.0
      } else if v < 0.0 {
        -1.0
      } else {
        0.0
      }
    }),
    IrCmd::SelectNum => {
      if is_const(2) && is_const(3) {
        let c = function.double_op(read(2));
        let d = function.double_op(read(3));
        let repl = if c == d { read(1) } else { read(0) };
        substitute_at(function, index, repl);
      } else if read(0) == read(1) {
        // 两值相同则无需担心条件检查
        let repl = read(0);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::SelectVec => {
      if read(0) == read(1) {
        let repl = read(0);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::SelectIfTruthy => {
      if read(1) == read(2) {
        let repl = read(1);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::NotAny => {
      if is_const(0) {
        let a = function.tag_op(read(0));

        if a == LUA_TNIL {
          let c = function.const_int(map, 1);
          substitute_at(function, index, c);
        } else if a != LUA_TBOOLEAN {
          let c = function.const_int(map, 0);
          substitute_at(function, index, c);
        } else if is_const(1) {
          let c = function.const_int(map, if function.int_op(read(1)) == 1 { 0 } else { 1 });
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::CmpInt => fold_two!(
      int_op,
      const_int,
      |a, b| compare_int(a, b, condition_op(read(2))) as i32
    ),
    IrCmd::CmpInt64 => fold_two!(int64_op, const_int, |a, b| compare_int(
      a,
      b,
      condition_op(read(2))
    ) as i32),
    IrCmd::CmpTag => {
      if is_const(0) && is_const(1) {
        let cond = condition_op(read(2));
        CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));

        let same = function.tag_op(read(0)) == function.tag_op(read(1));
        let val = if cond == IrCondition::Equal {
          if same { 1 } else { 0 }
        } else if !same {
          1
        } else {
          0
        };
        let c = function.const_int(map, val);
        substitute_at(function, index, c);
      }
    }
    IrCmd::CmpSplitTvalue => {
      CODEGEN_ASSERT!(is_const(1));

      let cond = condition_op(read(4));
      CODEGEN_ASSERT!(matches!(cond, IrCondition::Equal | IrCondition::NotEqual));

      if cond == IrCondition::Equal {
        if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
          let c = function.const_int(map, 0);
          substitute_at(function, index, c);
        } else if is_const(2) && is_const(3) {
          // tag 为常量说明前一个条件失败，因为 tag 相同
          let known_same_tag = is_const(0);
          let tag_b = function.tag_op(read(1));

          let same_value = if tag_b == LUA_TBOOLEAN {
            compare_int(
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
            compare_int(lhs, rhs, IrCondition::Equal)
          } else {
            CODEGEN_ASSERT!(false, "unsupported type");
            false
          };

          if known_same_tag && same_value {
            let c = function.const_int(map, 1);
            substitute_at(function, index, c);
          } else if same_value {
            let r = IrInst::ir_inst_new(IrCmd::CmpTag, &[read(0), read(1), read(4)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
          } else {
            let c = function.const_int(map, 0);
            substitute_at(function, index, c);
          }
        }
      } else {
        if is_const(0) && function.tag_op(read(0)) != function.tag_op(read(1)) {
          let c = function.const_int(map, 1);
          substitute_at(function, index, c);
        } else if is_const(2) && is_const(3) {
          let known_same_tag = is_const(0);
          let tag_b = function.tag_op(read(1));

          let different_value = if tag_b == LUA_TBOOLEAN {
            compare_int(
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
            compare_int(lhs, rhs, IrCondition::NotEqual)
          } else {
            CODEGEN_ASSERT!(false, "unsupported type");
            false
          };

          if different_value {
            let c = function.const_int(map, 1);
            substitute_at(function, index, c);
          } else if known_same_tag {
            let c = function.const_int(map, 0);
            substitute_at(function, index, c);
          } else {
            let r = IrInst::ir_inst_new(IrCmd::CmpTag, &[read(0), read(1), read(4)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
          }
        }
      }
    }
    IrCmd::JumpEqTag => {
      if is_const(0) && is_const(1) {
        let same = function.tag_op(read(0)) == function.tag_op(read(1));
        let r = IrInst::ir_inst_new(IrCmd::JUMP, &[if same { read(2) } else { read(3) }]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
      }
    }
    IrCmd::JumpCmpInt => fold_jump_cmp!(compare_int(
      function.int_op(read(0)),
      function.int_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::JumpCmpInt64 => fold_jump_cmp!(compare_int(
      function.int64_op(read(0)),
      function.int64_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::JumpCmpNum => fold_jump_cmp!(compare_f64_f64_ir_condition(
      function.double_op(read(0)),
      function.double_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::JumpCmpFloat => fold_jump_cmp!(compare_f64_f64_ir_condition(
      function.double_op(read(0)) as f32 as f64,
      function.double_op(read(1)) as f32 as f64,
      condition_op(read(2)),
    )),
    IrCmd::TryNumToIndex => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // 先做范围检查，避免把不可表示的值转到目标类型的未定义行为
        if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
          let arr_index = value as i32;

          if arr_index as f64 == value {
            let c = function.const_int(map, arr_index);
            substitute_at(function, index, c);
          } else {
            let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(1)]);
            replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
          }
        } else {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(1)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        }
      }
    }
    IrCmd::IntToNum => fold_one!(int_op, const_double, |v| v as f64),
    IrCmd::Int64ToNum => fold_one!(int64_op, const_double, |v| v as f64),
    IrCmd::UintToNum => fold_one!(int_op, const_double, |v| (v as u32) as f64),
    IrCmd::UintToFloat => fold_one!(int_op, const_double, |v| ((v as u32) as f32) as f64),
    IrCmd::NumToInt => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // 与 luai_num2int 的范围检查一致
        if value >= i32::MIN as f64 && value <= i32::MAX as f64 {
          let c = function.const_int(map, value as i32);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::NumToUint => {
      if is_const(0) {
        let value = function.double_op(read(0));

        // 与 luai_num2unsigned 的范围检查一致
        if (-K_DOUBLE_MAX_EXACT_INTEGER..=K_DOUBLE_MAX_EXACT_INTEGER).contains(&value) {
          let c = function.const_int(map, (value as i64 as u32) as i32);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::NumToInt64 => {
      if is_const(0) {
        let value = function.double_op(read(0));

        if value >= i64::MIN as f64 && value < i64::MAX as f64 {
          let c = function.const_int_64(map, value as i64);
          substitute_at(function, index, c);
        }
      }
    }
    // constant 的 float -> double 是空操作
    IrCmd::FloatToNum => fold_one!(double_op, const_double, |v| v),
    // constant 的 double -> float 只需降低精度
    IrCmd::NumToFloat => fold_one!(double_op, const_double, |v| (v as f32) as f64),
    IrCmd::TruncateUint => {
      // 截断 constant 整数是空操作，constant 整数本身只存 32 位
      if is_const(0) {
        let repl = read(0);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::CheckTag => {
      if is_const(0) && is_const(1) {
        if function.tag_op(read(0)) == function.tag_op(read(1)) {
          kill_ir_function_ir_inst_at(function, index);
        } else {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        }
      }
    }
    IrCmd::CheckTruthy => {
      if is_const(0) {
        if function.tag_op(read(0)) == LUA_TNIL {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        } else if function.tag_op(read(0)) == LUA_TBOOLEAN {
          if is_const(1) {
            if function.int_op(read(1)) == 0 {
              let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(2)]); // Shows a conflict in assumptions on this path
              replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
            } else {
              kill_ir_function_ir_inst_at(function, index);
            }
          }
        } else {
          kill_ir_function_ir_inst_at(function, index);
        }
      }
    }
    IrCmd::CheckCmpNum => fold_check_cmp!(compare_f64_f64_ir_condition(
      function.double_op(read(0)),
      function.double_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::CheckCmpInt => fold_check_cmp!(compare_int(
      function.int_op(read(0)),
      function.int_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::AddInt64 => fold_two!(int64_op, const_int_64, i64::wrapping_add),
    IrCmd::SubInt64 => fold_two!(int64_op, const_int_64, i64::wrapping_sub),
    IrCmd::MulInt64 => fold_two!(int64_op, const_int_64, i64::wrapping_mul),
    IrCmd::DivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let c = function.const_int_64(map, lhs / rhs);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::IdivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let mut q = lhs / rhs;
          // 向下取整除法：符号相异且有余数时需调整
          if (lhs ^ rhs) < 0 && q.wrapping_mul(rhs) != lhs {
            q -= 1;
          }
          let c = function.const_int_64(map, q);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::UdivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0)) as u64;
        let rhs = function.int64_op(read(1)) as u64;
        if let Some(q) = lhs.checked_div(rhs) {
          let c = function.const_int_64(map, q as i64);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::RemInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let c = function.const_int_64(map, lhs % rhs);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::UremInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0)) as u64;
        let rhs = function.int64_op(read(1)) as u64;
        if rhs != 0 {
          let c = function.const_int_64(map, (lhs % rhs) as i64);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::ModInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          let mut rem = lhs % rhs;
          // 向下取整模运算：余数 != 0 且符号相异时需调整
          if rem != 0 && (rem ^ rhs) < 0 {
            rem += rhs;
          }
          let c = function.const_int_64(map, rem);
          substitute_at(function, index, c);
        }
      }
    }
    IrCmd::CheckDivInt64 => {
      if is_const(0) && is_const(1) {
        let lhs = function.int64_op(read(0));
        let rhs = function.int64_op(read(1));
        if rhs != 0 && !(lhs == i64::MIN && rhs == -1) {
          kill_ir_function_ir_inst_at(function, index);
        // guard 已满足，消除它
        } else {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(2)]);
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        }
      }
    }
    IrCmd::CheckCmpInt64 => fold_check_cmp!(compare_int(
      function.int64_op(read(0)),
      function.int64_op(read(1)),
      condition_op(read(2)),
    )),
    IrCmd::BitandInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = function.const_int_64(map, op1 & op2);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let c = function.const_int_64(map, 0);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int64_op(read(0)) == -1 {
        let repl = read(1);
        substitute_at(function, index, repl);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let c = function.const_int_64(map, 0);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int64_op(read(1)) == -1 {
        let repl = read(0);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::BitxorInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = function.const_int_64(map, op1 ^ op2);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let repl = read(1);
        substitute_at(function, index, repl);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let repl = read(0);
        substitute_at(function, index, repl);
      }
    }
    IrCmd::BitorInt64 => {
      if is_const(0) && is_const(1) {
        let op1 = function.int64_op(read(0));
        let op2 = function.int64_op(read(1));
        let c = function.const_int_64(map, op1 | op2);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int64_op(read(0)) == 0 {
        let repl = read(1);
        substitute_at(function, index, repl);
      } else if is_const(0) && function.int64_op(read(0)) == -1 {
        let c = function.const_int_64(map, -1);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int64_op(read(1)) == 0 {
        let repl = read(0);
        substitute_at(function, index, repl);
      } else if is_const(1) && function.int64_op(read(1)) == -1 {
        let c = function.const_int_64(map, -1);
        substitute_at(function, index, c);
      }
    }
    IrCmd::BitnotInt64 => fold_one!(int64_op, const_int_64, |n: i64| !n),
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
        let c = function.const_int_64(map, result);
        substitute_at(function, index, c);
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
        let c = function.const_int_64(map, result);
        substitute_at(function, index, c);
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
        let c = function.const_int_64(map, result);
        substitute_at(function, index, c);
      }
    }
    IrCmd::BitlrotateInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
        let r = if s != 0 { n.rotate_left(s) } else { n };
        let c = function.const_int_64(map, r as i64);
        substitute_at(function, index, c);
      }
    }
    IrCmd::BitrrotateInt64 => {
      if is_const(0) && is_const(1) {
        let n = function.int64_op(read(0)) as u64;
        let s = ((function.int64_op(read(1)) as u64) % 64) as u32;
        let r = if s != 0 { n.rotate_right(s) } else { n };
        let c = function.const_int_64(map, r as i64);
        substitute_at(function, index, c);
      }
    }
    IrCmd::BitcountlzInt64 => fold_one!(int64_op, const_int_64, |n: i64| {
      countlz_u64(n as u64) as i64
    }),
    IrCmd::BitcountrzInt64 => fold_one!(int64_op, const_int_64, |n: i64| {
      countrz_u64(n as u64) as i64
    }),
    IrCmd::ByteswapInt64 => fold_one!(int64_op, const_int_64, |n: i64| {
      (n as u64).swap_bytes() as i64
    }),
    IrCmd::BitandUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = function.const_int(map, (op1 & op2) as i32);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let c = function.const_int(map, 0);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let op = read(1);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let c = function.const_int(map, 0);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitxorUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = function.const_int(map, (op1 ^ op2) as i32);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let op = read(1);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let r = IrInst::ir_inst_new(IrCmd::BitnotUint, &[read(1)]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let r = IrInst::ir_inst_new(IrCmd::BitnotUint, &[read(0)]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
      }
    }
    IrCmd::BitorUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1)) as u32;
        let c = function.const_int(map, (op1 | op2) as i32);
        substitute_at(function, index, c);
      } else if is_const(0) && function.int_op(read(0)) == 0 {
        let op = read(1);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      } else if is_const(0) && function.int_op(read(0)) == -1 {
        let c = function.const_int(map, -1);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      } else if is_const(1) && function.int_op(read(1)) == -1 {
        let c = function.const_int(map, -1);
        substitute_at(function, index, c);
      }
    }
    IrCmd::BitnotUint => fold_one!(int_op, const_int, |v: i32| !(v as u32) as i32),
    IrCmd::BitlshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1));
        let c = function.const_int(map, (op1 << ((op2 & 31) as u32)) as i32);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitrshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0)) as u32;
        let op2 = function.int_op(read(1));
        let c = function.const_int(map, (op1 >> ((op2 & 31) as u32)) as i32);
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitarshiftUint => {
      if is_const(0) && is_const(1) {
        let op1 = function.int_op(read(0));
        let op2 = function.int_op(read(1));
        // 有符号算术右移
        let c = function.const_int(map, op1 >> ((op2 & 31) as u32));
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitlrotateUint => {
      if is_const(0) && is_const(1) {
        let c = function.const_int(
          map,
          lrotate(function.int_op(read(0)) as u32, function.int_op(read(1))),
        );
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitrrotateUint => {
      if is_const(0) && is_const(1) {
        let c = function.const_int(
          map,
          rrotate(function.int_op(read(0)) as u32, function.int_op(read(1))),
        );
        substitute_at(function, index, c);
      } else if is_const(1) && function.int_op(read(1)) == 0 {
        let op = read(0);
        substitute_with_truncated_uint_at(function, block_idx, index, op);
      }
    }
    IrCmd::BitcountlzUint => fold_one!(int_op, const_int, |v: i32| countlz_u32(v as u32)),
    IrCmd::BitcountrzUint => fold_one!(int_op, const_int, |v: i32| countrz_u32(v as u32)),
    IrCmd::CheckBufferLen => {
      if is_const(1) && is_const(4) {
        // 若 base offset 与其源的 double 值都是常量，可去掉该检查或走 fallback
        if (function.int_op(read(1)) as f64) == function.double_op(read(4)) {
          let u = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
          // 这会让运行期的等值检查失效
          replace_ir_function_ir_op_ir_op_at(function, index, 4, u);
        } else {
          let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
        }
      } else if read(1).kind() == IrOpKind::Inst && is_const(4) {
        // 若只有 base offset 源的 double 值是常量，说明 NUM_TO_INT 未能常量折叠
        let e_op = read(4);
        let inner = function.inst_op(read(1));
        let ok = inner.cmd == IrCmd::NumToInt
          && inner.ops.as_slice().first().copied().unwrap_or_default() == e_op;
        CODEGEN_ASSERT!(ok);

        let r = IrInst::ir_inst_new(IrCmd::JUMP, &[read(5)]); // Shows a conflict in assumptions on this path
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, r);
      }
    }
    _ => {}
  }
}
