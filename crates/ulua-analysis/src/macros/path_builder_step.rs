//! cpp `PathBuilder`（`TypePath.cpp:174-272`）部件工厂方法的骨架单点。
//!
//! 该 builder 的每个方法都是同一形状：`self.components.push(<部件>); self`。
//! 本仓库原先把这套骨架在 15 个模块里各手抄一遍（单词方法为 cpp 同名方法需要
//! 拆出的扩展 trait，`prop`/`index_key` 一类是固有 impl），只有部件表达式不同。
//! [`path_builder_step!`] 保留两种形态与所有对外路径、trait 名、方法签名不变，
//! 调用点只剩部件表达式本身。

/// 生成 `PathBuilder` 的一个「追加部件并返回 `&mut Self`」方法。
///
/// 用法（扩展 trait 形态，`trait` 关键字）：
/// ```ignore
/// path_builder_step!(
///   trait PathBuilderMt,
///   mt() => Component::TypeField(TypeField::Metatable)
/// );
/// ```
/// 用法（固有方法形态，直接给方法名）：
/// ```ignore
/// path_builder_step!(
///   pack_slice(start_index: usize) => Component::PackSlice(PackSlice { start_index })
/// );
/// ```
/// 部件表达式里用到的类型（`Component` 及其 payload）仍由调用方 `use`，
/// `PathBuilder` 由宏自带全路径，调用方无需再导入。
macro_rules! path_builder_step {
  // 扩展 trait 形态。
  (
    trait $trait:ident, $name:ident($($arg:ident : $ty:ty),*) => $component:expr $(,)?
  ) => {
    pub trait $trait {
      fn $name(&mut self $(, $arg: $ty)*) -> &mut Self;
    }

    impl $trait for $crate::records::path_builder::PathBuilder {
      fn $name(&mut self $(, $arg: $ty)*) -> &mut Self {
        self.components.push($component);
        self
      }
    }
  };
  // 固有方法形态。
  (
    $name:ident($($arg:ident : $ty:ty),*) => $component:expr $(,)?
  ) => {
    impl $crate::records::path_builder::PathBuilder {
      pub fn $name(&mut self $(, $arg: $ty)*) -> &mut Self {
        self.components.push($component);
        self
      }
    }
  };
}

pub(crate) use path_builder_step;
