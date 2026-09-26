/// x64 常量池四兄弟（`i32`/`f32`/`i64`/`f64`）的公共骨架，收口 4×46 行逐字重复的
/// RIP 相对常量发射逻辑（对照 cpp：`AssemblyBuilderX64.cpp` 的同名四函数）。
///
/// 骨架：算 key → 命中缓存直接返回 `rip + prev`；否则 `allocate_data` 落位、写入位型、
/// 算偏移、回填缓存、返回 `rip + offset`。哨兵位型 `!0` 预留给内部寻址，不入池。
/// 四处细节差异全部由调用点以自包含闭包显式给出（宏卫生：调用点闭包只见自身参数，
/// 不引用宏内部绑定），不做静默统一：
/// - `key`：整型直转位型，浮点经 `get_float_bits`/`get_double_bits`；
/// - `write`：`writeu_*`/`writef_*` 写原语（闭包体内自带 unsafe）；
/// - `offset`：整型用 isize 差转 i32，浮点用 i32 直接相减；
/// - `store`：整型 `try_insert`（先到先得），浮点 `*get_or_insert(k) = v`（后到覆盖）；
///   本路径回填前必然未命中，两式行为等价，但保留 cpp 原写法以利逐行对照。
#[macro_export]
macro_rules! x64_rip_const {
  (
    $fn_name:ident($value:ident : $ty:ty):
      key $key_conv:expr,
      cache $cache:ident,
      size $size:ident,
      bytes $bytes:literal,
      write $write:expr,
      offset $offset_conv:expr,
      store $store:expr $(,)?
  ) => {
    impl $crate::records::assembly_builder_x_64::AssemblyBuilderX64 {
      pub fn $fn_name(&mut self, $value: $ty) -> $crate::records::operand_x_64::OperandX64 {
        // `rip + imm` 内存操作数（NOREG 基址、scale 1），两次返回共用
        fn rip(
          size: $crate::enums::size_x_64::SizeX64,
          imm: i32,
        ) -> $crate::records::operand_x_64::OperandX64 {
          $crate::records::operand_x_64::OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
            size,
            $crate::records::register_x_64::RegisterX64::NOREG,
            1,
            $crate::records::register_x_64::RegisterX64::RIP,
            imm,
          )
        }

        let key = ($key_conv)($value);

        // 哨兵位型不入池；命中则复用既有偏移
        if key != !0
          && let Some(prev) = self.$cache.find(&key)
        {
          return rip($crate::enums::size_x_64::SizeX64::$size, *prev);
        }

        let pos = self.allocate_data($bytes, $bytes);

        // Safety: allocate_data(bytes, bytes) 预留 ≥bytes 字节使 pos+bytes ≤ data.len()，
        // 写原语于 [pos, pos+bytes) 写入；&mut self 独占 data，无别名。
        let dst = unsafe { self.data.as_mut_ptr().add(pos) };
        ($write)(dst, $value);

        let offset = ($offset_conv)(pos, self.data.len());

        if key != !0 {
          ($store)(&mut self.$cache, key, offset);
        }

        rip($crate::enums::size_x_64::SizeX64::$size, offset)
      }
    }
  };
}
