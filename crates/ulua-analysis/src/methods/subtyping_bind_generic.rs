use core::marker::PhantomData;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  methods::dense_hash_table_find::{dense_hash_table_find, dense_hash_table_find_mut},
  records::{
    dense_hash_map::DenseHashMap,
    dense_hash_table::{DenseDefault, DenseEq, DenseHashTable, DenseHasher, ItemInterface},
  },
};

use crate::{
  functions::{follow_type, get_type},
  records::{
    generic_bounds::GenericBounds, generic_type::GenericType, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment,
  },
  type_aliases::type_id::TypeId,
};
impl DenseDefault for GenericBounds {
  fn dense_default() -> Self {
    GenericBounds::default()
  }
}

struct ItemInterfaceMapNoDefault<K, V>(PhantomData<(K, V)>);

impl<K: Clone, V> ItemInterface<K, (K, V)> for ItemInterfaceMapNoDefault<K, V> {
  fn get_key(item: &(K, V)) -> &K {
    &item.0
  }

  fn set_key(item: &mut (K, V), key: K) {
    item.0 = key;
  }

  fn make_empty(_empty_key: &K) -> (K, V) {
    unreachable!("find does not construct empty DenseHashMap entries")
  }
}

type DenseHashMapTable<K, V, H, E> =
  DenseHashTable<K, (K, V), ItemInterfaceMapNoDefault<K, V>, H, E>;

pub(crate) fn dense_hash_map_find_no_default<'a, K, V, H, E>(
  map: &'a DenseHashMap<K, V, H, E>,
  key: &K,
) -> Option<&'a V>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  // Safety: DenseHashMap 是仅含单字段 impl_: DenseHashTable<K,(K,V),ItemInterfaceMap,H,E>
  // 的包装，基址与表重合；DenseHashTable 的 Iface 参数仅是 PhantomData（零大小、不
  // 参与布局），换成 ItemInterfaceMapNoDefault 后字段序列与 (K,V) 条目类型完全一致，
  // 仅切换无默认值的查找语义。map 由上层借用传入，期间存活且此处取只读引用，无别名
  // 写者。
  let table =
    unsafe { &*(map as *const DenseHashMap<K, V, H, E> as *const DenseHashMapTable<K, V, H, E>) };
  dense_hash_table_find(table, key).map(|item| &item.1)
}

pub(crate) fn dense_hash_map_find_mut_no_default<'a, K, V, H, E>(
  map: &'a mut DenseHashMap<K, V, H, E>,
  key: &K,
) -> Option<&'a mut V>
where
  K: Clone,
  H: DenseHasher<K> + Default,
  E: DenseEq<K> + Default,
{
  // Safety: 与只读版同一布局论证（单字段包装基址重合、Iface 为 PhantomData 不参与
  // 布局、条目类型同为 (K,V)）；map 是调用方独占的 &mut，重建 &mut 借用不产生别名。
  let table =
    unsafe { &mut *(map as *mut DenseHashMap<K, V, H, E> as *mut DenseHashMapTable<K, V, H, E>) };
  dense_hash_table_find_mut(table, key).map(|item| &mut item.1)
}

impl Subtyping {
  pub fn bind_generic(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
  ) -> bool {
    let sub_ty = follow_type::follow(sub_ty);
    let super_ty = follow_type::follow(super_ty);
    let mut original_sub_ty_bounds: Option<GenericBounds> = None;

    let super_bounds_snapshot = dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
      .and_then(|bounds| bounds.last().cloned());

    let sub_has_local_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &sub_ty)
      .is_some_and(|bounds| !bounds.is_empty());

    if sub_has_local_bounds {
      LUAU_ASSERT!(get_type::get::<GenericType>(sub_ty).is_some());

      let sub_bounds = dense_hash_map_find_mut_no_default(&mut env.mapped_generics, &sub_ty)
        .expect("sub_has_local_bounds 判据即同键 find==Some 且非空，中间无删改");
      let sub_bounds_back = sub_bounds.last_mut().expect("判据已含 !is_empty()");
      original_sub_ty_bounds = Some(sub_bounds_back.clone());

      let upper_sub_bounds = &mut sub_bounds_back.upper_bound;

      if let Some(super_bounds) = &super_bounds_snapshot {
        LUAU_ASSERT!(get_type::get::<GenericType>(super_ty).is_some());

        self.maybe_update_bounds(
          sub_ty,
          super_ty,
          upper_sub_bounds,
          &super_bounds.lower_bound,
          &super_bounds.upper_bound,
        );
      } else {
        upper_sub_bounds.insert_type_id(super_ty);
      }
    } else if env.contains_mapped_type(sub_ty) {
      // Safety: self.ice_reporter.as_ptr() 由 Subtyping 构造按 C++ NotNull<InternalErrorReporter>
      // 契约接线为非空裸指针，指向比本求解器长寿的报告器；ice_string 仅 &self 读取。
      {
        self
          .ice_reporter
          .get()
          .ice_string("attempting to modify bounds of a potentially visited generic");
      }
    }

    let super_has_local_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
      .is_some_and(|bounds| !bounds.is_empty());

    if super_has_local_bounds {
      LUAU_ASSERT!(get_type::get::<GenericType>(super_ty).is_some());

      let super_bounds = dense_hash_map_find_mut_no_default(&mut env.mapped_generics, &super_ty)
        .expect("super_has_local_bounds 判据即同键 find==Some 且非空，中间无删改");
      let super_bounds_back = super_bounds.last_mut().expect("判据已含 !is_empty()");
      let lower_super_bounds = &mut super_bounds_back.lower_bound;

      if let Some(original_sub_ty_bounds) = original_sub_ty_bounds {
        LUAU_ASSERT!(get_type::get::<GenericType>(sub_ty).is_some());

        self.maybe_update_bounds(
          super_ty,
          sub_ty,
          lower_super_bounds,
          &original_sub_ty_bounds.upper_bound,
          &original_sub_ty_bounds.lower_bound,
        );
      } else {
        lower_super_bounds.insert_type_id(sub_ty);
      }
    } else if env.contains_mapped_type(super_ty) {
      // Safety: 同 sub 分支——ice_reporter 为构造时接线的 NotNull 存活报告器，
      // ice_string 只读报告，无别名冲突。
      {
        self
          .ice_reporter
          .get()
          .ice_string("attempting to modify bounds of a potentially visited generic");
      }
    }

    true
  }
}
