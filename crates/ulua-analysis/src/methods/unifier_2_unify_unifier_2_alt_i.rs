use ulua_common::FFlag;

use crate::{
  enums::{table_state::TableState, unify_result::UnifyResult},
  records::{table_indexer::TableIndexer, table_type::TableType, unifier_2::Unifier2},
};

impl Unifier2 {
  pub fn unify_table_type_table_type(
    &mut self,
    sub_table: &mut TableType,
    super_table: &TableType,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for (prop_name, sub_prop) in &sub_table.props {
      if let Some(super_prop_opt) = super_table.props.get(prop_name) {
        let super_prop = super_prop_opt;

        if let (Some(sub_read), Some(super_read)) = (sub_prop.read_ty, super_prop.read_ty) {
          result &= self.unify_type_id_type_id(sub_read, super_read);
        }

        // C++ 写属性方向反转：`unify_(*superProp.writeTy, *subProp.writeTy)`（写逆变）。
        if let (Some(sub_write), Some(super_write)) = (sub_prop.write_ty, super_prop.write_ty) {
          result &= self.unify_type_id_type_id(super_write, sub_write);
        }
      }
    }

    let mut sub_type_params_iter = sub_table.instantiated_type_params.iter();
    let mut super_type_params_iter = super_table.instantiated_type_params.iter();

    while let (Some(sub_tp), Some(super_tp)) =
      (sub_type_params_iter.next(), super_type_params_iter.next())
    {
      result &= self.unify_type_id_type_id(*sub_tp, *super_tp);
    }

    let mut sub_type_pack_params_iter = sub_table.instantiated_type_pack_params.iter();
    let mut super_type_pack_params_iter = super_table.instantiated_type_pack_params.iter();

    while let (Some(sub_tpp), Some(super_tpp)) = (
      sub_type_pack_params_iter.next(),
      super_type_pack_params_iter.next(),
    ) {
      result &= self.unify_type_pack_id_type_pack_id(*sub_tpp, *super_tpp);
    }

    if let (Some(sub_indexer), Some(super_indexer)) = (&sub_table.indexer, &super_table.indexer) {
      result &= self.unify_type_id_type_id(sub_indexer.index_type, super_indexer.index_type);
      result &= self.unify_type_id_type_id(
        sub_indexer.index_result_type,
        super_indexer.index_result_type,
      );

      result &= self.unify_type_id_type_id(super_indexer.index_type, sub_indexer.index_type);
      result &= self.unify_type_id_type_id(
        super_indexer.index_result_type,
        sub_indexer.index_result_type,
      );
    }

    if sub_table.indexer.is_none()
      && sub_table.state == TableState::Unsealed
      && let Some(super_indexer) = super_table.indexer.as_ref()
    {
      // FFlag 开启时走 instantiateWithBoundTypes，避免把泛型裸漏进 unsealed 表的
      // indexer（Unifier2.cpp:598 `LuauDoNotLeakGenericsInIndexer`）。
      let (index_type, index_result_type) = if FFlag::LUAU_DO_NOT_LEAK_GENERICS_IN_INDEXER.get() {
        (
          self.instantiate_with_bound_types(super_indexer.index_type),
          self.instantiate_with_bound_types(super_indexer.index_result_type),
        )
      } else {
        let mut index_type = super_indexer.index_type;
        if let Some(subst) = self.generic_substitutions.find(&index_type) {
          index_type = *subst;
        }

        let mut index_result_type = super_indexer.index_result_type;
        if let Some(subst) = self.generic_substitutions.find(&index_result_type) {
          index_result_type = *subst;
        }
        (index_type, index_result_type)
      };

      sub_table.indexer = Some(TableIndexer {
        index_type,
        index_result_type,
        is_read_only: false,
      });
    }

    result
  }
}
