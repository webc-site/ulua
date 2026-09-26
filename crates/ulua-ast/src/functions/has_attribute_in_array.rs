use crate::{
  functions::find_attribute_in_array::find_attribute_in_array,
  records::{
    ast_array::AstArray,
    ast_attr::{AstAttr, AstAttrType},
  },
};

/// 旧 `AstArray<*mut AstAttr>` 形态的查找入口:元素裸指针逐个解引用为 `&T`。
pub(crate) fn find_attribute_in_raw<'a>(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> Option<&'a AstAttr> {
  find_attribute_in_array(
    attributes.as_slice().iter().filter_map(|&attribute| {
      // Safety: 元素是 parser 在 arena 分配的存活 AstAttr 节点或 null(attributes
      // 数组由构造期写入,arena 地址不移动);as_ref 把判空折叠为 None 跳过空槽,
      // 命中后仅读 type 字段,无写入无别名。
      unsafe { attribute.as_ref() }
    }),
    attribute_type,
  )
}

pub(crate) fn has_attribute_in_array(
  attributes: AstArray<*mut AstAttr>,
  attribute_type: AstAttrType,
) -> bool {
  find_attribute_in_raw(attributes, attribute_type).is_some()
}
