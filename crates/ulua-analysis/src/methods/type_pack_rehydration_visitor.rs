//! `type_pack_rehydration_visitor` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::ptr::null_mut;

use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArrayBuilder, ast_name::AstName, ast_type_list::AstTypeList,
  ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
  ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
  location::Location, node_handle::Node,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    get_name_type_attach::get_name_allocator_synthetic_names_generic_type_pack,
    get_type_pack::type_pack_variant_of,
  },
  records::{
    arena_handle::alias, blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    type_pack_rehydration_visitor::TypePackRehydrationVisitor,
    type_rehydration_visitor::TypeRehydrationVisitor, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack,
    synthetic_names::SyntheticNames, type_pack_id::TypePackId, type_pack_variant::TypePackVariant,
  },
};

impl TypePackRehydrationVisitor {
  pub fn type_pack_rehydration_visitor_type_pack_rehydration_visitor(
    allocator: *mut Allocator,
    synthetic_names: *mut SyntheticNames,
    type_visitor: *mut TypeRehydrationVisitor,
  ) -> Self {
    LUAU_ASSERT!(!allocator.is_null());
    LUAU_ASSERT!(!synthetic_names.is_null());
    LUAU_ASSERT!(!type_visitor.is_null());

    Self {
      allocator,
      synthetic_names,
      type_visitor,
    }
  }
}

impl TypePackRehydrationVisitor {
  /// C++ `AstTypePack* operator()(const BoundTypePack& btp) const` —
  /// `return Luau::visit(*this, btp.bound_to->ty);`.
  fn rehydrate_bound_pack(&self, btp: &BoundTypePack) -> *mut AstTypePack {
    // Safety: `btp.bound_to` 由 BoundTypePack 变体持有，对应 C++
    // `NotNull<TypePackId>`，恒指向类型 arena 中存活节点（attach 全程 arena
    // 只读）。这满足被调 `visit_type_pack` 契约对 `tp` 可解引用、变体稳定
    // 的要求；本分支正由该分发器构造 BoundTypePack 后回调进入。
    self.visit_type_pack(btp.bound_to)
  }

