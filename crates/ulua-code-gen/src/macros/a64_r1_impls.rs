//! A64 `placeR1` 单源浮点臂坍缩骨架（对照 cpp `AssemblyBuilderA64.cpp` 各
//! `placeR1(name, dst, src, op)` 三点段）。
//!
//! 这些指令的 Q/D/S 三臂互为镜像：仅差助记符字面量与三枚 22 位 op 常量，
//! 体形一律为「两条 debug_assert + Q→D→S 级联 place_r_1」，生成的调用与
//! 原手写体逐字同形（同一 place_r_1、同一实参次序、同一 op 位串），发射
//! 编码与日志文本完全不变。
//!
//! 每次调用同时生成 `A64_R1_ROWS` 对账表（行 token 与生成体同源），由
//! tests/a64_r1_table_matches_cpp.rs 与 cpp 逐格核对。
//!
//! 不纳入本宏的臂（差异实质性，保留手写）：
//! - `fneg`：每臂各自 debug_assert 且 D→S→Q 次序不同（cpp :869-880）；
//! - `fsqrt`：仅 D/S 两臂（cpp :1837-1846），无 Q 臂级联形态。
#[macro_export]
macro_rules! a64_r1_impls {
  ($( $meth:ident => ( $mnem:literal, $q:expr, $d:expr, $s:expr ); )+) => {
    /// 助记符 × Q/D/S 三臂 op 对账表（与下方生成体同源）。
    pub const A64_R1_ROWS: &[(&str, u32, u32, u32)] = &[ $( ($mnem, $q, $d, $s), )+ ];

    $(
      impl $crate::records::assembly_builder_a_64::AssemblyBuilderA64 {
        pub fn $meth(
          &mut self,
          dst: $crate::records::register_a_64::RegisterA64,
          src: $crate::records::register_a_64::RegisterA64,
        ) {
          use $crate::enums::kind_a_64::KindA64;

          debug_assert!(dst.kind() == src.kind());
          debug_assert!(matches!(dst.kind(), KindA64::D | KindA64::S | KindA64::Q));

          if dst.kind() == KindA64::Q {
            self.place_r_1($mnem, dst, src, $q);
          } else if dst.kind() == KindA64::D {
            self.place_r_1($mnem, dst, src, $d);
          } else {
            self.place_r_1($mnem, dst, src, $s);
          }
        }
      }
    )+
  };
}

pub use a64_r1_impls;
