//! `internal_type_function_finder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;

use ulua_common::{
  fflag::LuauSkipUnusedTypeTraversals, macros::luau_assert::LUAU_ASSERT,
  records::dense_hash_set::DenseHashSet,
};

use crate::{
  functions::{
    are_equivalent::are_equivalent, follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    internal_type_function_finder::InternalTypeFunctionFinder,
    type_function_finder::TypeFunctionFinder,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_once_visitor::TypeOnceVisitor,
    visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl<'a> InternalTypeFunctionFinder<'a> {
  /// C++ `explicit InternalTypeFunctionFinder(std::vector<TypeId>& declStack)`
  /// (TypeChecker2.cpp:194), seeding `mentioned{Functions,FunctionPacks}` from a
  /// `TypeFunctionFinder` traversal of the declaration stack.
  pub fn new(decl_stack: &'a [TypeId]) -> Self {
    if LuauSkipUnusedTypeTraversals.get() {
      InternalTypeFunctionFinder {
        base: TypeOnceVisitor::new(String::from("InternalTypeFunctionFinder"), true),
        internal_functions: DenseHashSet::default(),
        internal_pack_functions: DenseHashSet::default(),
        mentioned_functions: DenseHashSet::default(),
        mentioned_function_packs: DenseHashSet::default(),
        unscanned_decl_stack: Some(decl_stack),
      }
    } else {
      let mut f = TypeFunctionFinder::new();
      for fn_ty in decl_stack.iter().copied() {
        f.traverse_type_id(fn_ty);
      }

      InternalTypeFunctionFinder {
        base: TypeOnceVisitor::new(String::from("InternalTypeFunctionFinder"), true),
        internal_functions: DenseHashSet::default(),
        internal_pack_functions: DenseHashSet::default(),
        mentioned_functions: f.mentioned_functions,
        mentioned_function_packs: f.mentioned_function_packs,
        unscanned_decl_stack: None,
      }
    }
  }

  pub(crate) fn ensure_mentioned_functions(&mut self) {
    if LuauSkipUnusedTypeTraversals.get() {
      if let Some(decl_stack) = self.unscanned_decl_stack.take() {
        let mut f = TypeFunctionFinder::new();
        for &fn_ty in decl_stack {
          f.traverse_type_id(fn_ty);
        }
        self.mentioned_functions = f.mentioned_functions;
        self.mentioned_function_packs = f.mentioned_function_packs;
      }
    } else {
      LUAU_ASSERT!(self.unscanned_decl_stack.is_none());
    }
  }
}
impl GenericTypeVisitorTrait for TypeFunctionFinder {
  type Seen = DenseHashSet<VisitKey>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// TypeChecker2.cpp:171 — record every mentioned type function instance.
  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _tfit: &TypeFunctionInstanceType,
  ) -> bool {
    self.mentioned_functions.insert(ty);
    true
  }

  /// TypeChecker2.cpp:177 — record every mentioned type function instance pack.
  fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    _tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    self.mentioned_function_packs.insert(tp);
    true
  }
}
impl<'a> GenericTypeVisitorTrait for InternalTypeFunctionFinder<'a> {
  type Seen = DenseHashSet<VisitKey>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  /// TypeChecker2.cpp:205 — `bool visit(TypeId, const TypeFunctionInstanceType&)`.
  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    let has_generic = tfit
      .type_arguments
      .iter()
      .any(|&p| get_type::get::<GenericType>(follow_type::follow(p)).is_some())
      || tfit
        .pack_arguments
        .iter()
        .any(|&p| get_type_pack::get::<GenericTypePack>(follow_type_pack::follow(p)).is_some());

    if has_generic {
      self.ensure_mentioned_functions();
      let is_mentioned = self.mentioned_functions.iter().any(|&mentioned| {
        let mentioned_tfit = get_type::get::<TypeFunctionInstanceType>(mentioned);
        LUAU_ASSERT!(mentioned_tfit.is_some());
        are_equivalent(
          tfit,
          mentioned_tfit.expect("C++ `LUAU_ASSERT(mentionedTf)` 紧邻断言蕴含必命中"),
        )
      });
      if is_mentioned {
        return true;
      }

      self.internal_functions.insert(ty);
    }

    true
  }

  /// TypeChecker2.cpp:245 — `bool visit(TypePackId, const TypeFunctionInstanceTypePack&)`.
  fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    let has_generic = tfitp
      .type_arguments
      .iter()
      .any(|&p| get_type::get::<GenericType>(follow_type::follow(p)).is_some())
      || tfitp
        .pack_arguments
        .iter()
        .any(|&p| get_type_pack::get::<GenericTypePack>(follow_type_pack::follow(p)).is_some());

    if has_generic {
      self.ensure_mentioned_functions();
      let is_mentioned = self.mentioned_function_packs.iter().any(|&mentioned| {
        let mentioned_tfitp = get_type_pack::get::<TypeFunctionInstanceTypePack>(mentioned);
        LUAU_ASSERT!(mentioned_tfitp.is_some());
        are_equivalent(
          tfitp,
          mentioned_tfitp.expect("C++ `LUAU_ASSERT(mentionedTf)` 紧邻断言蕴含必命中"),
        )
      });
      if is_mentioned {
        return true;
      }

      self.internal_pack_functions.insert(tp);
    }

    true
  }
}
