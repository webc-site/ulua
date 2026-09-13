use core::ptr::null;

use ulua_common::{functions::format_append::formatAppend, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::get_type_pack::get_type_pack_id,
  records::{
    error_type_pack::ErrorTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack, state_dot::StateDot, type_pack::TypePack,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{bound_type_pack::BoundTypePack, type_pack_id::TypePackId},
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
      formatAppend(&mut self.result, format_args!("BoundTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      unsafe { self.visit_child_type_pack_id_i32_c_char(btp.bound_to, index, null()) };
    } else if let Some(tpp) = get_type_pack_id::<TypePack>(tp) {
      formatAppend(&mut self.result, format_args!("TypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      for tv in &tpp.head {
        unsafe { self.visit_child_type_id_i32_c_char(*tv, index, null()) };
      }
      if let Some(tail) = tpp.tail {
        unsafe { self.visit_child_type_pack_id_i32_c_char(tail, index, c"tail".as_ptr()) };
      }
    } else if let Some(vtp) = get_type_pack_id::<VariadicTypePack>(tp) {
      formatAppend(
        &mut self.result,
        format_args!(
          "VariadicTypePack {}{}",
          if vtp.hidden { "hidden " } else { "" },
          index
        ),
      );
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();

      unsafe { self.visit_child_type_id_i32_c_char(vtp.ty, index, null()) };
    } else if get_type_pack_id::<FreeTypePack>(tp).is_some() {
      formatAppend(&mut self.result, format_args!("FreeTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if let Some(gtp) = get_type_pack_id::<GenericTypePack>(tp) {
      if gtp.explicit_name {
        formatAppend(
          &mut self.result,
          format_args!("GenericTypePack {}", gtp.name),
        );
      } else {
        formatAppend(&mut self.result, format_args!("GenericTypePack {}", index));
      }
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else if get_type_pack_id::<ErrorTypePack>(tp).is_some() {
      formatAppend(&mut self.result, format_args!("ErrorTypePack {}", index));
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    } else {
      LUAU_ASSERT!(false);
      self.finish_node_label_type_pack_id(tp);
      self.finish_node();
    }
  }
}
