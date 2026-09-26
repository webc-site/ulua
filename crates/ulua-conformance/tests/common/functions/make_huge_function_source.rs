use alloc::{fmt::Write, string::String};

/// cpp 压力用例的表项数：4 万个数字字面量撑大单函数（原 `0..40000` 魔法数字）。
const TABLE_ENTRIES: u32 = 40_000;

pub fn make_huge_function_source() -> String {
  // 预估容量：每项 "0.<i>," 约 10 字节，免 4 万次扩容搬运。
  let mut source = String::with_capacity(TABLE_ENTRIES as usize * 10 + 64);
  source.push_str("if ... then\n");
  source.push_str("local _ = {\n");
  // write! 直写进 source，替代每轮 format! 的临时 String 分配；
  // String 的 fmt 写入只可能 OOM，不会 Err，unwrap 100% 安全。
  for i in 0..TABLE_ENTRIES {
    write!(source, "0.{i},").unwrap();
  }
  source.push_str("}\n");
  source.push_str("end\n");
  source.push_str("return bit32.lshift('84', -1)");
  source
}
