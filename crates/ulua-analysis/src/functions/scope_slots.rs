//! `ScopePtr`（cpp `std::shared_ptr<Scope>`）槽位写入门面的收口点
//! （`review.md` §2「非空指针 → 引用/句柄」在本 crate 的落点）。
//!
//! cpp 侧约束生成对 `scope->bindings`、`scope->lvalueTypes` 等表的写入是语言
//! 层面的普通成员访问；Rust 侧 `ScopePtr = Arc<Scope>` 借不出 `&mut`（同一
//! Scope 在 `self.scopes` 里另有一份强引用），既有惯用法是 `arc_as_mut` 派生
//! 裸句柄再 `(*p).field` 解引用，于是每个写入点都拖一个 `unsafe` 块。本门面
//! 把「派生句柄 + 解引用」合并为一次调用，业务侧只留下具名写入动作，
//! `unsafe` 不再渗进 visit 分支。
//!
//! # 契约（唯一句柄物化点 [`scope_mut`]）
//!
//! 1. 目标 `Scope` 由入参 `Arc` 保活：调用点持有的 `&ScopePtr` 在整条语句内
//!    存活，故物化出的引用不会悬垂；
//! 2. 单线程驱动：任一时刻只有一条经本门面对象写入的可变借用存活（与原
//!    `arc_as_mut` + 逐处 `// SAFETY:` 注释同一前提，`Scope` 的
//!    `unsafe impl Send/Sync` 亦以此为依据）；
//! 3. 只读路径不经本门面：`Arc` 的 `Deref` 已给出 `&Scope`，读取一律走
//!    `scope.field` / `scope.method(..)`，不产生可变别名。

use core::ptr::{from_mut, from_ref, null_mut};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_function::AstExprFunction, ast_local::AstLocal,
  ast_stat_type_alias::AstStatTypeAlias, location::Location,
};

use crate::{
  functions::arc_as_mut::arc_as_mut,
  methods::constraint_generator_prototype_type_definitions::make_binding,
  records::{
    binding::Binding, class_decl_record::ClassDeclRecord,
    constraint_generator::ConstraintGenerator, function_signature::FunctionSignature,
    inference_pack::InferencePack, scope::Scope, symbol::Symbol, type_fun::TypeFun,
  },
  type_aliases::{def_id_def::DefId, scope_ptr_type::ScopePtr, type_id::TypeId},
};

/// `ScopePtr` → 独占写句柄：`arc_as_mut` 派生 + 一次解引用，全部收拢在此。
///
/// 返回 `'static` 与原 `alias`/`arc_as_mut` 组合的借用检查行为同构（生命周期
/// 刻意不受约束，避免迁移改变调用点的借用语义）。
#[inline]
pub(crate) fn scope_mut(scope: &ScopePtr) -> &'static mut Scope {
  // Safety: 前置条件即模块契约——入参借用持有的 Arc 在本语句内存活，且分析为
  // 单线程驱动、无并存可变别名（与原 `unsafe { (*arc_as_mut(scope)) … }` 同构）。
  unsafe { &mut *arc_as_mut(scope) }
}

/// `scope->bindings[sym] = binding`（cpp 直译）。
#[inline]
pub(crate) fn bind(scope: &ScopePtr, sym: Symbol, binding: Binding) {
  scope_mut(scope).bindings.insert(sym, binding);
}

/// `scope->bindings[Symbol(local)] = Binding{ty, local->location}`。
///
/// cpp 的 local 绑定多处同款（`Binding` 其余字段恒为默认值），折叠为一个动作
/// 以免调用点重复手搓字面量：默认值与 [`make_binding`] 同一来源。
#[inline]
pub(crate) fn bind_local(scope: &ScopePtr, local: &AstLocal, ty: TypeId) {
  let local_ptr = from_ref(local).cast_mut();
  scope_mut(scope).bindings.insert(
    Symbol::from_local(local_ptr),
    make_binding(ty, local.location),
  );
}

/// `scope->lvalueTypes[def] = ty`。
#[inline]
pub(crate) fn set_lvalue_type(scope: &ScopePtr, def: DefId, ty: TypeId) {
  *scope_mut(scope).lvalue_types.get_or_insert(def) = ty;
}

/// cpp `ty = scope->lvalueTypes.getOrInsert(def)`：缺失时插入默认值再读回，
/// 与写侧同一 `getOrInsert` 语义（不是纯查找）。
#[inline]
pub(crate) fn lvalue_type(scope: &ScopePtr, def: DefId) -> TypeId {
  *scope_mut(scope).lvalue_types.get_or_insert(def)
}

