use ulua_common::{functions::format_append::format_append, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack, state_dot::StateDot,
    type_pack::TypePack, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
  },
};
impl StateDot {
  pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, index: i32) {
    if self.seen_tp.contains(&tp) {
      return;
    }
    self.seen_tp.insert(tp);

    self.start_node(index);
    self.start_node_label();

    if let Some(btp) = get_type_pack_id::<BoundTypePack>(tp) {
      format_append(&mut self.result, format_args!("BoundTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      self.visit_child_type_pack_id(btp.bound_to, index, None);
    } else if let Some(tpp) = get_type_pack_id::<TypePack>(tp) {
      format_append(&mut self.result, format_args!("TypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      for tv in &tpp.head {
        self.visit_child_type_id(*tv, index, None);
      }
      if let Some(tail) = tpp.tail {
        self.visit_child_type_pack_id(tail, index, Some("tail"));
      }
    } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
      format_append(
        &mut self.result,
        format_args!(
          "VariadicTypePack {}{}",
          if vtp.hidden { "hidden " } else { "" },
          index
        ),
      );
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      self.visit_child_type_id(vtp.ty, index, None);
    } else if get_type_pack_id::<FreeTypePack>(tp).is_some() {
      format_append(&mut self.result, format_args!("FreeTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if let Some(gtp) = get_type_pack_id::<GenericTypePack>(tp) {
      if gtp.explicit_name {
        format_append(
          &mut self.result,
          format_args!("GenericTypePack {}", gtp.name),
        );
      } else {
        format_append(&mut self.result, format_args!("GenericTypePack {}", index));
      }
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if get_type_pack_id::<ErrorTypePack>(tp).is_some() {
      format_append(&mut self.result, format_args!("ErrorTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else {
      LUAU_ASSERT!(false);
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    }
  }
}
