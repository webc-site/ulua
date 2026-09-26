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
