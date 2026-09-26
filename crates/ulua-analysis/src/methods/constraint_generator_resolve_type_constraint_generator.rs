use alloc::{string::String, sync::Arc, vec::Vec};
use core::{mem::ManuallyDrop, ptr::NonNull, str::from_utf8};

use ulua_ast::{enums::ast_type_ref::AstTypeRef, records::ast_type::AstType};

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
    // 一次裸解引用换得 &AstType，后续下转与字段读取全部走安全模式匹配。
    let node: &AstType = unsafe { &*ty };
    // SAFETY: builtin_types 指向存活内建单例，此处只读取常量 TypeId。
    let bt = self.builtin_types.get();
    let result: TypeId = match node.as_type_ref() {
      AstTypeRef::Reference(ref_) => {
        // callee 仍按 cpp 契约收 *mut 参数；引用派生指针指向同一 place。
        let ref_ptr = NonNull::from(ref_).as_ptr();
        // Safety: 满足 resolve_reference_type 的 # Safety 契约——ty 与 ref_ptr 由
        // 模式匹配确认，指向同一处 parse arena 存活的 AstTypeReference；&sp 借用
        // ManuallyDrop 出的存活 Arc<Scope>，仅作 &ScopePtr 读取。
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
      AstTypeRef::Table(tab) => {
        let tab_ptr = NonNull::from(tab).as_ptr();
        // Safety: 满足 resolve_table_type 的形参契约——tab 与 ty 由模式匹配确认，
        // 同一存活 parse arena 节点、同址非空；scope 为调用方 arc_as_mut 派生的
        // 存活 Arc<Scope> 写句柄（与上方 Arc::from_raw 同一来源）。
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
      AstTypeRef::Function(fn_node) => self.resolve_function_type(
        &sp,
        ty,
        fn_node,
        in_type_arguments,
        replace_error_with_fresh,
      ),
      AstTypeRef::Typeof(tof) => {
        // SAFETY: tof.expr 是存活类型节点名下的 arena 子表达式，只取共享引用递归。
        self.check_expr(&sp, unsafe { &*tof.expr }).ty
      }
      AstTypeRef::Optional(_) => bt.nil_type,
      AstTypeRef::Union(union_annotation) => {
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
      AstTypeRef::Intersection(intersection_annotation) => {
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
      AstTypeRef::Group(type_group_annotation) => {
        self.resolve_type_inner(scope, type_group_annotation.type_, in_type_arguments, false)
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
        // Safety: self.arena.as_ptr() 构造时接线、非空且存活于整个解析；string_annotation 是
        // 安全引用的存活节点，其 value 仅在此处只读拷贝进 arena。
        self
          .arena
          .get_mut()
          .add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(s),
          )))
      }
      AstTypeRef::Error(_) => {
        if replace_error_with_fresh {
          self.fresh_type(&sp, self.polarity)
        } else {
          bt.error_type
        }
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
