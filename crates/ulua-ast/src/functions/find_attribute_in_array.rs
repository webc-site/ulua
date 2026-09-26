use crate::records::ast_attr::{AstAttr, AstAttrType};

/// 在属性节点迭代器中按类型查找:`Option<&AstAttr>` 出,null 槽与未命中统一折叠
/// 为 `None`(cpp `findAttribute` 的 null 哨兵在 Rust 侧消失)。旧指针数组形态
/// (`AstArray::iter_nodes`)与句柄化形态(`Nodes::iter`)喂同一实现,杜绝双份
/// 逻辑漂移。
pub(crate) fn find_attribute_in_array<'a>(
  mut attributes: impl Iterator<Item = &'a AstAttr>,
  attribute_type: AstAttrType,
) -> Option<&'a AstAttr> {
  attributes.find(|attr| attr.r#type == attribute_type)
}
