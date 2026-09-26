use alloc::{string::String, sync::Arc, vec::Vec};
use core::{mem::ManuallyDrop, ptr::NonNull, str::from_utf8};

use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::{AstNodeClass, ast_node_as_unchecked},
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::polarity::Polarity,
  functions::arc_as_mut::arc_as_mut,
  records::{
    constraint_generator::ConstraintGenerator, intersection_type::IntersectionType, scope::Scope,
    singleton_type::SingletonType, string_singleton::StringSingleton, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ConstraintGenerator {
  pub fn resolve_type(
    &mut self,
    _scope: *mut Scope,
    ty: *mut AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypeId {
    // Reset the polarity
    self.polarity = initial_polarity;
    self.resolve_type_inner(_scope, ty, in_type_arguments, replace_error_with_fresh)
  }

  // ConstraintGenerator::resolveType_(const ScopePtr&, AstType*, bool, bool)
  // (ConstraintGenerator.cpp:4578).
  pub(crate) fn resolve_type_inner(
    &mut self,
    scope: *mut Scope,
    ty: *mut AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    // The resolve helpers and `check`/`freshType` want a `const ScopePtr&`; the
    // C++ overload also takes a `const ScopePtr&`. Reconstruct one without
    // taking ownership of the refcount.
    // Safety: 调用方（visit_* 入口）以 arc_as_mut(&ScopePtr) 派生本 `scope`，它指向
    // 调用方持有的 Arc<Scope> 堆分配（对齐、非空）且在该借用期内存活；ManuallyDrop
    // 保证本句柄离开作用域时既不 decrement 也不 dealloc，引用计数与分配所有权始终
    // 归属调用方的 Arc，等价 C++ 仅借用 const ScopePtr& 的形参契约。
    let sp = ManuallyDrop::new(unsafe { Arc::from_raw(scope as *const Scope) });

    // SAFETY: 调用方契约保证 ty 非空且指向 arena 存活节点；本函数对其只读，
    // 一次裸解引用换得 &AstNode，后续下转与字段读取全部走安全引用。
    let node: &AstNode = unsafe { &*(ty as *const AstNode) };
    // SAFETY: builtin_types 指向存活内建单例，此处只读取常量 TypeId。
    let bt = self.builtin_types.get();
    let result: TypeId = match node.class_index {
      AstTypeReference::CLASS_INDEX => {
        // callee 仍按 cpp 契约收 *mut 参数；引用派生指针指向同一 place。
        let ref_: &AstTypeReference = unsafe { ast_node_as_unchecked(node) };
        let ref_ptr = NonNull::from(ref_).as_ptr();
        // Safety: 满足 resolve_reference_type 的 # Safety 契约——ty 与 ref_ptr 由
        // class_index 命中确认，指向同一处 parse arena 存活的
        // AstTypeReference（repr(C) 首字段基址重合，非空同址）；&sp 借用 ManuallyDrop
        // 出的存活 Arc<Scope>，仅作 &ScopePtr 读取。
        unsafe {
          self.resolve_reference_type(
            &sp,
            ty,
            ref_ptr,
            in_type_arguments,
            replace_error_with_fresh,
          )
        }
      }
      AstTypeTable::CLASS_INDEX => {
        let tab: &AstTypeTable = unsafe { ast_node_as_unchecked(node) };
        let tab_ptr = NonNull::from(tab).as_ptr();
        // Safety: 满足 resolve_table_type 的形参契约——tab 与 ty 由 class_index
        // 命中确认，同一存活 parse arena 节点、同址非空；scope 为调用方
        // arc_as_mut 派生的存活 Arc<Scope> 写句柄（与上方 Arc::from_raw 同一来源）。
        unsafe {
          self.resolve_table_type(
            scope,
            ty,
            tab_ptr,
            in_type_arguments,
            replace_error_with_fresh,
          )
        }
      }
      AstTypeFunction::CLASS_INDEX => {
        let fn_node: &AstTypeFunction = unsafe { ast_node_as_unchecked(node) };
        self.resolve_function_type(
          &sp,
          ty,
          fn_node,
          in_type_arguments,
          replace_error_with_fresh,
        )
      }
      AstTypeTypeof::CLASS_INDEX => {
        let tof: &AstTypeTypeof = unsafe { ast_node_as_unchecked(node) };
        // SAFETY: tof.expr 是存活类型节点名下的 arena 子表达式，只取共享引用递归。
        self.check_expr(&sp, unsafe { &*tof.expr }).ty
      }
      AstTypeOptional::CLASS_INDEX => bt.nil_type,
      AstTypeUnion::CLASS_INDEX => {
        let union_annotation: &AstTypeUnion = unsafe { ast_node_as_unchecked(node) };
        if union_annotation.types.size == 1 {
          self.resolve_type_inner(
            scope,
            union_annotation.types.as_slice()[0],
            in_type_arguments,
            false,
          )
        } else {
          let mut parts: Vec<TypeId> = Vec::new();
          for &part in union_annotation.types.as_slice() {
            parts.push(self.resolve_type_inner(scope, part, in_type_arguments, false));
          }
          // Safety: self.arena.as_ptr() 在 ConstraintGenerator 构造时接线为非空裸指针，指向本次
          // check 会话存活的类型 arena（bump 块地址不移动），add_type 只追加节点。
          self.arena.get_mut().add_type(UnionType { options: parts })
        }
      }
      AstTypeIntersection::CLASS_INDEX => {
        let intersection_annotation: &AstTypeIntersection = unsafe { ast_node_as_unchecked(node) };
        if intersection_annotation.types.size == 1 {
          self.resolve_type_inner(
            scope,
            intersection_annotation.types.as_slice()[0],
            in_type_arguments,
            false,
          )
        } else {
          let mut parts: Vec<TypeId> = Vec::new();
          for &part in intersection_annotation.types.as_slice() {
            parts.push(self.resolve_type_inner(scope, part, in_type_arguments, false));
          }
          // Safety: 同 union 分支——self.arena.as_ptr() 构造时接线、非空且比本次解析长寿，
          // add_type 仅在 bump arena 追加新节点。
          self.arena.get_mut().add_type(IntersectionType { parts })
        }
      }
      AstTypeGroup::CLASS_INDEX => {
        let type_group_annotation: &AstTypeGroup = unsafe { ast_node_as_unchecked(node) };
        self.resolve_type_inner(scope, type_group_annotation.type_, in_type_arguments, false)
      }
      AstTypeSingletonBool::CLASS_INDEX => {
        let bool_annotation: &AstTypeSingletonBool = unsafe { ast_node_as_unchecked(node) };
        if bool_annotation.value {
          bt.true_type
        } else {
          bt.false_type
        }
      }
      AstTypeSingletonString::CLASS_INDEX => {
        let string_annotation: &AstTypeSingletonString = unsafe { ast_node_as_unchecked(node) };
        let s: String = String::from(from_utf8(string_annotation.value.as_bytes()).unwrap_or(""));
        // Safety: self.arena.as_ptr() 构造时接线、非空且存活于整个解析；string_annotation 是
        // class_index 命中的 & 引用（同址节点），其 value 仅在此处只读拷贝进 arena。
        self
          .arena
          .get_mut()
          .add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(s),
          )))
      }
      AstTypeError::CLASS_INDEX => {
        if replace_error_with_fresh {
          self.fresh_type(&sp, self.polarity)
        } else {
          bt.error_type
        }
      }
      _ => {
        LUAU_ASSERT!(false);
        bt.error_type
      }
    };

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // Safety: module_ptr 由 self.module（Option<ModulePtr>，Arc clone）经 arc_as_mut
      // 派生，Arc 存活至本语句末；分析单线程串行，ast_resolved_types 表此刻无其他
      // 存活借用，重建 &mut 写一个条目与 C++ `module->astResolvedTypes[ty] = result`
      // 同址同值，无别名冲突。
      unsafe {
        *(*module_ptr)
          .ast_resolved_types
          .get_or_insert(ty as *const AstType) = result;
      }
    }

    result
  }
}
