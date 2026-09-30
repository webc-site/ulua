use alloc::{str::from_utf8, string::String, vec::Vec};
use core::ptr::from_ref;

use ulua_ast::{enums::ast_type_ref::AstTypeRef, records::ast_type::AstType};

use crate::{
  enums::polarity::Polarity,
  functions::shared_mut::shared_mut,
  records::{
    arena_handle::{alias, alias_ref},
    constraint_generator::ConstraintGenerator,
    intersection_type::IntersectionType,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    union_type::UnionType,
  },
  type_aliases::{scope_ptr_type::ScopePtr, singleton_variant::SingletonVariant, type_id::TypeId},
};

impl ConstraintGenerator {
  /// C++ `TypeId ConstraintGenerator::resolveType(const ScopePtr& scope, AstType* ty, bool, bool, Polarity)`。
  /// `scope` 为调用方持有的 `ScopePtr` 共享借用（cpp `const ScopePtr&` 直译），
  /// `ty` 为 parse arena 存活节点的共享借用（AST 指针字段经 `alias_ref`/`slot_opt`
  /// 收口后传入），函数全程只读该节点。
  pub fn resolve_type(
    &mut self,
    scope: &ScopePtr,
    ty: &AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
    initial_polarity: Polarity,
  ) -> TypeId {
    // Reset the polarity
    self.polarity = initial_polarity;
    self.resolve_type_inner(scope, ty, in_type_arguments, replace_error_with_fresh)
  }

  // ConstraintGenerator::resolveType_(const ScopePtr&, AstType*, bool, bool)
  // (ConstraintGenerator.cpp:4578).
  pub(crate) fn resolve_type_inner(
    &mut self,
    scope: &ScopePtr,
    ty: &AstType,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    // SAFETY: builtin_types 指向存活内建单例，此处只读取常量 TypeId。
    let bt = self.builtin_types.get();
    let result: TypeId = match ty.as_type_ref() {
      AstTypeRef::Reference(ref_) => {
        // 引用形参链：`ty` 与 `ref_` 由模式匹配确认指向同一存活 parse arena 节点。
        self.resolve_reference_type(scope, ty, ref_, in_type_arguments, replace_error_with_fresh)
      }
      AstTypeRef::Table(tab) => {
        self.resolve_table_type(scope, ty, tab, in_type_arguments, replace_error_with_fresh)
      }
      AstTypeRef::Function(fn_node) => self.resolve_function_type(
        scope,
        ty,
        fn_node,
        in_type_arguments,
        replace_error_with_fresh,
      ),
      AstTypeRef::Typeof(tof) => {
        // tof.expr 是存活类型节点名下的 arena 子表达式（parser 保证非空），
        // alias_ref 收口为共享引用递归。
        self.check_expr(scope, alias_ref(tof.expr)).ty
      }
      AstTypeRef::Optional(_) => bt.nil_type,
      AstTypeRef::Union(union_annotation) => {
        if union_annotation.types.size == 1 {
          let first_ty = alias_ref(union_annotation.types.as_slice()[0]);
          self.resolve_type_inner(scope, first_ty, in_type_arguments, false)
        } else {
          let mut parts: Vec<TypeId> = Vec::new();
          for &part in union_annotation.types.as_slice() {
            parts.push(self.resolve_type_inner(scope, alias_ref(part), in_type_arguments, false));
          }
          // arena 独占追加窗口；add_type 仅在 bump 块尾追加新节点。
          self.arena.get_mut().add_type(UnionType { options: parts })
        }
      }
      AstTypeRef::Intersection(intersection_annotation) => {
        if intersection_annotation.types.size == 1 {
          let first_ty = alias_ref(intersection_annotation.types.as_slice()[0]);
          self.resolve_type_inner(scope, first_ty, in_type_arguments, false)
        } else {
          let mut parts: Vec<TypeId> = Vec::new();
          for &part in intersection_annotation.types.as_slice() {
            parts.push(self.resolve_type_inner(scope, alias_ref(part), in_type_arguments, false));
          }
          // 同 union 分支——arena 构造时接线、非空且比本次解析长寿。
          self.arena.get_mut().add_type(IntersectionType { parts })
        }
      }
      AstTypeRef::Group(type_group_annotation) => {
        let inner = alias_ref(type_group_annotation.type_);
        self.resolve_type_inner(scope, inner, in_type_arguments, false)
      }
      AstTypeRef::SingletonBool(bool_annotation) => {
        if bool_annotation.value {
          bt.true_type
        } else {
          bt.false_type
        }
      }
      AstTypeRef::SingletonString(string_annotation) => {
        let s: String = String::from(from_utf8(string_annotation.value.as_bytes()).unwrap_or(""));
        // string_annotation 是安全引用的存活节点，其 value 仅在此处只读拷贝进 arena。
        self
          .arena
          .get_mut()
          .add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(s),
          )))
      }
      AstTypeRef::Error(_) => {
        if replace_error_with_fresh {
          self.fresh_type(scope, self.polarity)
        } else {
          bt.error_type
        }
      }
    };

    if let Some(module) = &self.module {
      // `module` Arc 由 self.module 持有、与会话同寿；分析单线程串行，
      // ast_resolved_types 表此刻无其他存活借用，经 alias 收口写一个条目，
      // 与 C++ `module->astResolvedTypes[ty] = result` 同址同值。
      let module_ptr = shared_mut(module);
      *alias(module_ptr)
        .ast_resolved_types
        .get_or_insert(from_ref(ty)) = result;
    }

    result
  }
}
