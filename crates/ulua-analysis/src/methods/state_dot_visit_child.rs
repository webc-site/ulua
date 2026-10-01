use ulua_common::functions::format_append::format_append;

use crate::{
  functions::{get_type, to_string_to_string::to_string_type_id},
  records::{
    any_type::AnyType, never_type::NeverType, primitive_type::PrimitiveType, state_dot::StateDot,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl StateDot {
  /// 输出父子边；link_name 为 None 时输出无边标签的边。
  /// 对应 C++ `StateDot::visitChild` 两个重载共用的连边段。
  fn link_child(&mut self, index: i32, parent_index: i32, link_name: Option<&str>) {
    if parent_index == 0 {
      return;
    }
    match link_name {
      Some(name) => format_append(
        &mut self.result,
        format_args!("n{} -> n{} [label=\"{}\"];\n", parent_index, index, name),
      ),
      None => format_append(
        &mut self.result,
        format_args!("n{} -> n{};\n", parent_index, index),
      ),
    }
  }

  /// C++ `StateDot::visitChild(TypeId, int, const char*)`：
  /// 登记 ty 的节点索引、连边，再递归访问子节点。
  /// duplicate_primitives 下基本类型每次出现都分配新索引。
  pub fn visit_child_type_id(&mut self, ty: TypeId, parent_index: i32, link_name: Option<&str>) {
    if !self.ty_to_index.contains_key(&ty)
      || (self.opts.duplicate_primitives && self.can_duplicate_primitive(ty))
    {
      *self.ty_to_index.get_or_insert(ty) = self.next_index;
      self.next_index += 1;
    }

    // Safety: 上方登记块两路（未含键则插入 / 含键但需重复分配则覆写）后键恒在，
    // 与 cpp tyToIndex.find(ty)->second 同位，取回必命中。
    let index = *self
      .ty_to_index
      .find(&ty)
      .expect("上方登记块蕴含键恒在，取回必命中");

    self.link_child(index, parent_index, link_name);

    if self.opts.duplicate_primitives && self.can_duplicate_primitive(ty) {
      if get_type::get::<PrimitiveType>(ty).is_some() {
        let s = to_string_type_id(ty);
        format_append(
          &mut self.result,
          format_args!("n{} [label=\"{}\"];\n", index, s),
        );
      } else if get_type::get::<AnyType>(ty).is_some() {
        format_append(
          &mut self.result,
          format_args!("n{} [label=\"any\"];\n", index),
        );
      } else if get_type::get::<UnknownType>(ty).is_some() {
        format_append(
          &mut self.result,
          format_args!("n{} [label=\"unknown\"];\n", index),
        );
      } else if get_type::get::<NeverType>(ty).is_some() {
        format_append(
          &mut self.result,
          format_args!("n{} [label=\"never\"];\n", index),
        );
      }
    } else {
      self.visit_children_type_id_i32(ty, index);
    }
  }

  /// C++ `StateDot::visitChild(TypePackId, int, const char*)`：
  /// 登记 tp 的节点索引、连边，再递归访问子节点。
  pub fn visit_child_type_pack_id(
    &mut self,
    tp: TypePackId,
    parent_index: i32,
    link_name: Option<&str>,
  ) {
    if !self.tp_to_index.contains_key(&tp) {
      self.tp_to_index.try_insert(tp, self.next_index);
      self.next_index += 1;
    }

    // Safety: 同上——登记块后 tp_to_index 键恒在，与 cpp 同位必命中。
    let tp_index = *self
      .tp_to_index
      .find(&tp)
      .expect("上方登记块蕴含键恒在，取回必命中");

    self.link_child(tp_index, parent_index, link_name);

    self.visit_children_type_pack_id_i32(tp, tp_index);
  }
}
