//! 声明式样板坍缩宏族（生成宏本体逐字保真；r7-macros98 合并票）。

/// 为「以 `1 << n` 为值、以裸整型位集做容器」的位标志枚举生成全套 cast 风样板，
/// 收口 `code_gen_flags` / `features_a_64` / `features_x_64` / `function_stats_flags`
/// 四份逐字重复的手写体（对照 cpp：`CodeGenOptions.h:17`、`AssemblyBuilderA64.h:20`、
/// `AssemblyBuilderX64.h:24`、`LoweringStats.h:37`）。
///
/// 生成物与原手写版逐项一致：
/// - 派生 `Debug, Clone, Copy, PartialEq, Eq, Hash` + `#[repr($repr)]` 的枚举本体；
/// - 固有实现 `mask` / `is_set` / `set`（前两者 `const`）；
/// - `Self | Self -> $repr`、`$repr | Self -> $repr`、`$repr |= Self`、`$repr & Self -> $repr`
///   四个位运算实现（运算符统一走 `::core::ops` 绝对路径，免调用点导入）；
/// - 可选 `aliases { SCREAMING_CASE = Variant }` 别名块（对齐 cpp 下划线命名）。
#[macro_export]
macro_rules! flag_enum {
  (
    $(#[$enum_meta:meta])*
    $vis:vis enum $Name:ident : $repr:ty {
      $($(#[$v_meta:meta])* $variant:ident = $value:expr),+ $(,)?
    }
    $(aliases { $($alias:ident = $alias_src:ident),+ $(,)? })?
  ) => {
    $(#[$enum_meta])*
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr($repr)]
    $vis enum $Name {
      $($(#[$v_meta])* $variant = $value,)+
    }

    impl $Name {
      $($(pub const $alias: Self = Self::$alias_src;)+)?

      #[inline]
      pub const fn mask(self) -> $repr {
        self as $repr
      }

      #[inline]
      pub const fn is_set(self, flags: $repr) -> bool {
        (flags & (self as $repr)) != 0
      }

      #[inline]
      pub fn set(self, flags: &mut $repr) {
        *flags |= self as $repr;
      }
    }

    impl ::core::ops::BitOr for $Name {
      type Output = $repr;

      #[inline]
      fn bitor(self, rhs: Self) -> $repr {
        (self as $repr) | (rhs as $repr)
      }
    }

    impl ::core::ops::BitOr<$Name> for $repr {
      type Output = $repr;

      #[inline]
      fn bitor(self, rhs: $Name) -> $repr {
        self | (rhs as $repr)
      }
    }

    impl ::core::ops::BitOrAssign<$Name> for $repr {
      #[inline]
      fn bitor_assign(&mut self, rhs: $Name) {
        *self |= rhs as $repr;
      }
    }

    impl ::core::ops::BitAnd<$Name> for $repr {
      type Output = $repr;

      #[inline]
      fn bitand(self, rhs: $Name) -> $repr {
        self & (rhs as $repr)
      }
    }
  };
}

/// 为 cpp 刻意共享裸指针三件套（`build`/`function`/`stats`）的四个结构
/// （`IrLoweringX64`/`IrLoweringA64`/`IrRegAllocX64`/`IrRegAllocA64`）生成同名
/// 访问器家族：`function_mut`/`function_ref`/`build_mut`/`stats_mut`。
///
/// 统一契约（与各结构体文档登记一致）：三个指针由构造点（`lower_function`）以
/// 可变引用注入为裸指针，所指对象覆盖整个降级栈帧、比本对象长寿且与本对象无
/// 内存重叠；访问器即时派生借用、语句内消费，不长期持有。`stats` 可为空（顶层
/// 允许不采集统计），由 `stats_mut` 统一判空。宏只收口 4×4 份逐位重复的手写体，
/// 不引入新语义；各结构特有的混窗/闭包窗门面（`build_regs_mut`、`with_op_label`
/// 等）仍在各自文件手写。
#[macro_export]
macro_rules! shared_ptr_accessors {
  ($build_ty:ty) => {
    /// Safety:`function` 由构造点以 `&mut IrFunction` 注入为裸指针，所指对象覆盖
    /// 整个降级栈帧、比本对象长寿且与本对象无内存重叠；此处即时派生唯一可变
    /// 借用、语句内消费。
    #[inline]
    pub(crate) fn function_mut(&mut self) -> &mut IrFunction {
      unsafe { &mut *self.function }
    }

    /// 只读视图：供 `&self` 接收者的读取门面（操作数取值、判空前的字段读）使用。
    ///
    /// Safety:同 `function_mut`；仅派生共享借用。
    #[inline]
    pub(crate) fn function_ref(&self) -> &IrFunction {
      unsafe { &*self.function }
    }

    /// 发射视图访问器：`build` 的注入契约同 `function_mut`；同一调用语句内还需
    /// 其它视图时走各结构手写的混窗门面（`build_regs_mut` 等），不复用本访问器。
    ///
    /// Safety:见结构体注释；即时派生唯一可变借用、语句内消费。
    #[inline]
    pub(crate) fn build_mut(&mut self) -> &mut $build_ty {
      unsafe { &mut *self.build }
    }

    /// 统计指针可为空（顶层允许不采集统计），收敛判空 + 解引用样板。
    ///
    /// Safety:`stats` 非空时契约同 `function_mut`。
    #[inline]
    pub(crate) fn stats_mut(&mut self) -> Option<&mut LoweringStats> {
      if self.stats.is_null() {
        return None;
      }
      // Safety:同 `function_mut`。
      Some(unsafe { &mut *self.stats })
    }
  };
}

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
