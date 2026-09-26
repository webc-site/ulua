use crate::{
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    normalized_extern_type::NormalizedExternType, normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType, type_ids::TypeIds,
  },
  type_aliases::{normalized_tyvars::NormalizedTyvars, type_id::TypeId},
};

#[derive(Debug, Clone)]
pub struct NormalizedType {
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) tops: TypeId,
  pub(crate) booleans: TypeId,
  pub(crate) extern_types: NormalizedExternType,
  pub(crate) errors: TypeId,
  pub(crate) nils: TypeId,
  pub(crate) numbers: TypeId,
  pub(crate) integers: TypeId,
  pub(crate) strings: NormalizedStringType,
  pub(crate) threads: TypeId,
  pub(crate) buffers: TypeId,
  pub(crate) tables: TypeIds,
  pub(crate) functions: NormalizedFunctionType,
  pub(crate) tyvars: NormalizedTyvars,
  pub(crate) is_cacheable: bool,
}

// Safety: 本类型不含自引用指针，仅 `builtin_types: Handle<BuiltinTypes>`（借用自
// 长生命周期共享 `BuiltinTypes` 单例的 NonNull 句柄，令自动 Send/Sync 失效）与各
// `TypeId`/`TypeIds` 字段
// （借用自类型 arena 的 arena 句柄）令自动 Send 失效；跨线程移动只转移这些
// 身份/查找值，被借用对象由持有方保证比 `NormalizedType` 长寿时即可靠。
unsafe impl Send for NormalizedType {}
// Safety: 同上：`builtin_types` 与各 arena id 仅被只读访问以构造规范化视图，
// 共享 `&NormalizedType` 在借用对象有效期间不引入数据竞争（写访问只经
// `&mut`，与共享借用互斥由类型系统在借用窗口内维持）。
unsafe impl Sync for NormalizedType {}
