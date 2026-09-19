use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_binary::AstExprBinaryOp, ast_name_table::AstNameTable};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{constants_equal::constants_equal, cvar::cvar},
  records::constant::Constant,
};

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: u32 = 4096;

/// 双操作数均为 Number 常量时取出 (左, 右) 浮点值
#[inline]
fn num2(la: &Constant, ra: &Constant) -> Option<(f64, f64)> {
  match (la, ra) {
    (Constant::Number(a), Constant::Number(b)) => Some((*a, *b)),
    _ => None,
  }
}

/// 双操作数均为 Vector 常量时取出 (左, 右) 分量组
#[inline]
fn vec2(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  match (la, ra) {
    (Constant::Vector(a), Constant::Vector(b)) => Some((*a, *b)),
    _ => None,
  }
}

/// Number 标量 × Vector：标量按 C++ 语义广播到 4 分量
#[inline]
fn num_vec(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  match (la, ra) {
    (Constant::Number(a), Constant::Vector(b)) => Some(([*a as f32; 4], *b)),
    _ => None,
  }
}

/// Vector × Number 标量
#[inline]
fn vec_num(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  match (la, ra) {
    (Constant::Vector(a), Constant::Number(b)) => Some((*a, [*b as f32; 4])),
    _ => None,
  }
}

/// 按分量折叠向量二元运算（对齐 C++ foldBinary 的 Vectorf 分支）：
/// `op` 逐分量运算；`w` 为第 4 分量结果；`had_w` 表示任一输入的第 4 分量非零。
/// 仅当结果第 4 分量非零或有输入携带 w 分量时才折叠。
fn fold_vector(
  result: &mut Constant,
  la: [f32; 4],
  ra: [f32; 4],
  op: impl Fn(f32, f32) -> f32,
  w: f32,
  had_w: bool,
) {
  if w == 0.0 || had_w {
    *result = Constant::Vector([op(la[0], ra[0]), op(la[1], ra[1]), op(la[2], ra[2]), w]);
  }
}

/// 向量逐分量除法后取整（FloorDiv 用）
fn floor_div(a: f32, b: f32) -> f32 {
  (a / b).floor()
}

/// Number/Vector 组合的公共折叠路径：vec×vec、num×vec、vec×num 三种形态
/// 统一走 [`fold_vector`]；`had_w` 仅看原始 Vector 操作数的 w 分量
/// （标量广播侧的 [3] 是标量本身，不参与判定，逐一对齐 C++ 分支语义）
fn fold_vector_combo(
  result: &mut Constant,
  la: &Constant,
  ra: &Constant,
  op: impl Fn(f32, f32) -> f32,
) {
  if let Some((a, b)) = vec2(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(result, a, b, op, w, a[3] != 0.0 || b[3] != 0.0);
  } else if let Some((a, b)) = num_vec(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(result, a, b, op, w, b[3] != 0.0);
  } else if let Some((a, b)) = vec_num(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(result, a, b, op, w, a[3] != 0.0);
  }
}

/// C++ `foldBinary`：折叠二元运算，未折叠时返回 `Constant::Unknown`
/// （cpp 侧表现为不写出参 `result`）
pub fn fold_binary(
  op: AstExprBinaryOp,
  la: &Constant,
  ra: &Constant,
  string_table: &mut AstNameTable,
) -> Constant {
  let mut result = cvar();

  match op {
    AstExprBinaryOp::Add => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a + b);
      } else if let Some((a, b)) = vec2(la, ra) {
        // 加减法的 w 分量恒参与折叠（C++ had_w = true）
        fold_vector(&mut result, a, b, |x, y| x + y, a[3] + b[3], true);
      }
    }
    AstExprBinaryOp::Sub => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a - b);
      } else if let Some((a, b)) = vec2(la, ra) {
        fold_vector(&mut result, a, b, |x, y| x - y, a[3] - b[3], true);
      }
    }
    AstExprBinaryOp::Mul => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a * b);
      } else {
        fold_vector_combo(&mut result, la, ra, |x, y| x * y);
      }
    }
    AstExprBinaryOp::Div => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a / b);
      } else {
        fold_vector_combo(&mut result, la, ra, |x, y| x / y);
      }
    }
    AstExprBinaryOp::FloorDiv => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number((a / b).floor());
      } else {
        fold_vector_combo(&mut result, la, ra, floor_div);
      }
    }
    AstExprBinaryOp::Mod => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a - (a / b).floor() * b);
      }
    }
    AstExprBinaryOp::Pow => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(a.powf(b));
      }
    }
    AstExprBinaryOp::Concat => {
      if let (Constant::Str(l), Constant::Str(r)) = (la, ra) {
        let total = l.len + r.len;
        if total <= K_CONSTANT_FOLD_STRING_LIMIT {
          // 任一侧为空时直接沿用另一侧指针，避免拼接
          let ptr = if l.len == 0 {
            r.ptr
          } else if r.len == 0 {
            l.ptr
          } else {
            let mut tmp = Vec::with_capacity(total as usize);
            tmp.extend_from_slice(la.get_string_bytes());
            tmp.extend_from_slice(ra.get_string_bytes());
            string_table.get_or_add_slice(&tmp).value
          };
          result = Constant::string(ptr, total);
        }
      }
    }
    AstExprBinaryOp::CompareNe => {
      if !la.is_unknown() && !ra.is_unknown() {
        result = Constant::Boolean(!constants_equal(la, ra));
      }
    }
    AstExprBinaryOp::CompareEq => {
      if !la.is_unknown() && !ra.is_unknown() {
        result = Constant::Boolean(constants_equal(la, ra));
      }
    }
    AstExprBinaryOp::CompareLt => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Boolean(a < b);
      }
    }
    AstExprBinaryOp::CompareLe => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Boolean(a <= b);
      }
    }
    AstExprBinaryOp::CompareGt => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Boolean(a > b);
      }
    }
    AstExprBinaryOp::CompareGe => {
      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Boolean(a >= b);
      }
    }
    AstExprBinaryOp::And => {
      if !la.is_unknown() {
        result = if la.is_truthful() { *ra } else { *la };
      }
    }
    AstExprBinaryOp::Or => {
      if !la.is_unknown() {
        result = if la.is_truthful() { *la } else { *ra };
      }
    }
    // C++ default：仅哨兵 OpCount 会到达
    AstExprBinaryOp::OpCount => {
      LUAU_ASSERT!(false);
    }
  }

  result
}
