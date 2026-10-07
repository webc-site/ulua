use alloc::sync::Weak;
use core::ptr::null_mut;

use ulua_ast::records::ast_stat_type_function::AstStatTypeFunction;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{module::Module, type_fun::TypeFun},
  type_aliases::name_type::Name,
};
#[derive(Debug, Clone)]
pub struct UserDefinedFunctionData {
  /// Store a weak module reference to ensure the lifetime requirements are preserved
  pub(crate) owner: Weak<Module>,

  /// References to AST elements are owned by the Module allocator which also stores this type
  /// §2(b)：cpp `UserDefinedFunctionData { AstStatTypeFunction* definition; }`。本字段
  /// 作**指针身份键**贯穿一整组裸指针容器/边界：读侧把 `definition` 原样写入
  /// `environment_function: DenseHashMap<Name, (*mut AstStatTypeFunction, usize)>` 的
  /// 值、用作 `referenced_type_functions: BTreeMap<*mut AstStatTypeFunction, ..>` 的键、
  /// 并 cast 后作 `ast_type_function_environment_scopes` 的 `*const` 键（见
  /// constraint_generator_prototype_type_definitions.rs:867-881），还经
  /// `push_lightuserdata(definition as *mut ())` 落到 VM 身份值；写侧取自
  /// `stat_node.as_ptr().cast()`。单改本字段为 `Option<NonNull>` 会在每个容器键/值与
  /// lightuserdata 边界逼出 `.as_ptr()` 倒灌，或被迫重打这些 map/形参类型（均超出本轮
  /// 12 字段范围），收益仅形态，故按 §2(b) 连同该身份键子系统整体保留，避免后续误改。
  pub(crate) definition: *mut AstStatTypeFunction,

  pub(crate) environment_function: DenseHashMap<Name, (*mut AstStatTypeFunction, usize)>,
  pub(crate) environment_alias: DenseHashMap<Name, (*mut TypeFun, usize)>,
}

impl UserDefinedFunctionData {
  /// `definition` 空哨兵收口于 [`Self::new_empty`] 单点（arena/身份键字段的既有
  /// 约定见字段注），带 owner 形态复用同一构造，不再重复裸指针字面量。
  pub(crate) fn new(owner: Weak<Module>) -> Self {
    Self {
      owner,
      ..Self::new_empty()
    }
  }

  /// Creates an empty instance with no owning module (used when no owner is present).
  pub(crate) fn new_empty() -> Self {
    Self {
      owner: Weak::new(),
      definition: null_mut(),
      environment_function: DenseHashMap::default(),
      environment_alias: DenseHashMap::default(),
    }
  }
}