  #[inline]
  fn rehydrate_blocked_pack(&self, _btp: &BlockedTypePack) -> *mut AstTypePack {
    // Safety: `self.allocator` 是 `TypeRehydrationVisitor::rehydrate` 构造本
    // visitor 时存入的 AST arena 裸指针（SourceModule 拥有，晚于整个 attach
    // 才析构，构造处 LUAU_ASSERT 判过非空）。借出的 `&mut` 仅用于分配
    // `*blocked*` 泛型包节点，随函数返回即释放；此刻无任何并存的 arena
    // 借用（嵌套 visit 尚未发生）。
    let allocator = self.allocator_mut();
    let name = AstName::from_static(b"*blocked*");
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_type_pack(&self, tp: &TypePack) -> *mut AstTypePack {
    // Safety: 以下所有 `self.allocator` 重借的都是构造期存入的存活 AST arena
    // 指针（SourceModule 拥有、非空对齐）。每次 `&mut` 借出都收敛在单条语句
    // 内、随语句结束归还，保证与嵌套 visit（内部对同一 arena 再次借出）时序
    // 串行、互不重叠。
    let head_size = tp.head.len();
    // 槽位申请与逐槽写入收口在 `AstArrayBuilder`（容量 head_size、每元素恰一
    // 槽；界检与初始化契约见其 push），调用点不再手写裸块记账。
    let mut head_slots = AstArrayBuilder::new(self.allocator_mut(), head_size);
    for &type_id in tp.head.iter() {
      // `type_id` 取自 `TypePack.head`，是类型 arena 中存活的 TypeId
      //（C++ `NotNull<TypeId>` 同源），满足被调 `visit_type` 对入参可解引用
      // 的契约。`self.type_visitor` 的独占重借仅存在于本次调用期间，指向的
      // 父 visitor 在本分支未被别处借用。
      // C++ `head.data[i] = Luau::visit(*typeVisitor, tp.head[i]->ty);`
      let ast_type = alias(self.type_visitor).visit_type(type_id);
      head_slots.push(ast_type);
    }

    let head = head_slots.finish();

    // Safety: 此刻 arena 借出已全部归还，重借合法；`tail_tp_id` 来自
    // `Option<TypePackId>`，与 bound_to 同理是类型 arena 存活节点，满足
    // `visit_type_pack` 的入参契约。
    // C++ `if (tp.tail) tail = Luau::visit(*this, (*tp.tail)->ty);`
    let tail = if let Some(tail_tp_id) = tp.tail {
      // Safety: 此前所有 arena/type_visitor 借出均已归还，此刻独占；tail_tp_id 取自
      // Option<TypePackId>，与 bound_to 同理是类型 arena 存活节点，满足被调 visit_type_pack 契约。
      self.visit_type_pack(tail_tp_id)
    } else {
      null_mut()
    };

    let type_list = AstTypeList {
      types: head,
      tail_type: tail,
    };

    let node = AstTypePackExplicit::new(Location::default(), type_list);
    // Safety: 重借构造期存入且仍独占的 arena 指针分配 explicit 包节点，
    // `&mut` 借出止于本语句（尾表达式），与函数体内此前所有借出串行。
    self.allocator_mut().alloc(node).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_variadic_pack(&self, vtp: &VariadicTypePack) -> *mut AstTypePack {
    if vtp.hidden {
      return null_mut();
    }

    // C++ `Luau::visit(*typeVisitor, vtp.ty->ty)`.
    let type_visitor = alias(self.type_visitor);
    let variadic_type = type_visitor.visit_type(vtp.ty);

    // Safety: arena 借出已全部归还，重借构造期存入的存活 arena 指针分配
    // 变参包节点，`&mut` 随尾表达式返回释放，无重叠别名。
    let allocator = self.allocator_mut();
    let node = AstTypePackVariadic::new(Location::default(), Node::from_raw(variadic_type));
    allocator.alloc(node).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_generic_pack(&self, gtp: &GenericTypePack) -> *mut AstTypePack {
    // get_name 助手以 `gtp` 的 arena 地址作键读写名字缓存（gtp 借自存活类型
    // arena），两个借出均随函数返回释放。
    let allocator = self.allocator_mut();
    let synthetic_names = alias(self.synthetic_names);
    let name_ptr =
      get_name_allocator_synthetic_names_generic_type_pack(allocator, synthetic_names, gtp);
    let name = AstName::ast_name_u8(name_ptr);
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_free_pack(&self, _gtp: &FreeTypePack) -> *mut AstTypePack {
    // Safety: 独占重借构造期存入的存活 arena 指针（SourceModule 拥有，attach
    // 期间无人并发访问），仅用于分配 `free` 泛型包节点；借出随尾表达式释放。
    let allocator = self.allocator_mut();
    let name = AstName::from_static(b"free");
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_error_pack(&self, _tp: &ErrorTypePack) -> *mut AstTypePack {
    // Safety: 同其余分支——构造期存入的 arena 指针存活且此刻独占，这次借出
    // 只分配 `Unifiable<Error>` 泛型包节点，尾表达式后即归还借用。
    let allocator = self.allocator_mut();
    let name = AstName::from_static(b"Unifiable<Error>");
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  fn rehydrate_type_function_instance_pack(
    &self,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> *mut AstTypePack {
    let allocator = self.allocator_mut();
    let name_str = &tfitp.function().name;
    let name = AstName::from_raw_parts(name_str.as_ptr(), name_str.len() as u32);
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }
}

// Source: `Analysis/src/TypeAttach.cpp` (the `Luau::visit(*this, TypePackId->ty)`
// overload dispatch over `TypePackVariant`).
//
// C++ `Luau::visit(tprv, tp->ty)` selects the `operator()` overload matching
// the active pack alternative. The Rust port is a `match` over the variant
// calling the pinned `operator_call_N` arm per member.

impl TypePackRehydrationVisitor {
  pub fn visit_type_pack(&self, tp: TypePackId) -> *mut AstTypePack {
    // 变体读取收口在 `type_pack_variant_of`（arena 节点有效性契约同 C++ get）。
    match type_pack_variant_of(tp) {
      TypePackVariant::Bound(b) => {
        // C++ `operator()(const BoundTypePack& btp)` returns
        // `Luau::visit(*this, btp.bound_to->ty)`.
        let btp = BoundTypePack { bound_to: *b };
        self.rehydrate_bound_pack(&btp)
      }
      TypePackVariant::Error(e) => self.rehydrate_error_pack(e),
      TypePackVariant::Free(f) => self.rehydrate_free_pack(f),
      TypePackVariant::Generic(g) => self.rehydrate_generic_pack(g),
      TypePackVariant::TypePack(t) => self.rehydrate_type_pack(t),
      TypePackVariant::Variadic(v) => self.rehydrate_variadic_pack(v),
      TypePackVariant::Blocked(b) => self.rehydrate_blocked_pack(b),
      TypePackVariant::TypeFunctionInstance(t) => self.rehydrate_type_function_instance_pack(t),
    }
  }
}
