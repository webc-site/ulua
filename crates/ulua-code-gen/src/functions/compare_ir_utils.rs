//! `Luau::CodeGen` 的 `compare` 重载（cpp/CodeGen/src/IrUtils.cpp）：按 `IrCondition`
//! 求值两个常量操作数，供常量折叠（fold_constants）与内联跳转折叠（const_prop_in_inst）使用。

use core::cmp::Ordering;

use crate::{enums::ir_condition::IrCondition, macros::codegen_assert::CODEGEN_ASSERT};

/// 整数 `compare` 的位宽参数：`Unsigned*` 条件族需要把同宽操作数按无符号重新解释
/// （cpp 里的 `unsigned(a)` / `uint64_t(a)`），其余条件族按有符号 `Ord` 比较。
pub(crate) trait IrIntOperand: Copy + Ord {
  type Unsigned: Ord;

  fn to_unsigned(self) -> Self::Unsigned;
}

impl IrIntOperand for i32 {
  type Unsigned = u32;

  fn to_unsigned(self) -> u32 {
    self as u32
  }
}

impl IrIntOperand for i64 {
  type Unsigned = u64;

  fn to_unsigned(self) -> u64 {
    self as u64
  }
}

/// cpp/CodeGen/src/IrUtils.cpp 的 `compare(int, int, IrCondition)`（:727）与
/// `compare(int64_t, int64_t, IrCondition)`（:766）条件表逐行相同，只有位宽不同，
/// 故合并为一个泛型实现（调用点按 `a`/`b` 的类型推断出 `i32` 或 `i64`）。
///
/// 整数是全序，cpp 中 `!(a < b)` 一类的取反分支等价于反向比较；
/// `IrCondition::Count` 不是条件，对应 cpp 的 `default: CODEGEN_ASSERT(!"Unsupported condition")`。
pub(crate) fn compare_int<T: IrIntOperand>(a: T, b: T, cond: IrCondition) -> bool {
  let ord = match cond {
    IrCondition::UnsignedLess
    | IrCondition::UnsignedLessEqual
    | IrCondition::UnsignedGreater
    | IrCondition::UnsignedGreaterEqual => a.to_unsigned().cmp(&b.to_unsigned()),
    _ => a.cmp(&b),
  };

  match cond {
    IrCondition::Equal => ord == Ordering::Equal,
    IrCondition::NotEqual => ord != Ordering::Equal,
    IrCondition::Less | IrCondition::NotGreaterEqual | IrCondition::UnsignedLess => {
      ord == Ordering::Less
    }
    IrCondition::NotLessEqual | IrCondition::Greater | IrCondition::UnsignedGreater => {
      ord == Ordering::Greater
    }
    IrCondition::NotLess | IrCondition::GreaterEqual | IrCondition::UnsignedGreaterEqual => {
      ord != Ordering::Less
    }
    IrCondition::LessEqual | IrCondition::NotGreater | IrCondition::UnsignedLessEqual => {
      ord != Ordering::Greater
    }
    IrCondition::Count => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      false
    }
  }
}

/// cpp/CodeGen/src/IrUtils.cpp `compare(double, double, IrCondition)`（:695）。
/// 浮点是偏序：取反分支必须用 `partial_cmp` 显式判 NaN，不能化简为反向比较
/// （cpp 注释里那些冗余 `bool()` 强转就是为守住 IEEE754 语义）。
/// `Unsigned*` 条件族对 double 无意义，与 cpp 一样落进 `default` 断言分支。
pub fn compare_f64_f64_ir_condition(a: f64, b: f64, cond: IrCondition) -> bool {
  match cond {
    IrCondition::Equal => a == b,
    IrCondition::NotEqual => a != b,
    IrCondition::Less => a < b,
    IrCondition::NotLess => !matches!(a.partial_cmp(&b), Some(Ordering::Less)),
    IrCondition::LessEqual => a <= b,
    IrCondition::NotLessEqual => {
      !matches!(a.partial_cmp(&b), Some(Ordering::Less | Ordering::Equal))
    }
    IrCondition::Greater => a > b,
    IrCondition::NotGreater => !matches!(a.partial_cmp(&b), Some(Ordering::Greater)),
    IrCondition::GreaterEqual => a >= b,
    IrCondition::NotGreaterEqual => {
      !matches!(a.partial_cmp(&b), Some(Ordering::Greater | Ordering::Equal))
    }
    _ => {
      CODEGEN_ASSERT!(false, "Unsupported condition");
      false
    }
  }
}
