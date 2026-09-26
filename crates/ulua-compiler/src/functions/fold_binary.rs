use alloc::vec::Vec;

use ulua_ast::records::{ast_expr_binary::AstExprBinaryOp, ast_name_table::AstNameTable};
use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{c_const::cvar, constants_equal::constants_equal},
  records::constant::Constant,
};

/// 常量字符串折叠上限（C++ `kConstantFoldStringLimit`）
const K_CONSTANT_FOLD_STRING_LIMIT: u32 = 4096;

/// 二元折叠用的标量(f64)/向量分量(f32)运算指针对
type FoldPair = (fn(f64, f64) -> f64, fn(f32, f32) -> f32);

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

/// C++ `vectorsDifferOnlyInW`（ConstantFolding.cpp:63-75）：LUA_VECTOR_SIZE == 3 的运行时
/// 相等（`luai_veceq` 只比 x/y/z）看不到第 4 分量 w，仅 w 不同的两个向量若被编译期折叠成
/// `==`/`~=` 常量，结果会与运行时不一致；`LuauCompileNoFoldVectorEqW` 开启时跳过折叠。
/// 本端口常量向量统一 `Vector([f32; 4])` 存储，无需 cpp 的 Vectorf/Vectord 双分支。
#[inline]
fn vectors_differ_only_in_w(la: &Constant, ra: &Constant) -> bool {
  if let (Constant::Vector(l), Constant::Vector(r)) = (la, ra) {
    l[0] == r[0] && l[1] == r[1] && l[2] == r[2] && l[3] != r[3]
  } else {
    false
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
/// 仅当结果第 4 分量非零或有输入携带 w 分量时才折叠，否则返回 `None`
/// （cpp 侧即"不写出参 result"）。
fn fold_vector(
  la: [f32; 4],
  ra: [f32; 4],
  op: impl Fn(f32, f32) -> f32,
  w: f32,
  had_w: bool,
) -> Option<Constant> {
  (w == 0.0 || had_w)
    .then(|| Constant::Vector([op(la[0], ra[0]), op(la[1], ra[1]), op(la[2], ra[2]), w]))
}

/// 向量逐分量除法后取整（FloorDiv 用）
fn floor_div(a: f32, b: f32) -> f32 {
  (a / b).floor()
}

/// Number/Vector 组合的公共折叠路径：vec×vec、num×vec、vec×num 三种形态
/// 统一走 [`fold_vector`]；`had_w` 仅看原始 Vector 操作数的 w 分量
/// （标量广播侧的 [3] 是标量本身，不参与判定，逐一对齐 C++ 分支语义）；
/// 三形态皆不命中或折叠条件不成立都返回 `None`。
fn fold_vector_combo(
  la: &Constant,
  ra: &Constant,
  op: impl Fn(f32, f32) -> f32,
) -> Option<Constant> {
  if let Some((a, b)) = vec2(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(a, b, op, w, a[3] != 0.0 || b[3] != 0.0)
  } else if let Some((a, b)) = num_vec(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(a, b, op, w, b[3] != 0.0)
  } else if let Some((a, b)) = vec_num(la, ra) {
    let w = op(a[3], b[3]);
    fold_vector(a, b, op, w, a[3] != 0.0)
  } else {
    None
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
    // Add/Sub 两臂同构（标量式 + vec×vec 逐分量式，w 分量恒参与折叠）：
    // 运算符以函数指针收口为一条骨架（C++ had_w = true，必得 Some）。
    AstExprBinaryOp::Add | AstExprBinaryOp::Sub => {
      let (scalar, component): FoldPair = if op == AstExprBinaryOp::Add {
        (|a, b| a + b, |a, b| a + b)
      } else {
        (|a, b| a - b, |a, b| a - b)
      };

      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(scalar(a, b));
      } else if let Some((a, b)) = vec2(la, ra)
        && let Some(v) = fold_vector(a, b, component, component(a[3], b[3]), true)
      {
        result = v;
      }
    }
    // Mul/Div/FloorDiv 三臂同构（标量式 + 逐分量向量式）：两份运算以函数指针
    // 收口，共享一条折叠骨架。
    AstExprBinaryOp::Mul | AstExprBinaryOp::Div | AstExprBinaryOp::FloorDiv => {
      let (scalar, component): FoldPair = match op {
        AstExprBinaryOp::Mul => (|a, b| a * b, |a, b| a * b),
        AstExprBinaryOp::Div => (|a, b| a / b, |a, b| a / b),
        _ => (|a, b| (a / b).floor(), floor_div),
      };

      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Number(scalar(a, b));
      } else if let Some(v) = fold_vector_combo(la, ra, component) {
        result = v;
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
        // checked_add：u32 裸加在极端长度下会回绕成小值，绕过 4096 折叠
        // 上限；长度和溢出即放弃折叠（result 保持 Unknown）。
        if let Some(total) = l.len.checked_add(r.len)
          && total <= K_CONSTANT_FOLD_STRING_LIMIT
        {
          // 任一侧为空时直接沿用另一侧指针，避免拼接
          let ptr = if l.len == 0 {
            r.ptr
          } else if r.len == 0 {
            l.ptr
          } else {
            let mut tmp = Vec::with_capacity(total as usize);
            tmp.extend_from_slice(la.get_string_bytes());
            tmp.extend_from_slice(ra.get_string_bytes());
            string_table.get_or_add_slice(&tmp).value.cast()
          };
          result = Constant::string(ptr, total);
        }
      }
    }
    // CompareNe/CompareEq 两臂只差结果取反：等式判定单点求值，按 op 决定是否取反
    // （C++ ConstantFolding.cpp:490-506：flag 开启且两向量仅 w 不同时不折叠）。
    AstExprBinaryOp::CompareNe | AstExprBinaryOp::CompareEq => {
      if !la.is_unknown()
        && !ra.is_unknown()
        && !(fflag::LuauCompileNoFoldVectorEqW.get() && vectors_differ_only_in_w(la, ra))
      {
        let equal = constants_equal(la, ra);
        result = Constant::Boolean(equal == (op == AstExprBinaryOp::CompareEq));
      }
    }
    // 四个有序比较臂同构（仅标量谓词不同）：谓词以函数指针收口为一条骨架。
    AstExprBinaryOp::CompareLt
    | AstExprBinaryOp::CompareLe
    | AstExprBinaryOp::CompareGt
    | AstExprBinaryOp::CompareGe => {
      let ordered: fn(f64, f64) -> bool = match op {
        AstExprBinaryOp::CompareLt => |a, b| a < b,
        AstExprBinaryOp::CompareLe => |a, b| a <= b,
        AstExprBinaryOp::CompareGt => |a, b| a > b,
        _ => |a, b| a >= b,
      };

      if let Some((a, b)) = num2(la, ra) {
        result = Constant::Boolean(ordered(a, b));
      }
    }
    // And/Or 两臂同构：And 取假侧值、Or 取真侧值，即「保留左值」的条件
    // 恰为真值性与运算符一致。
    AstExprBinaryOp::And | AstExprBinaryOp::Or => {
      if !la.is_unknown() {
        let keep_left = la.is_truthful() == (op == AstExprBinaryOp::Or);
        result = if keep_left { *la } else { *ra };
      }
    }
    // C++ default：仅哨兵 OpCount 会到达
    AstExprBinaryOp::OpCount => {
      LUAU_ASSERT!(false);
    }
  }

  result
}
