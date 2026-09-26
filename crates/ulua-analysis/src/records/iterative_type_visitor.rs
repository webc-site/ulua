use alloc::{string::String, vec::Vec};

use crate::{
  functions::{follow_type, get_type::type_variant_of, get_type_pack::type_pack_variant_of},
  macros::visit_type_delegators,
  records::{
    any_type::AnyType, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    extern_type::ExternType, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType, no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType, primitive_type::PrimitiveType,
    singleton_type::SingletonType, table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    union_type::UnionType, unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
    work_item_iterative_type_visitor::WorkItem,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, error_type::ErrorType,
    error_type_pack::ErrorTypePack, seen_set_iterative_type_visitor::SeenSet, type_id::TypeId,
    type_pack_id::TypePackId, type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
#[derive(Debug, Clone)]
pub struct IterativeTypeVisitor {
  pub(crate) seen: SeenSet,
  pub(crate) work_queue: Vec<WorkItem>,
  pub(crate) parent_cursor: i32,
  pub(crate) work_cursor: u32,
  pub(crate) visitor_name: String,
  pub(crate) skip_bound_types: bool,
  pub(crate) visit_once: bool,
}

/// 解引用并克隆类型变体。裸指针解引用收口在
/// [`crate::functions::get_type::type_variant_of`]（arena 节点有效性契约同 `get`），
/// 此处仅克隆一份供分派匹配、不持有引用。
fn type_variant(ty: TypeId) -> TypeVariant {
  type_variant_of(ty).clone()
}

/// 同 [`type_variant`]，类型包侧。
fn pack_variant(tp: TypePackId) -> TypePackVariant {
  type_pack_variant_of(tp).clone()
}

impl Default for IterativeTypeVisitor {
  fn default() -> Self {
    Self {
      seen: SeenSet::default(),
      work_queue: Vec::new(),
      parent_cursor: -1,
      work_cursor: 0,
      visitor_name: String::new(),
      skip_bound_types: false,
      visit_once: true,
    }
  }
}

pub trait IterativeTypeVisitorTrait {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor;

  fn cycle_type_id(&mut self, _ty: TypeId) {}
  fn cycle_type_pack_id(&mut self, _tp: TypePackId) {}

  fn visit_type_id(&mut self, _ty: TypeId) -> bool {
    true
  }
  fn visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    true
  }

  // 28 个按变体重载的默认方法由共享宏生成，逐条转发到上面的裸 visit，
  // 与 C++ `IterativeTypeVisitor` 的默认实现等价。
  visit_type_delegators!();

  /// 重置工作队列状态（C++ `run` 入口的 `{parentCursor=-1; workCursor=0;
  /// workQueue.clear()}`）。
  fn reset_work_queue(&mut self) {
    let base = self.visitor_base();
    base.parent_cursor = -1;
    base.work_cursor = 0;
    base.work_queue.clear();
  }

  fn run_type_id(&mut self, root_ty: TypeId) {
    self.reset_work_queue();

    self.traverse_type_id(root_ty);
    self.process_work_queue();
  }

  fn run_type_pack_id(&mut self, root_tp: TypePackId) {
    self.reset_work_queue();

    self.traverse_type_pack_id(root_tp);
    self.process_work_queue();
  }

  fn traverse_type_id(&mut self, ty: TypeId) {
    let parent = self.visitor_base().parent_cursor;
    self
      .visitor_base()
      .work_queue
      .push(WorkItem::work_item_type_id_i32(ty, parent));
  }

  fn traverse_type_pack_id(&mut self, tp: TypePackId) {
    let parent = self.visitor_base().parent_cursor;
    self
      .visitor_base()
      .work_queue
      .push(WorkItem::work_item_type_pack_id_i32(tp, parent));
  }

  fn process_work_queue(&mut self) {
    loop {
      let item = {
        let base = self.visitor_base();
        if (base.work_cursor as usize) >= base.work_queue.len() {
          break;
        }

        base.parent_cursor = base.work_cursor as i32;
        base.work_queue[base.work_cursor as usize].clone()
      };

      if let Some(ty) = item.type_id() {
        if self.is_cyclic_type_id(ty) {
          self.cycle_type_id(ty);
        } else {
          self.process_type_id(ty);
        }
      } else if let Some(tp) = item.type_pack_id() {
        if self.is_cyclic_type_pack_id(tp) {
          self.cycle_type_pack_id(tp);
        } else {
          self.process_type_pack_id(tp);
        }
      } else {
        ulua_common::LUAU_ASSERT!(false);
      }

      self.visitor_base().work_cursor += 1;
    }
  }

  /// 与 [`Self::process_type_pack_id`] 对称：`unsafe` 收敛在函数体内的裸指针
  /// 解引用处，入队项由 `process_work_queue` 保证来自已注册的 TypeId。
  fn process_type_id(&mut self, mut ty: TypeId) {
    if self.visitor_base().skip_bound_types {
      ty = follow_type::follow(ty);
    }

    if self.has_seen(ty as *const ()) {
      return;
    }

    let variant = type_variant(ty);

    match variant {
      TypeVariant::Bound(bound_to) => {
        let btv = BoundType { bound_to };
        ulua_common::LUAU_ASSERT!(!self.visitor_base().skip_bound_types);
        if self.visit_type_id_bound_type(ty, &btv) {
          self.traverse_type_id(bound_to);
        }
      }
      TypeVariant::Free(ftv) => {
        if self.visit_type_id_free_type(ty, &ftv) {
          ulua_common::LUAU_ASSERT!(!ftv.lower_bound.is_null());
          ulua_common::LUAU_ASSERT!(!ftv.upper_bound.is_null());
          self.traverse_type_id(ftv.lower_bound);
          self.traverse_type_id(ftv.upper_bound);
        }
      }
      TypeVariant::Generic(gtv) => {
        self.visit_type_id_generic_type(ty, &gtv);
      }
      TypeVariant::Error(etv) => {
        self.visit_type_id_error_type(ty, &etv);
      }
      TypeVariant::Primitive(ptv) => {
        self.visit_type_id_primitive_type(ty, &ptv);
      }
      TypeVariant::Function(ftv) => {
        if self.visit_type_id_function_type(ty, &ftv) {
          self.traverse_type_pack_id(ftv.arg_types);
          self.traverse_type_pack_id(ftv.ret_types);
        }
      }
      TypeVariant::Table(ttv) => {
        ulua_common::LUAU_ASSERT!(!self.visitor_base().skip_bound_types || ttv.bound_to.is_none());
        if self.visitor_base().skip_bound_types
          && let Some(bound_to) = ttv.bound_to
        {
          self.traverse_type_id(bound_to);
        } else if self.visit_type_id_table_type(ty, &ttv) {
          if let Some(bound_to) = ttv.bound_to {
            self.traverse_type_id(bound_to);
          } else {
            for prop in ttv.props.values() {
              if let Some(read_ty) = prop.read_ty {
                self.traverse_type_id(read_ty);
              }
              if let Some(write_ty) = prop.write_ty
                && !prop.is_shared()
              {
                self.traverse_type_id(write_ty);
              }
            }

            if let Some(indexer) = &ttv.indexer {
              self.traverse_type_id(indexer.index_type);
              self.traverse_type_id(indexer.index_result_type);
            }
          }
        }
      }
      TypeVariant::Metatable(mtv) => {
        if self.visit_type_id_metatable_type(ty, &mtv) {
          self.traverse_type_id(mtv.table);
          self.traverse_type_id(mtv.metatable);
        }
      }
      TypeVariant::Extern(etv) => {
        if self.visit_type_id_extern_type(ty, &etv) {
          for prop in etv.props.values() {
            if let Some(read_ty) = prop.read_ty {
              self.traverse_type_id(read_ty);
            }
            if let Some(write_ty) = prop.write_ty
              && !prop.is_shared()
            {
              self.traverse_type_id(write_ty);
            }
          }

          if let Some(parent) = etv.parent {
            self.traverse_type_id(parent);
          }
          if let Some(metatable) = etv.metatable {
            self.traverse_type_id(metatable);
          }
          if let Some(indexer) = &etv.indexer {
            self.traverse_type_id(indexer.index_type);
            self.traverse_type_id(indexer.index_result_type);
          }
        }
      }
      TypeVariant::Any(atv) => {
        self.visit_type_id_any_type(ty, &atv);
      }
      TypeVariant::NoRefine(nrt) => {
        self.visit_type_id_no_refine_type(ty, &nrt);
      }
      TypeVariant::Union(utv) => {
        if self.visit_type_id_union_type(ty, &utv) {
          let mut union_changed = false;
          for opt_ty in utv.options {
            self.traverse_type_id(opt_ty);
            // follow_type_id 沿 Bound/Lazy 链解析回存活的代表 Type 节点，
            // type_variant_of 只读取其判别式（收口于 arena 边界，与 `get` 同契约）。
            if !matches!(
              type_variant_of(follow_type::follow(ty)),
              TypeVariant::Union(_)
            ) {
              union_changed = true;
              break;
            }
          }

          if union_changed {
            self.traverse_type_id(ty);
          }
        }
      }
      TypeVariant::Intersection(itv) => {
        if self.visit_type_id_intersection_type(ty, &itv) {
          let mut intersection_changed = false;
          for part_ty in itv.parts {
            self.traverse_type_id(part_ty);
            // 同 union 分支：type_variant_of 只读取 follow 结果的判别式（arena 边界契约）。
            if !matches!(
              type_variant_of(follow_type::follow(ty)),
              TypeVariant::Intersection(_)
            ) {
              intersection_changed = true;
              break;
            }
          }

          if intersection_changed {
            self.traverse_type_id(ty);
          }
        }
      }
      TypeVariant::Lazy(ltv) => {
        if !ltv.unwrapped.is_null() {
          self.traverse_type_id(ltv.unwrapped);
        }
      }
      TypeVariant::Singleton(stv) => {
        self.visit_type_id_singleton_type(ty, &stv);
      }
      TypeVariant::Blocked(btv) => {
        self.visit_type_id_blocked_type(ty, &btv);
      }
      TypeVariant::Unknown(utv) => {
        self.visit_type_id_unknown_type(ty, &utv);
      }
      TypeVariant::Never(ntv) => {
        self.visit_type_id_never_type(ty, &ntv);
      }
      TypeVariant::PendingExpansion(petv) => {
        if self.visit_type_id_pending_expansion_type(ty, &petv) {
          for a in petv.type_arguments {
            self.traverse_type_id(a);
          }
          for a in petv.pack_arguments {
            self.traverse_type_pack_id(a);
          }
        }
      }
      TypeVariant::Negation(ntv) => {
        if self.visit_type_id_negation_type(ty, &ntv) {
          self.traverse_type_id(ntv.ty);
        }
      }
      TypeVariant::TypeFunctionInstance(tfit) => {
        if self.visit_type_id_type_function_instance_type(ty, &tfit) {
          for &p in &tfit.type_arguments {
            self.traverse_type_id(p);
          }
          for &p in &tfit.pack_arguments {
            self.traverse_type_pack_id(p);
          }
        }
      }
    }

    self.unsee(ty as *const ());
  }

  fn process_type_pack_id(&mut self, tp: TypePackId) {
    if self.has_seen(tp as *const ()) {
      return;
    }

    let variant = pack_variant(tp);

    match variant {
      TypePackVariant::Bound(bound_to) => {
        let btp = BoundTypePack { bound_to };
        if self.visit_type_pack_id_bound_type_pack(tp, &btp) {
          self.traverse_type_pack_id(bound_to);
        }
      }
      TypePackVariant::Free(ftp) => {
        self.visit_type_pack_id_free_type_pack(tp, &ftp);
      }
      TypePackVariant::Generic(gtp) => {
        self.visit_type_pack_id_generic_type_pack(tp, &gtp);
      }
      TypePackVariant::Error(etp) => {
        self.visit_type_pack_id_error_type_pack(tp, &etp);
      }
      TypePackVariant::TypePack(pack) => {
        if self.visit_type_pack_id_type_pack(tp, &pack) {
          for ty in pack.head {
            self.traverse_type_id(ty);
          }
          if let Some(tail) = pack.tail {
            self.traverse_type_pack_id(tail);
          }
        }
      }
      TypePackVariant::Variadic(vtp) => {
        if self.visit_type_pack_id_variadic_type_pack(tp, &vtp) {
          self.traverse_type_id(vtp.ty);
        }
      }
      TypePackVariant::Blocked(btp) => {
        self.visit_type_pack_id_blocked_type_pack(tp, &btp);
      }
      TypePackVariant::TypeFunctionInstance(tfitp) => {
        if self.visit_type_pack_id_type_function_instance_type_pack(tp, &tfitp) {
          for &t in &tfitp.type_arguments {
            self.traverse_type_id(t);
          }
          for &t in &tfitp.pack_arguments {
            self.traverse_type_pack_id(t);
          }
        }
      }
    }

    self.unsee(tp as *const ());
  }

  fn has_seen(&mut self, tv: *const ()) -> bool {
    if !self.visitor_base().visit_once {
      return false;
    }

    let base = self.visitor_base();
    let is_fresh = !base.seen.contains(&tv);
    base.seen.insert(tv);
    !is_fresh
  }

  /// C++ `unsee`：`if (!visitOnce) seen.erase(tv)`。visitOnce=false 时
  /// `has_seen` 从不插入，erase 恒为空操作，故此处退化为无操作。
  fn unsee(&mut self, _tv: *const ()) {}

  /// C++ `isCycle(TID)` / `isCycle(TypePackId)` 共用的父链回溯：沿
  /// `parent` 游标向上找队列祖先，命中判别式即判环。
  fn is_cyclic_in_queue(&mut self, eq: impl Fn(&WorkItem) -> bool) -> bool {
    let base = self.visitor_base();
    let mut cursor = base.work_cursor as i32;
    let mut item = &base.work_queue[base.work_cursor as usize];

    while item.parent >= 0 {
      ulua_common::LUAU_ASSERT!(item.parent < cursor);
      cursor = item.parent;
      item = &base.work_queue[cursor as usize];

      if eq(item) {
        return true;
      }
    }

    false
  }

  fn is_cyclic_type_id(&mut self, ty: TypeId) -> bool {
    self.is_cyclic_in_queue(|item| item.type_id() == Some(ty))
  }

  fn is_cyclic_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.is_cyclic_in_queue(|item| item.type_pack_id() == Some(tp))
  }
}

impl IterativeTypeVisitorTrait for IterativeTypeVisitor {
  fn visitor_base(&mut self) -> &mut IterativeTypeVisitor {
    self
  }
}
