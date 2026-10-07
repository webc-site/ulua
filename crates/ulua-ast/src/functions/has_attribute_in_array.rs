use crate::{
  functions::{find_attribute_in_array::find_attribute_in_array, optional_node::slot_opt},
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
  },
};

/// 旧 `AstArray<*mut AstAttr>` 形态的查找入口:元素槽位经 [`slot_opt`] 把 null
/// 折叠为 `None`(cpp `findAttribute` 在空槽上会解引用,此处与其防御性等价的
/// 跳过形态),再交给共用内核 [`find_attribute_in_array`]——本函数零裸指针解引用。
/// `'static` 即 arena 只读纪律(`optional_node::slot_opt` 的契约),不再开
/// 无约束的 `'a` 让调用方自选寿命。
pub(crate) fn find_attribute_in_raw(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> Option<&'static AstAttr> {
  find_attribute_in_array(
    attributes.iter().filter_map(|&p| slot_opt(p)),
    attribute_type,
  )
}

pub(crate) fn has_attribute_in_array(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> bool {
  find_attribute_in_raw(attributes, attribute_type).is_some()
}
