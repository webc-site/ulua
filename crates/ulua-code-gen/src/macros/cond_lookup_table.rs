//! 条件查找表骨架宏：把「条件枚举 ↔ 平台编码/别名」的编译期定表 + 哨兵自检 +
//! 越界回退三件套收口为单一 `macro_rules!`，消除 get_condition_fp /
//! get_condition_int_64 / get_condition_int_emit_common_x_64 /
//! get_condition_int_ir_lowering_a_64 / get_negated_condition_* 各文件的结构漂移。

/// 生成一张「源条件枚举 → 目标枚举」的编译期索引定表与查询函数。
///
/// 生成物与收敛前各文件手写体逐项一致：
/// - `const $table: [$dst; $len]`：以 `$key as usize` 为下标、`$sentinel` 填充
///   未赋值项的编译期定表；
/// - `const _` 哨兵自检：任何未赋值项（仍等于 `$sentinel`）都在编译期报错；
/// - `$vis fn $fn_name($arg: $src) -> $dst`：表命中返回映射值；下标越界走
///   回退臂——可选 `CODEGEN_ASSERT`（消息为字面量）后返回 `$fallback`。
///
/// `$(#[$meta])*` 原样附着于函数（文档注释与 `#[inline]` 均经此传入）。
#[macro_export]
macro_rules! cond_lookup_table {
  (
    $(#[$meta:meta])*
    $vis:vis fn $fn_name:ident($arg:ident: $src:ty) -> $dst:ty {
      table: $table:ident,
      len: $len:expr,
      sentinel: $sentinel:expr,
      $(assert: $assert_msg:literal,)?
      fallback: $fallback:expr,
      entries: { $($key:path => $value:path),+ $(,)? }
    }
  ) => {
    const $table: [$dst; $len] = {
      let mut t = [$sentinel; $len];
      $(t[$key as usize] = $value;)+
      t
    };

    const _: () = {
      let mut i = 0;
      while i < $table.len() {
        assert!(
          $table[i] as u32 != $sentinel as u32,
          concat!(stringify!($table), " 存在未赋值的哨兵项")
        );
        i += 1;
      }
    };

    $(#[$meta])*
    $vis fn $fn_name($arg: $src) -> $dst {
      match $table.get($arg as usize) {
        Some(&value) => value,
        None => {
          $($crate::macros::codegen_assert::CODEGEN_ASSERT!(false, $assert_msg);)?
          $fallback
        }
      }
    }
  };
}
