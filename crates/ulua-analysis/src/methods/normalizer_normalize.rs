use alloc::sync::Arc;
use core::ptr::null;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  enums::normalization_result::NormalizationResult,
  methods::fresh_normalized_type::fresh_normalized_type,
  records::{
    fuel_initializer::FuelInitializer, normalized_type::NormalizedType, normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
  },
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};
impl Normalizer {
  pub fn try_normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    match catch_unwind(AssertUnwindSafe(|| self.normalize_uncaught(ty))) {
      Ok(norm) => norm,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => None,
      Err(payload) => resume_unwind(payload),
    }
  }

  fn normalize_uncaught(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    if self.arena.is_none() {
      // 契约：这是 arena 缺失（模块外归一化，null 哨兵 = Option::None）的内部
      // 错误上报路径：shared_state 由构造实参或 TypeChecker 定址后接线（使用前
      // 断言，等价原裸指针解引用）；其 ice_handler 字段同样在 UnifierSharedState
      // 构造期接线非空。这里只读取指针并调用 ice_string（仅消费 'static 字符串，
      // 不保留指针），无别名冲突。
      let ice = self.shared_state_ref().ice_handler;
      unsafe {
        (*ice).ice_string("Normalizing types outside a module");
      }
    }

    if let Some(shared) = self.cached_normals.get(&ty) {
      return Some(shared.clone());
    }

    let mut norm = fresh_normalized_type(self.builtin_types);
    let mut seen_set_types: DenseHashSet<TypeId> = DenseHashSet::default();
    let mut seen_table_prop_pairs: SeenTablePropPairs = SeenTablePropPairs::new((null(), null()));

    // FuelInitializer handles initializing and tearing down normalization fuel limits.
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };
    // Safety: fuel_initializer_not_null_normalizer 的契约是 normalizer 指针非空：
    // self as *mut 派生自当前 &mut self，必非空且对齐，指针仅在调用期内有效；
    // fi 经 _fi 绑定持有至函数返回（与 C++ FuelInitializer 栈对象对齐），构造与
    // Drop 间不会再有对 self 的第二条存活借用。
    unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };
    // 必须绑定命名变量：`let _ = fi` 会立即 drop，FuelInitializer 析构将刚初始化的
    // fuel 清空，导致整个 normalize 子树燃料计量失效（对齐 cpp `FuelInitializer fi{...}`）。
    let _fi = fi;

    let res = self.union_normal_with_ty(
      &mut norm,
      ty,
      &mut seen_table_prop_pairs,
      &mut seen_set_types,
      -1,
    );

    if res != NormalizationResult::True {
      return None;
    }

    if norm.is_unknown() {
      self.clear_normal(&mut norm);
      // Safety: builtin_types 由 Normalizer::new 的 C++ NotNull 形参接线，指向进程/
      // 模块级 BuiltinTypes，非空且比 normalizer 长寿；仅重建只读借用取
      // unknown_type 的 TypeId（arena 裸指针值），单线程内无并发写。
      norm.tops = self.builtin_types.get().unknown_type;
    }

    let shared = Arc::new(norm);

    if shared.is_cacheable {
      self.cached_normals.insert(ty, shared.clone());
    }

    Some(shared)
  }
}
