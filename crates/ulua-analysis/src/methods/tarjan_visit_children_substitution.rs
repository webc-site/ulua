use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{get_type, get_type_pack},
  records::{
    extern_type::ExternType, function_type::FunctionType, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType,
    pending_expansion_type::PendingExpansionType, table_type::TableType, tarjan::Tarjan,
    r#type::Type, type_function_instance_type::TypeFunctionInstanceType, type_pack::TypePack,
    type_pack_var::TypePackVar, union_type::UnionType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{nominal_relation::NominalRelation, type_id::TypeId, type_pack_id::TypePackId},
};

impl Tarjan {
  pub fn visit_children_type_id_i32(&mut self, ty: TypeId, _index: i32) {
    let mut ty = ty;
    // Safety: self.log 由 clear_tarjan/reset_state 在每轮 substitute 前接线，取值只
    // 可能是调用方存活的 &TxnLog 或 TxnLog::empty() 进程级单例指针，均非空且比本次
    // 遍历长寿；follow_type_id 按 &self 只读。
    unsafe {
      LUAU_ASSERT!(ty == (*self.log).follow_type_id(ty));
    }

    if self.ignore_children_visit_type_id(ty) {
      return;
    }

    // Safety: 同上，self.log 非空存活；pending_type_id 沿 parent 链只读查找，返回值
    // 要么是日志映射中 Box<PendingType> 堆对象的指针、要么是 null。
    let pty = unsafe { (*self.log).pending_type_id(ty) };
    if !pty.is_null() {
      // Safety: pty 指向 Box<PendingType> 的堆对象——容器 rehash 只移动 Box 指针不
      // 移动堆内容，且本轮遍历期间不回滚该日志，pending 字段保持存活；仅取共享引用。
      ty = unsafe { &(*pty).pending as *const Type };
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty) {
      for generic in ftv.generics.iter() {
        self.visit_child_type_id(*generic);
      }
      for generic_pack in ftv.generic_packs.iter() {
        self.visit_child_type_pack_id(*generic_pack);
      }

      self.visit_child_type_pack_id(ftv.arg_types);
      self.visit_child_type_pack_id(ftv.ret_types);
      return;
    }

    if let Some(ttv) = get_type::get::<TableType>(ty) {
      LUAU_ASSERT!(ttv.bound_to.is_none());
      for prop in ttv.props.values() {
        self.visit_child_optional_ty(prop.read_ty);
        self.visit_child_optional_ty(prop.write_ty);
      }

      if let Some(ref indexer) = ttv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      for itp in ttv.instantiated_type_params.iter() {
        self.visit_child_type_id(*itp);
      }

      for itp in ttv.instantiated_type_pack_params.iter() {
        self.visit_child_type_pack_id(*itp);
      }
      return;
    }

    if let Some(mtv) = get_type::get::<MetatableType>(ty) {
      self.visit_child_type_id(mtv.table);
      self.visit_child_type_id(mtv.metatable);
      return;
    }

    if let Some(utv) = get_type::get::<UnionType>(ty) {
      for opt in utv.options.iter() {
        self.visit_child_type_id(*opt);
      }
      return;
    }

    if let Some(itv) = get_type::get::<IntersectionType>(ty) {
      for part in itv.parts.iter() {
        self.visit_child_type_id(*part);
      }
      return;
    }

    if let Some(petv) = get_type::get::<PendingExpansionType>(ty) {
      for a in petv.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in petv.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(tfit) = get_type::get::<TypeFunctionInstanceType>(ty) {
      for a in tfit.type_arguments.iter() {
        self.visit_child_type_id(*a);
      }
      for a in tfit.pack_arguments.iter() {
        self.visit_child_type_pack_id(*a);
      }
      return;
    }

    if let Some(etv) = get_type::get::<ExternType>(ty) {
      for prop in etv.props.values() {
        if prop.read_ty.is_some() {
          self.visit_child_optional_ty(prop.read_ty);
        }
        if prop.write_ty.is_some() {
          self.visit_child_optional_ty(prop.write_ty);
        }
      }

      if let Some(parent) = etv.parent {
        self.visit_child_type_id(parent);
      }

      if let Some(metatable) = etv.metatable {
        self.visit_child_type_id(metatable);
      }

      if let Some(ref indexer) = etv.indexer {
        self.visit_child_type_id(indexer.index_type);
        self.visit_child_type_id(indexer.index_result_type);
      }

      if fflag::DebugLuauUserDefinedClasses.get()
        && let Some(ref relation) = etv.relation
      {
        match relation {
          NominalRelation::V0(obj) => {
            self.visit_child_type_id(obj.ty);
          }
          NominalRelation::V1(klass) => {
            self.visit_child_type_id(klass.ty);
          }
        }
      }
      return;
    }

    if let Some(ntv) = get_type::get::<NegationType>(ty) {
      self.visit_child_type_id(ntv.ty);
    }
  }

  pub fn visit_children_type_pack_id_i32(&mut self, tp: TypePackId, _index: i32) {
    let mut tp = tp;
    // Safety: 与 TypeId 版同源——self.log 每轮 substitute 前由 reset_state 接线为
    // 存活 &TxnLog 或 TxnLog::empty() 单例，非空且长寿；follow_type_pack_id 只读。
    unsafe {
      LUAU_ASSERT!(tp == (*self.log).follow_type_pack_id(tp));
    }

    if self.ignore_children_visit_type_pack_id(tp) {
      return;
    }

    // Safety: self.log 非空存活（同上）；pending_type_pack_id 沿 parent 链只读查找，
    // 返回 Box<PendingTypePack> 堆对象指针或 null。
    let ptp = unsafe { (*self.log).pending_type_pack_id(tp) };
    if !ptp.is_null() {
      // Safety: ptp 指向 Box 堆对象，容器重排不移动堆内容；遍历期间日志不回滚，
      // pending 字段存活，仅取共享引用。
      tp = unsafe { &(*ptp).pending as *const TypePackVar };
    }

    if let Some(tpp) = get_type_pack::get::<TypePack>(tp) {
      for tv in tpp.head.iter() {
        self.visit_child_type_id(*tv);
      }
      if let Some(tail) = tpp.tail {
        self.visit_child_type_pack_id(tail);
      }
      return;
    }

    if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tp) {
      self.visit_child_type_id(vtp.ty);
    }
  }
}