/// 绑定 + lvalue 一次写入（cpp 相邻的 `bindings[..]`/`lvalueTypes[..]` 两行）。
#[inline]
pub(crate) fn bind_with_lvalue(
  scope: &ScopePtr,
  sym: Symbol,
  binding: Binding,
  def: DefId,
  ty: TypeId,
) {
  let scope = scope_mut(scope);
  scope.bindings.insert(sym, binding);
  *scope.lvalue_types.get_or_insert(def) = ty;
}

/// `scope->inheritAssignments(child)`。
#[inline]
pub(crate) fn inherit_assignments(parent: &ScopePtr, child: &ScopePtr) {
  scope_mut(parent).inherit_assignments(child);
}

/// `scope->inheritRefinements(child)`。
#[inline]
pub(crate) fn inherit_refinements(parent: &ScopePtr, child: &ScopePtr) {
  scope_mut(parent).inherit_refinements(child);
}

/// cpp `checkFunctionSignature(parent, enclosingClass, fn, expectedType, name)`
/// 的安全包装：`self`/`parent`/`fn` 三者的存活契约在本门面集中兑现，
/// 调用点不再出现裸指针形参与 `unsafe` 块。
///
/// `enclosing_class` 取 `None` 即 cpp 传 `nullptr` 的「非类体内声明」形态
/// （被调方对 `DebugLuauUserDefinedClasses` 关闭态有相应断言兜底）。
#[inline]
pub(crate) fn check_function_signature(
  cg: &mut ConstraintGenerator,
  parent: &ScopePtr,
  func: &AstExprFunction,
  expected_type: Option<TypeId>,
  original_name: Option<Location>,
) -> FunctionSignature {
  check_function_signature_in(cg, parent, None, func, expected_type, original_name)
}

/// [`check_function_signature`] 的类体内形态：`enclosing_class` 为 `Some` 时
/// 传记录地址（cpp `ClassDeclRecord*`），目标由调用方的局部 `Arc` clone /
/// `find().cloned()` 保活。
pub(crate) fn check_function_signature_in(
  cg: &mut ConstraintGenerator,
  parent: &ScopePtr,
  enclosing_class: Option<&mut ClassDeclRecord>,
  func: &AstExprFunction,
  expected_type: Option<TypeId>,
  original_name: Option<Location>,
) -> FunctionSignature {
  let enclosing = enclosing_class.map_or_else(null_mut, |r| from_mut(r).cast_const().cast_mut());
  // Safety: `func` 由调用方自 parse arena 交出的存活节点借用还原地址，与 cpp
  // 传入的 `AstExprFunction*` 指向同一节点（AST 在检查会话内只读不搬迁）；
  // `enclosing` 为 `null_mut()`（非类体）或调用方保活的 `ClassDeclRecord` 地址。
  unsafe {
    cg.check_function_signature(
      parent,
      enclosing,
      from_ref(func).cast_mut(),
      expected_type,
      original_name,
    )
  }
}

/// `ConstraintGenerator::checkPack(scope, expr, expectedTypes, generalize)` 的
/// 安全包装：被调方是纯逻辑层 `unsafe fn`（形参为裸 `AstExpr*`），此处把
/// 「共享借用还原地址」的契约收在一处，visit 侧只交出引用。
pub(crate) fn check_pack_expr(
  cg: &mut ConstraintGenerator,
  scope: &ScopePtr,
  expr: &AstExpr,
  expected_types: &[Option<TypeId>],
  generalize: bool,
) -> InferencePack {
  // Safety: `expr` 由调用方自 parse arena 交出的存活节点借用还原地址，与 cpp
  // 传入的 `AstExpr*` 同一目标；被调方沿该节点做只读遍历。
  unsafe {
    cg.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
      scope,
      from_ref(expr).cast_mut(),
      expected_types,
      generalize,
    )
  }
}

/// [`ConstraintGenerator::resolve_generic_default_parameters`] 的安全包装：
/// cpp `resolveGenericDefaultParameters(defnScope, alias, fun)` 形态。
pub(crate) fn resolve_generic_defaults(
  cg: &mut ConstraintGenerator,
  defn_scope: &ScopePtr,
  alias: &AstStatTypeAlias,
  fun: &TypeFun,
) {
  // Safety: `defn_scope` 的 Arc 由调用方持有、本语句内存活（arc_as_mut 独占写
  // 惯用法，单线程驱动、借用窗口止于本调用）；`alias` 为分发层交出的存活
  // AstStatTypeAlias 共享借用还原的地址，被调方只读遍历其 generics 数组；
  // `fun` 是该 alias 的 TypeFun（形参个数由被调方断言校验）。
  unsafe {
    cg.resolve_generic_default_parameters(arc_as_mut(defn_scope), from_ref(alias).cast_mut(), fun)
  };
}
