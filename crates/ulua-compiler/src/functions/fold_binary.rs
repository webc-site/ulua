use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_binary::AstExprBinaryOp, ast_name_table::AstNameTable};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_constant_folding::Type, functions::constants_equal::constants_equal,
  records::constant::Constant,
};

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: u32 = 4096;

/// 双操作数均为 Number 常量时取出 (左, 右) 浮点值；union 读取由类型判别守卫。
/// 刻意不用 then_some：急切求值会在未命中时读取 union 非激活字段（UB）
#[inline]
fn num2(la: &Constant, ra: &Constant) -> Option<(f64, f64)> {
  if la.r#type == Type::Number && ra.r#type == Type::Number {
    Some(unsafe { (la.data.value_number, ra.data.value_number) })
  } else {
    None
  }
}

/// 双操作数均为 Vector 常量时取出 (左, 右) 分量组
#[inline]
fn vec2(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  if la.r#type == Type::Vector && ra.r#type == Type::Vector {
    Some(unsafe { (la.data.value_vector, ra.data.value_vector) })
  } else {
    None
  }
}

/// Number 标量 × Vector：标量按 C++ 语义广播到 4 分量
#[inline]
fn num_vec(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  if la.r#type == Type::Number && ra.r#type == Type::Vector {
    Some(unsafe { ([la.data.value_number as f32; 4], ra.data.value_vector) })
  } else {
    None
  }
}

/// Vector × Number 标量
#[inline]
fn vec_num(la: &Constant, ra: &Constant) -> Option<([f32; 4], [f32; 4])> {
  if la.r#type == Type::Vector && ra.r#type == Type::Number {
    Some(unsafe { (la.data.value_vector, [ra.data.value_number as f32; 4]) })
  } else {
    None
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
    result.r#type = Type::Vector;
    result.data.value_vector = [op(la[0], ra[0]), op(la[1], ra[1]), op(la[2], ra[2]), w];
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

pub fn fold_binary(
  result: &mut Constant,
  op: AstExprBinaryOp,
  la: &Constant,
  ra: &Constant,
  string_table: &mut AstNameTable,
) {
  match op {
    AstExprBinaryOp::Add => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a + b;
      } else if let Some((a, b)) = vec2(la, ra) {
        // 加减法的 w 分量恒参与折叠（C++ had_w = true）
        fold_vector(result, a, b, |x, y| x + y, a[3] + b[3], true);
      }
    }
    AstExprBinaryOp::Sub => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a - b;
      } else if let Some((a, b)) = vec2(la, ra) {
        fold_vector(result, a, b, |x, y| x - y, a[3] - b[3], true);
      }
    }
    AstExprBinaryOp::Mul => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a * b;
      } else {
        fold_vector_combo(result, la, ra, |x, y| x * y);
      }
    }
    AstExprBinaryOp::Div => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a / b;
      } else {
        fold_vector_combo(result, la, ra, |x, y| x / y);
      }
    }
    AstExprBinaryOp::FloorDiv => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = (a / b).floor();
      } else {
        fold_vector_combo(result, la, ra, floor_div);
      }
    }
    AstExprBinaryOp::Mod => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a - (a / b).floor() * b;
      }
    }
    AstExprBinaryOp::Pow => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Number;
        result.data.value_number = a.powf(b);
      }
    }
    AstExprBinaryOp::Concat => {
      if la.r#type == Type::String
        && ra.r#type == Type::String
        && (la.string_length + ra.string_length) <= K_CONSTANT_FOLD_STRING_LIMIT
      {
        result.r#type = Type::String;
        result.string_length = la.string_length + ra.string_length;
        if la.string_length == 0 {
          unsafe {
            result.data.value_string = ra.data.value_string;
          }
        } else if ra.string_length == 0 {
          unsafe {
            result.data.value_string = la.data.value_string;
          }
        } else {
          let mut tmp = Vec::with_capacity(result.string_length as usize);
          tmp.extend_from_slice(la.get_string_bytes());
          tmp.extend_from_slice(ra.get_string_bytes());
          let name = string_table.get_or_add_slice(&tmp);
          result.data.value_string = name.value;
        }
      }
    }
    AstExprBinaryOp::CompareNe => {
      if la.r#type != Type::Unknown && ra.r#type != Type::Unknown {
        result.r#type = Type::Boolean;
        result.data.value_boolean = !constants_equal(la, ra);
      }
    }
    AstExprBinaryOp::CompareEq => {
      if la.r#type != Type::Unknown && ra.r#type != Type::Unknown {
        result.r#type = Type::Boolean;
        result.data.value_boolean = constants_equal(la, ra);
      }
    }
    AstExprBinaryOp::CompareLt => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Boolean;
        result.data.value_boolean = a < b;
      }
    }
    AstExprBinaryOp::CompareLe => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Boolean;
        result.data.value_boolean = a <= b;
      }
    }
    AstExprBinaryOp::CompareGt => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Boolean;
        result.data.value_boolean = a > b;
      }
    }
    AstExprBinaryOp::CompareGe => {
      if let Some((a, b)) = num2(la, ra) {
        result.r#type = Type::Boolean;
        result.data.value_boolean = a >= b;
      }
    }
    AstExprBinaryOp::And => {
      if la.r#type != Type::Unknown {
        *result = if la.is_truthful() { *ra } else { *la };
      }
    }
    AstExprBinaryOp::Or => {
      if la.r#type != Type::Unknown {
        *result = if la.is_truthful() { *la } else { *ra };
      }
    }
    // C++ default：仅哨兵 OpCount 会到达
    AstExprBinaryOp::OpCount => {
      LUAU_ASSERT!(false);
    }
  }
}
