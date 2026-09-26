use core::ptr::NonNull;

use crate::records::{ast_type::AstType, ast_type_pack::AstTypePack};

/// cpp `AstTypeOrPack`（`Ast/include/Luau/Ast.h:138`）：类型注解槽，二者取一。
///
/// cpp 用一对默认 null 的裸指针当哨兵，但两端都是严格的「和/或」读法：构造点恒只填
/// 一侧（`Parser.cpp:3032/3041/3054/3074/4713/4768/4773/4787/4796`），消费点一律
/// `if (type) … else typePack->…`（`Ast.cpp:37-43`、`ConstraintGenerator.cpp:3730`，
/// `TypeChecker2.cpp:3121` 更直接 `LUAU_ASSERT(type || typePack)`）。故此处落成 enum，
/// 哨兵与判空一并消失；与 cpp 的差异仅在于「哪一侧被填」由类型系统而非运行期约定表达。
///
/// `Error` 承载 cpp「两侧皆 null」的形态：解析报错本身不产出它（`reportTypeError`
/// 返回 `AstTypeError` 节点，`Parser.cpp:5466`），它只出现在 `parseTypeParams` 等待
/// 后缀解析的占位中间态（`Parser.cpp:4719-4726`）与外部以 null 指针构造的槽位。
#[derive(Debug, Clone, Copy)]
pub enum AstTypeOrPack {
  /// 类型注解（cpp 仅 `type` 非空）。
  Type(&'static AstType),
  /// 类型包注解（cpp 仅 `typePack` 非空）。
  Pack(&'static AstTypePack),
  /// 错误态：cpp 双指针皆为 null（无注解可承载）。
  Error,
}

/// arena 裸指针 → 长寿共享引用的唯一收口：null 折叠为 `None`，非空才解引用。
///
/// 契约（本模块唯一的 `unsafe` 取值点）：`node` 必须出自 `Allocator`/`Parser::alloc`
/// 系的 bump arena——这类 arena 的页常驻直到解析会话结束，分配返回的地址不再移动，
/// 故非空 `node` 指向的节点必然比其消费者（`ParseResult` 及其下游检查器）长寿，
/// 解引用得到的 `'static` 共享引用成立；节点在引用存活期内只读不改写。
/// 刻意做成 safe fn（同 `Parser::alloc` 的取舍）：`AstTypeOrPack` 的构造入口因此
/// 不必是 `unsafe fn`，业务侧也就拿不到「裸指针 → `'static`」这条捷径。
#[inline]
fn arena_static<T>(node: *mut T) -> Option<&'static T> {
  // Safety: 前置条件即上方契约，由 `from_type`/`from_type_pack` 的全部调用点兑现
  // （传入的都是 arena 分配返回值或 null 哨兵）。
  unsafe { node.as_ref() }
}

impl AstTypeOrPack {
  /// 由 arena 产出的类型节点构造 `Type` 变体（cpp `AstTypeOrPack{type, nullptr}`）。
  ///
  /// `node` 为 null 时落 [`Self::Error`]，与 cpp 未填充哨兵的读法同值；非 null 时
  /// 必须是 arena 存活节点（见 `arena_static` 的契约）。
  #[inline]
  pub fn from_type(node: *mut AstType) -> Self {
    match arena_static(node) {
      Some(ty) => Self::Type(ty),
      None => Self::Error,
    }
  }

  /// [`Self::from_type`] 的 `AstTypePack` 形态（cpp `AstTypeOrPack{nullptr, typePack}`）。
  #[inline]
  pub fn from_type_pack(node: *mut AstTypePack) -> Self {
    match arena_static(node) {
      Some(pack) => Self::Pack(pack),
      None => Self::Error,
    }
  }

  /// 类型注解槽的只读投影：非 `Type` 形态（含错误态）返回 `None`，
  /// 对应 cpp 读 `.type` 得到 null。
  #[inline]
  pub fn as_type(&self) -> Option<&'static AstType> {
    match *self {
      Self::Type(ty) => Some(ty),
      Self::Pack(_) | Self::Error => None,
    }
  }

  /// 类型包注解槽的只读投影：非 `Pack` 形态（含错误态）返回 `None`，
  /// 对应 cpp 读 `.typePack` 得到 null。
  #[inline]
  pub fn as_pack(&self) -> Option<&'static AstTypePack> {
    match *self {
      Self::Pack(pack) => Some(pack),
      Self::Type(_) | Self::Error => None,
    }
  }

  /// [`Self::as_pack`] 的 `Option<NonNull>` 形态，供 arena 子节点槽位
  /// （`AstGenericTypePack::default_value`）承接：非 `Pack` 形态即 `None`，
  /// 与 cpp 读 `.typePack` 得 null 同值。`NonNull` 只承诺「非空 + 地址稳定」，
  /// 后续 pass 仍可经裸指针句柄写穿节点（见 `optional_node` 模块说明）。
  #[inline]
  pub fn as_pack_node(&self) -> Option<NonNull<AstTypePack>> {
    match *self {
      Self::Pack(pack) => Some(NonNull::from(pack)),
      Self::Type(_) | Self::Error => None,
    }
  }
}
