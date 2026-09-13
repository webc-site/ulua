use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_binary::AstExprBinaryOp, ast_name_table::AstNameTable};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_constant_folding::Type, functions::constants_equal::constants_equal,
  records::constant::Constant,
};

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: u32 = 4096;

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

pub fn fold_binary(
  result: &mut Constant,
  op: AstExprBinaryOp,
  la: &Constant,
  ra: &Constant,
  string_table: &mut AstNameTable,
) {
  match op {
    AstExprBinaryOp::Add => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number + ra.data.value_number;
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Vector {
        unsafe {
          let (la_v, ra_v) = (la.data.value_vector, ra.data.value_vector);
          fold_vector(result, la_v, ra_v, |a, b| a + b, la_v[3] + ra_v[3], true);
        }
      }
    }
    AstExprBinaryOp::Sub => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number - ra.data.value_number;
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Vector {
        unsafe {
          let (la_v, ra_v) = (la.data.value_vector, ra.data.value_vector);
          fold_vector(result, la_v, ra_v, |a, b| a - b, la_v[3] - ra_v[3], true);
        }
      }
    }
    AstExprBinaryOp::Mul => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number * ra.data.value_number;
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Vector {
        unsafe {
          let (la_v, ra_v) = (la.data.value_vector, ra.data.value_vector);
          fold_vector(
            result,
            la_v,
            ra_v,
            |a, b| a * b,
            la_v[3] * ra_v[3],
            la_v[3] != 0.0 || ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Number && ra.r#type == Type::Vector {
        unsafe {
          let ra_v = ra.data.value_vector;
          let n = la.data.value_number as f32;
          fold_vector(
            result,
            [n; 4],
            ra_v,
            |a, b| a * b,
            n * ra_v[3],
            ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Number {
        unsafe {
          let la_v = la.data.value_vector;
          let n = ra.data.value_number as f32;
          fold_vector(
            result,
            la_v,
            [n; 4],
            |a, b| a * b,
            la_v[3] * n,
            la_v[3] != 0.0,
          );
        }
      }
    }
    AstExprBinaryOp::Div => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number / ra.data.value_number;
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Vector {
        unsafe {
          let (la_v, ra_v) = (la.data.value_vector, ra.data.value_vector);
          fold_vector(
            result,
            la_v,
            ra_v,
            |a, b| a / b,
            la_v[3] / ra_v[3],
            la_v[3] != 0.0 || ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Number && ra.r#type == Type::Vector {
        unsafe {
          let ra_v = ra.data.value_vector;
          let n = la.data.value_number as f32;
          fold_vector(
            result,
            [n; 4],
            ra_v,
            |a, b| a / b,
            n / ra_v[3],
            ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Number {
        unsafe {
          let la_v = la.data.value_vector;
          let n = ra.data.value_number as f32;
          fold_vector(
            result,
            la_v,
            [n; 4],
            |a, b| a / b,
            la_v[3] / n,
            la_v[3] != 0.0,
          );
        }
      }
    }
    AstExprBinaryOp::FloorDiv => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = (la.data.value_number / ra.data.value_number).floor();
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Vector {
        unsafe {
          let (la_v, ra_v) = (la.data.value_vector, ra.data.value_vector);
          fold_vector(
            result,
            la_v,
            ra_v,
            floor_div,
            floor_div(la_v[3], ra_v[3]),
            la_v[3] != 0.0 || ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Number && ra.r#type == Type::Vector {
        unsafe {
          let ra_v = ra.data.value_vector;
          let n = la.data.value_number as f32;
          fold_vector(
            result,
            [n; 4],
            ra_v,
            floor_div,
            floor_div(n, ra_v[3]),
            ra_v[3] != 0.0,
          );
        }
      } else if la.r#type == Type::Vector && ra.r#type == Type::Number {
        unsafe {
          let la_v = la.data.value_vector;
          let n = ra.data.value_number as f32;
          fold_vector(
            result,
            la_v,
            [n; 4],
            floor_div,
            floor_div(la_v[3], n),
            la_v[3] != 0.0,
          );
        }
      }
    }
    AstExprBinaryOp::Mod => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number
            - (la.data.value_number / ra.data.value_number).floor() * ra.data.value_number;
        }
      }
    }
    AstExprBinaryOp::Pow => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Number;
        unsafe {
          result.data.value_number = la.data.value_number.powf(ra.data.value_number);
        }
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
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Boolean;
        unsafe {
          result.data.value_boolean = la.data.value_number < ra.data.value_number;
        }
      }
    }
    AstExprBinaryOp::CompareLe => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Boolean;
        unsafe {
          result.data.value_boolean = la.data.value_number <= ra.data.value_number;
        }
      }
    }
    AstExprBinaryOp::CompareGt => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Boolean;
        unsafe {
          result.data.value_boolean = la.data.value_number > ra.data.value_number;
        }
      }
    }
    AstExprBinaryOp::CompareGe => {
      if la.r#type == Type::Number && ra.r#type == Type::Number {
        result.r#type = Type::Boolean;
        unsafe {
          result.data.value_boolean = la.data.value_number >= ra.data.value_number;
        }
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
