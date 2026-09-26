use core::{mem::size_of, ptr::null_mut};

use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_list::AstTypeList,
  ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit,
  ast_type_pack_generic::AstTypePackGeneric, ast_type_pack_variadic::AstTypePackVariadic,
  location::Location,
};

use crate::{
  functions::get_name_type_attach::get_name_allocator_synthetic_names_generic_type_pack,
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    type_pack_rehydration_visitor::TypePackRehydrationVisitor,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack},
};

impl TypePackRehydrationVisitor {
  /// C++ `AstTypePack* operator()(const BoundTypePack& btp) const` —
  /// `return Luau::visit(*this, btp.bound_to->ty);`.
  pub fn rehydrate_bound_pack(&self, btp: &BoundTypePack) -> *mut AstTypePack {
    // Safety: `btp.bound_to` 由 BoundTypePack 变体持有，对应 C++
    // `NotNull<TypePackId>`，恒指向类型 arena 中存活节点（attach 全程 arena
    // 只读）。这满足被调 `visit_type_pack` 契约对 `tp` 可解引用、变体稳定
    // 的要求；本分支正由该分发器构造 BoundTypePack 后回调进入。
    self.visit_type_pack(btp.bound_to)
  }

  #[inline]
  pub fn rehydrate_blocked_pack(&self, _btp: &BlockedTypePack) -> *mut AstTypePack {
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
  pub fn rehydrate_type_pack(&self, tp: &TypePack) -> *mut AstTypePack {
    // Safety: 以下所有 `self.allocator` 重借的都是构造期存入的存活 AST arena
    // 指针（SourceModule 拥有、非空对齐）。每次 `&mut` 借出都收敛在单条语句
    // 内、随语句结束归还，保证与嵌套 visit（内部对同一 arena 再次借出）时序
    // 串行、互不重叠。
    let head_size = tp.head.len();
    // Safety: 独占重借构造期存入、SourceModule 拥有且此刻无并存借用的 AST arena 指针；allocate
    // 只在 bump 块内取 head_size*size_of::<*mut AstType>() 字节，`&mut` 借出止于本表达式。
    let head_data = self
      .allocator_mut()
      .allocate(size_of::<*mut AstType>() * head_size) as *mut *mut AstType;
    for (i, &type_id) in tp.head.iter().enumerate() {
      // Safety: `type_id` 取自 `TypePack.head`，是类型 arena 中存活的 TypeId
      //（C++ `NotNull<TypeId>` 同源），满足被调 `visit_type` 对入参可解引用
      // 的契约。`self.type_visitor` 的独占重借仅存在于本次调用期间，指向的
      // 父 visitor 在本分支未被别处借用。
      // C++ `head.data[i] = Luau::visit(*typeVisitor, tp.head[i]->ty);`
      let ast_type = unsafe { (*self.type_visitor).visit_type(type_id) };
      // Safety: head_data 是上面刚向 arena 申请的 head_size 槽 *mut AstType
      // 块，i < head_size 故 add(i) 在块内；槽位未初始化但元素为 Copy 裸指针
      // 无 Drop，`*ptr = v` 单次赋值即完成初始化。bump arena 的后续分配不会
      // 搬移已申请内存。
      unsafe { *head_data.add(i) = ast_type };
    }

    let head = AstArray {
      data: head_data,
      size: head_size,
    };

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
  pub fn rehydrate_variadic_pack(&self, vtp: &VariadicTypePack) -> *mut AstTypePack {
    if vtp.hidden {
      return null_mut();
    }

    // Safety: `vtp.ty` 是 VariadicTypePack 变体内置的 TypeId（C++ NotNull
    // 语义），指向类型 arena 存活节点，满足 `visit_type` 契约；type_visitor
    // 的独占借出止于本次调用，嵌套侧重借同一 arena 与此借出时序串行。
    // C++ `Luau::visit(*typeVisitor, vtp.ty->ty)`.
    let type_visitor = unsafe { &mut *self.type_visitor };
    // Safety: type_visitor 的独占借用（上一行）此刻唯一；vtp.ty 为 VariadicTypePack 内嵌的存活
    // TypeId（C++ NotNull 语义），满足 visit_type 入参契约，借用止于本次调用。
    let variadic_type = type_visitor.visit_type(vtp.ty);

    // Safety: arena 借出已全部归还，重借构造期存入的存活 arena 指针分配
    // 变参包节点，`&mut` 随尾表达式返回释放，无重叠别名。
    let allocator = self.allocator_mut();
    let node = AstTypePackVariadic::new(Location::default(), variadic_type);
    allocator.alloc(node).cast::<AstTypePack>()
  }

  #[inline]
  pub fn rehydrate_generic_pack(&self, gtp: &GenericTypePack) -> *mut AstTypePack {
    // Safety: allocator 与 synthetic_names 都是构造期（rehydrate 经
    // TypeAttacher）存入的非空对齐裸指针，分别指向 SourceModule arena 与
    // SyntheticNames 表，两者指向不同对象故同时 `&mut` 重借互不混叠；存活
    // 期覆盖整个 attach。get_name 助手以 `gtp` 的 arena 地址作键读写名字
    // 缓存（gtp 借自存活类型 arena），两个借出均随函数返回释放。
    let allocator = self.allocator_mut();
    // Safety: synthetic_names 为构造期存入的非空对齐 SyntheticNames 表指针，与上一行 allocator 指向
    // 不同对象，故两个 &mut 并存不混叠；二者存活覆盖整个 attach。
    let synthetic_names = unsafe { &mut *self.synthetic_names };
    let name_ptr =
      get_name_allocator_synthetic_names_generic_type_pack(allocator, synthetic_names, gtp);
    let name = AstName::ast_name_u8(name_ptr);
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  pub fn rehydrate_free_pack(&self, _gtp: &FreeTypePack) -> *mut AstTypePack {
    // Safety: 独占重借构造期存入的存活 arena 指针（SourceModule 拥有，attach
    // 期间无人并发访问），仅用于分配 `free` 泛型包节点；借出随尾表达式释放。
    let allocator = self.allocator_mut();
    let name = AstName::from_static(b"free");
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  pub fn rehydrate_error_pack(&self, _tp: &ErrorTypePack) -> *mut AstTypePack {
    // Safety: 同其余分支——构造期存入的 arena 指针存活且此刻独占，这次借出
    // 只分配 `Unifiable<Error>` 泛型包节点，尾表达式后即归还借用。
    let allocator = self.allocator_mut();
    let name = AstName::from_static(b"Unifiable<Error>");
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }

  #[inline]
  pub fn rehydrate_type_function_instance_pack(
    &self,
    tfitp: &TypeFunctionInstanceTypePack,
  ) -> *mut AstTypePack {
    let allocator = self.allocator_mut();
    let name_str = unsafe { &(*tfitp.function).name };
    let name = AstName::from_raw_parts(name_str.as_ptr(), name_str.len() as u32);
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic).cast::<AstTypePack>()
  }
}
