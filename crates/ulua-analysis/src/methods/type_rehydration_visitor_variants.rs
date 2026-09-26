use core::{
  mem::size_of,
  ptr::{null_mut, write},
};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{
    allocator::Allocator, ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName,
    ast_table_indexer::AstTableIndexer, ast_table_prop::AstTableProp, ast_type::AstType,
    ast_type_function::AstTypeFunction, ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList, ast_type_or_pack::AstTypeOrPack, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_union::AstTypeUnion, location::Location,
  },
  type_aliases::ast_argument_name::AstArgumentName,
};

use crate::{
  functions::{
    alloc_nul_string::{alloc_name, alloc_nul_string},
    flatten_type_pack::flatten_type_pack_id,
    get_name_type_attach::get_name_allocator_synthetic_names_generic_type,
    get_singleton_type::get_singleton_type,
    get_type::get as get_type,
    get_type_pack::get as get_type_pack,
  },
  records::{
    any_type::AnyType,
    arena_handle::alias,
    blocked_type::BlockedType,
    boolean_singleton::BooleanSingleton,
    bound::Bound,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_pack::GenericTypePack,
    intersection_type::IntersectionType,
    lazy_type::LazyType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType,
    primitive_type::{PrimitiveType, Type},
    recursion_counter::RecursionCounter,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    type_rehydration_visitor::TypeRehydrationVisitor,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{
    error_type::ErrorType, synthetic_names::SyntheticNames, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl TypeRehydrationVisitor {
  /// C++ `AstType* operator()(const PrimitiveType& ptv)`.
  pub fn rehydrate_primitive(&mut self, ptv: &PrimitiveType) -> *mut AstType {
    let name = match ptv.r#type {
      Type::NilType => AstName::from_static(b"nil"),
      Type::Boolean => AstName::from_static(b"boolean"),
      Type::Number => AstName::from_static(b"number"),
      Type::Integer => AstName::from_static(b"integer"),
      Type::String => AstName::from_static(b"string"),
      Type::Thread => AstName::from_static(b"thread"),
      Type::Buffer => AstName::from_static(b"buffer"),
      Type::Function => AstName::from_static(b"function"),
      Type::Table => AstName::from_static(b"table"),
    };

    // Safety: alloc_named_reference 的 arena 前提见该方法；此处分配基本类型
    // 名引用节点。
    self.alloc_named_reference(name)
  }

  /// 分配「默认 location、无 parameters、非展开形态」的命名 `AstTypeReference`
  /// 引用节点——`*blocked*`/`free`/`<Lazy?>` 等十余个原子 `rehydrate_*` 分支
  /// 共用的骨架（cpp 侧各分支手抄的同款 ctor 实参列表）。
  ///
  /// Safety: `self.allocator` 是构造期由 TypeAttacher 存入的 SourceModule AST
  /// bump arena 指针，非空、对齐且整个 attach 期间存活，此刻仅本 visitor 独占
  /// 访问；借出的 `&mut` 只覆盖本函数内的 alloc，随函数返回释放。
  fn alloc_named_reference(&mut self, name: AstName) -> *mut AstType {
    // Safety: 前置条件见本方法 doc。
    let allocator = self.allocator_mut();
    let reference = AstTypeReference::new(
      Location::default(),
      None,
      name,
      None,
      Location::default(),
      false,
      AstArray::default(),
    );
    allocator.alloc(reference).cast::<AstType>()
  }

  #[inline]
  pub fn rehydrate_blocked(&mut self, _btv: &BlockedType) -> *mut AstType {
    // Safety: alloc_named_reference 的 arena 前提见该方法；此处分配
    // `*blocked*` 引用节点。
    self.alloc_named_reference(AstName::from_static(b"*blocked*"))
  }

  pub fn rehydrate_pending_expansion(&mut self, _petv: &PendingExpansionType) -> *mut AstType {
    // Safety: alloc_named_reference 的 arena 前提见该方法；此处分配
    // `*pending-expansion*` 引用节点。
    self.alloc_named_reference(AstName::from_static(b"*pending-expansion*"))
  }

  pub fn rehydrate_singleton(&mut self, stv: &SingletonType) -> *mut AstType {
    if let Some(bs) = get_singleton_type::<BooleanSingleton>(stv) {
      let location = Location::default();
      // Safety: arena 指针由 TypeAttacher 构造期传入、visit 期间存活且仅本
      // visitor 独占；这次 `&mut` 借出仅用于 alloc 布尔单例节点，随 return 结束。
      let allocator = self.allocator_mut();
      return allocator
        .alloc(AstTypeSingletonBool::new(location, bs.value))
        .cast::<AstType>();
    }
    if let Some(ss) = get_singleton_type::<StringSingleton>(stv) {
      let location = Location::default();
      let value = {
        let s = &ss.value;
        let data = s.as_ptr().cast_mut();
        let size = s.len();
        AstArray { data, size }
      };
      // Safety: 再次重借同一存活 arena 指针（bool 分支未走到，先前无借出仍
      // 活着）分配字符串单例节点；`ss` 借自入参 `stv`（类型 arena 里的节点，
      // 与 `&mut self` 出处无关），其 String 字节在节点存活期间不移动，
      // AstArray 借用之与 C++ `ss->value.c_str()` 同寿命语义。
      let allocator = self.allocator_mut();
      return allocator
        .alloc(AstTypeSingletonString::new(location, value))
        .cast::<AstType>();
    }
    // 未知单例变体：B 型（visitor 可空返回折叠）——cpp `TypeAttach.cpp:142-143`
    // 同款 `else return nullptr`：rehydration 对该分支的契约就是「无可重建节点」，
    // 返回类型 `*mut AstType` 与 cpp `AstType*` 一致；消费方（attach_type_data
    // 与各 visit 分支）把结果直存进 ulua-ast 结点的 `*mut AstType` 字段/AstArray
    // 槽位，读取侧经 optional_node::node_opt 折回 Option。
    null_mut()
  }

  #[inline]
  pub fn rehydrate_any(&mut self, _any: &AnyType) -> *mut AstType {
    // Safety: alloc_named_reference 的 arena 前提见该方法；此处分配 AnyType
    // 的空名（AstName::new()）引用节点。
    self.alloc_named_reference(AstName::new())
  }

  #[inline]
  pub fn rehydrate_no_refine(&mut self, _no_refine: &NoRefineType) -> *mut AstType {
    // Safety: alloc_named_reference 的 arena 前提见该方法；此处分配
    // `*no-refine*` 引用节点。
    self.alloc_named_reference(AstName::from_static(b"*no-refine*"))
  }

  pub fn rehydrate_table(&mut self, ttv: &TableType) -> *mut AstType {
    // Safety: `&mut self.count` 指向本 visitor 自身的 i32 字段，非空且对齐；
    // guard 只持有该裸指针并即时增减，其生命周期不超过 `&mut self`，嵌套
    // guard 以 LIFO 方式进出（等价 C++ `RecursionCounter counter(&count)`）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    if let Some(ref name) = ttv.name
      && !self.options.banned_names.contains_str(name.as_str())
    {
      let params_size =
        ttv.instantiated_type_params.len() + ttv.instantiated_type_pack_params.len();
      // Safety: `self.allocator_mut()` 重借构造期传入的存活 arena 指针；allocate
      // 返回 `size_of::<AstTypeOrPack>() * params_size` 字节的非空连续块
      // （bump arena 按 ≥8 对齐切块，AstTypeOrPack 是「指针 + 判别字」的 enum、
      // 对齐 8，满足）。
      let params_data = self
        .allocator_mut()
        .allocate(size_of::<AstTypeOrPack>() * params_size)
        .cast::<AstTypeOrPack>();

      let mut idx = 0;
      for &ty_param in &ttv.instantiated_type_params {
        // Safety: `ty_param` 借自 ttv.instantiated_type_params，是指向
        // TypeArena 分配的 Type 节点的非空地址；attach 期间 arena 节点地址
        // 稳定，满足 visit_type 对 `ty` 的契约。
        let rehydrated = self.visit_type(ty_param);
        // Safety: params_data 指向上方刚申请的 params_size 元素块，idx 每写
        // 一次自增、恒 < params_size，每槽恰写一次；AstTypeOrPack 无 Drop，
        // write 不读未初始化旧值（对应 C++ parameters.data[i] 赋值）。
        // `from_type` 即 cpp `AstTypeOrPack{rehydrated, nullptr}` 的定形构造。
        unsafe {
          write(params_data.add(idx), AstTypeOrPack::from_type(rehydrated));
        }
        idx += 1;
      }

      for &tp_param in &ttv.instantiated_type_pack_params {
        let rehydrated = self.rehydrate(tp_param);
        // Safety: idx 延续上一循环的编号，两轮总写入次数 = 两类参数向量长度
        // 之和 = params_size，仍在块内；槽位原本未初始化，write 完成初始化。
        // `rehydrated` 为 rehydrate 从 arena 产出的 AstTypePack*，`from_type_pack`
        // 即 cpp `AstTypeOrPack{nullptr, rehydrated}` 的定形构造。
        unsafe {
          write(
            params_data.add(idx),
            AstTypeOrPack::from_type_pack(rehydrated),
          );
        }
        idx += 1;
      }

      // Safety: 此前借出的 arena `&mut` 均已随语句结束释放，此处重新独占借出，
      // 分配表类型名的 NUL 结尾拷贝与最终 AstTypeReference 节点。
      let allocator = self.allocator_mut();
      let name_ast = alloc_name(allocator, name);

      let parameters = AstArray {
        data: params_data,
        size: params_size,
      };

      let ref_node = AstTypeReference::new(
        Location::default(),
        None,
        name_ast,
        None,
        Location::default(),
        params_size != 0,
        parameters,
      );

      return allocator.alloc(ref_node).cast::<AstType>();
    }

    if self.has_seen(ttv as *const TableType as *const ()) {
      // Safety: 环检测分支再次重借存活的 arena 指针，用于拷贝 <Cycle>/名字并
      // 分配引用节点；同一时刻无其他 Allocator 借用。
      let allocator = self.allocator_mut();
      let name_ast = match &ttv.name {
        Some(name) => alloc_name(allocator, name),
        None => alloc_name(allocator, "<Cycle>"),
      };

      let ref_node = AstTypeReference::new(
        Location::default(),
        None,
        name_ast,
        None,
        Location::default(),
        false,
        // 对应 cpp 环检测分支（TypeAttach.cpp:181-187）走 6 参 ctor，parameters
        // 形参默认 `{}`（Ast.h:1236）即 `{nullptr,0}` 空数组——`AstArray::EMPTY`
        // 是其定形构造，手写字面量消失。
        AstArray::EMPTY,
      );

      return allocator.alloc(ref_node).cast::<AstType>();
    }

    let props_size = ttv.props.len();
    // Safety: 重借存活 arena 申请 props.len() 个 AstTableProp 的连续块，块非空
    // 且 ≥8 对齐覆盖元素布局；idx 的槽位记账与 C++ TypeAttach.cpp
    // props.data[idx++] 完全一致，写入均在块内。
    let props_data = self
      .allocator_mut()
      .allocate(size_of::<AstTableProp>() * props_size)
      .cast::<AstTableProp>();

    let mut idx = 0;
    for (prop_name, prop) in &ttv.props {
      // Safety: 嵌套 guard 同样借用本对象 count 字段（有效对齐的 i32 指针），
      // 先于外层 _counter 释放，LIFO 恢复计数。
      let _counter_inner = RecursionCounter::recursion_counter_i32(&mut self.count);

      let name_ast = {
        // Safety: 短生命借出 arena 仅用于 prop 名的 NUL 结尾拷贝，块尾即释放，
        // 不与后续 visit_type 内部重新借出的 arena 借用交叠。
        let allocator = self.allocator_mut();
        alloc_name(allocator, prop_name)
      };

      if prop.is_shared() {
        // Safety: is_shared() 蕴含 read_ty 为 Some，其值是 TypeArena 中存活的
        // 非空 TypeId（ttv 自身字段派生），visit_type 契约成立。
        let read_ty_rehydrated = self.visit_type(
          prop
            .read_ty
            .expect("is_shared() 蕴含 read_ty 为 Some（上方 Safety 契约）"),
        );
        // Safety: 共享读写属性占 1 个槽，idx 每次写入自增一次，恒 < props_size
        // （与 C++ 同一记账）；槽位未初始化且 AstTableProp 无 Drop，write 就地初始化。
        unsafe {
          write(
            props_data.add(idx),
            AstTableProp {
              name: name_ast,
              location: Location::default(),
              r#type: read_ty_rehydrated,
              access: AstTableAccess::ReadWrite,
              access_location: None,
            },
          );
        }
        idx += 1;
      } else {
        if let Some(read_ty) = prop.read_ty {
          // Safety: read_ty 由 if-let 保证 Some，指向类型 arena 的稳定非空节点。
          let read_ty_rehydrated = self.visit_type(read_ty);
          // Safety: 非共享属性按 C++ 同不变式至多占一个读槽（读写异类型的属性
          // 不进入本分支序列，越界风险与上游 oracle 等同）；write 初始化未写过的槽。
          unsafe {
            write(
              props_data.add(idx),
              AstTableProp {
                name: name_ast,
                location: Location::default(),
                r#type: read_ty_rehydrated,
                access: AstTableAccess::Read,
                access_location: None,
              },
            );
          }
          idx += 1;
        }

        if let Some(write_ty) = prop.write_ty {
          // Safety: write_ty 同样为 Some 的存活 arena TypeId。
          let write_ty_rehydrated = self.visit_type(write_ty);
          // Safety: 写只读互斥分支的对应槽位，idx 延续块内推进；槽未初始化、
          // 元素无 Drop，write 单次初始化。
          unsafe {
            write(
              props_data.add(idx),
              AstTableProp {
                name: name_ast,
                location: Location::default(),
                r#type: write_ty_rehydrated,
                access: AstTableAccess::Write,
                access_location: None,
              },
            );
          }
          idx += 1;
        }
      }
    }

    let indexer = if let Some(ref indexer_ref) = ttv.indexer {
      // Safety: 又一处嵌套 RAII 计数 guard，借用本对象 count 字段，随块尾释放。
      let _counter_indexer = RecursionCounter::recursion_counter_i32(&mut self.count);

      // Safety: index_type 是 TableIndexer 保存的存活 arena 节点地址（非空），
      // 与 C++ ttv.indexer->indexType->ty 同前提。
      let index_type = self.visit_type(indexer_ref.index_type);
      // Safety: index_result_type 同上为 TableIndexer 保存的存活 arena 节点
      // 地址（C++ ttv.indexer->indexResultType->ty 同前提）。
      let result_type = self.visit_type(indexer_ref.index_result_type);

      let indexer_node = AstTableIndexer {
        index_type,
        result_type,
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      };

      // Safety: arena 指针有效且独占，借出仅用于 alloc AstTableIndexer 节点。
      let allocator = self.allocator_mut();
      allocator.alloc(indexer_node)
    } else {
      // B 型（落点字段布局契约）：cpp `TypeAttach.cpp:229` 同款
      // `AstTableIndexer* indexer = nullptr;`，仅当 `ttv.indexer` 存在才建节点；
      // 落点 `AstTypeTable.indexer` 为 ulua-ast bump 结点字段 `*mut
      // AstTableIndexer`（读取方经 optional_node 门面判空），空即「无 indexer」。
      null_mut()
    };

    let props_array = AstArray {
      data: props_data,
      size: props_size,
    };

    let table_node = AstTypeTable::new(Location::default(), props_array, indexer);

    // Safety: 最后一次重借存活的 arena 指针分配 AstTypeTable 节点，此前所有
    // arena/子调用借用均已释放。
    let allocator = self.allocator_mut();
    allocator.alloc(table_node).cast::<AstType>()
  }

  /// C++ `AstType* operator()(const MetatableType& mtv)` —
  /// `return Luau::visit(*this, mtv.table->ty);`.
  pub fn rehydrate_metatable(&mut self, mtv: &MetatableType) -> *mut AstType {
    // Safety: mtv.table() 返回 MetatableType 内嵌的 TypeId（记录注释：恒指向
    // 存活的 TableType 节点），arena 节点在 attach 期间非空且地址稳定。
    self.visit_type(mtv.table())
  }

  pub fn rehydrate_extern(&mut self, etv: &ExternType) -> *mut AstType {
    // Safety: &mut self.count 为本对象 i32 字段的对齐有效指针，guard 先于
    // self 析构，嵌套增减 LIFO，与 C++ RecursionCounter(&count) 等价。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    let name = {
      // Safety: 借出构造期传入的存活 arena 拷贝 ExternType 名，块尾归还借用。
      let allocator = self.allocator_mut();
      alloc_name(allocator, &etv.name)
    };

    if !self.options.expand_extern_type_props
      || self.has_seen(etv as *const ExternType as *const ())
      || self.count > 1
    {
      // Safety: alloc_named_reference 重借的是仍存活的 arena 指针（Name 借自
      // 入参 etv，出处与 visitor 无关），短路分支直接 alloc 引用节点。
      return self.alloc_named_reference(name);
    }

    let props_size = etv.props.len();
    // Safety: 存活 arena 申请 etv.props.len() 个 AstTableProp 的连续块，尺寸/
    // 对齐覆盖元素布局；下方 .write() 目的地址恒在块内（同 C++ idx 记账）。
    let props_data = self
      .allocator_mut()
      .allocate(size_of::<AstTableProp>() * props_size)
      .cast::<AstTableProp>();

    let mut idx = 0;
    for (prop_name, prop) in &etv.props {
      let name = {
        // Safety: 短生命 arena 借出仅用于属性名拷贝，块尾释放。
        let allocator = self.allocator_mut();
        alloc_nul_string(allocator, prop_name)
      };

      if prop.is_shared() {
        // Safety: 共享属性蕴含 read_ty Some，值为 ExternType 字段里的存活
        // arena 节点地址；visit_type 契约满足。
        let read_type_ptr = self.visit_type(
          prop
            .read_ty
            .expect("is_shared() 蕴含 read_ty 为 Some（上方 Safety 契约）"),
        );
        // Safety: props_data.add(idx) 在已申请块内（每属性 1 槽，idx 与写入
        // 一一对应）；槽未初始化、元素无 Drop，ptr write 单次初始化。
        unsafe {
          props_data.add(idx).write(AstTableProp {
            name: AstName::ast_name_u8(name),
            location: Location::default(),
            r#type: read_type_ptr,
            access: AstTableAccess::ReadWrite,
            access_location: None,
          });
        }
        idx += 1;
      } else {
        if let Some(read_ty) = prop.read_ty {
          // Safety: if-let 保证 read_ty 非空且为 arena 稳定节点。
          let read_type_ptr = self.visit_type(read_ty);
          // Safety: 读分支独占下一槽位，idx < props_size 沿用 C++ 属性级
          // 不变式；write 跳过对未初始化字节的读取。
          unsafe {
            props_data.add(idx).write(AstTableProp {
              name: AstName::ast_name_u8(name),
              location: Location::default(),
              r#type: read_type_ptr,
              access: AstTableAccess::Read,
              access_location: None,
            });
          }
          idx += 1;
        }

        if let Some(write_ty) = prop.write_ty {
          // Safety: write_ty 为 Some 的存活 arena TypeId。
          let write_type_ptr = self.visit_type(write_ty);
          // Safety: 写分支槽位延续块内推进，元素无 Drop，单次 write 初始化。
          unsafe {
            props_data.add(idx).write(AstTableProp {
              name: AstName::ast_name_u8(name),
              location: Location::default(),
              r#type: write_type_ptr,
              access: AstTableAccess::Write,
              access_location: None,
            });
          }
          idx += 1;
        }
      }
    }

    let props = AstArray {
      data: props_data,
      size: idx,
    };

    let indexer = if let Some(ref indexer_data) = etv.indexer {
      // Safety: 嵌套计数 guard 借用自身 count 字段，随 if 块尾先于外层释放。
      let _inner_counter = RecursionCounter::recursion_counter_i32(&mut self.count);

      // Safety: index_type 是 ExternType indexer 保存的存活 arena 类型节点地址
      // （C++ etv.indexer->indexType->ty 同前提）。
      let index_type = self.visit_type(indexer_data.index_type);
      // Safety: index_result_type 同上为 ExternType indexer 的存活 arena 类型
      // 节点地址（C++ etv.indexer->indexResultType->ty 同前提）。
      let result_type = self.visit_type(indexer_data.index_result_type);

      // Safety: arena 指针存活且独占，借出仅用于 alloc AstTableIndexer。
      let allocator = self.allocator_mut();
      allocator.alloc(AstTableIndexer {
        index_type,
        result_type,
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      })
    } else {
      // B 型（落点字段布局契约）：cpp `TypeAttach.cpp:294` 同款
      // `AstTableIndexer* indexer = nullptr;`（ExternType 分支）；落点为 ulua-ast
      // `AstTypeTable.indexer: *mut AstTableIndexer`，空即「无 indexer」。
      null_mut()
    };

    let table = AstTypeTable::new(Location::default(), props, indexer);
    // Safety: 收尾再借 arena 分配 AstTypeTable，前述借用均已结束。
    let allocator = self.allocator_mut();
    allocator.alloc(table).cast::<AstType>()
  }

  /// 将扁平化后的类型向量逐个 rehydrate，写入 AST 分配器的原始数组。
  ///
  /// # Safety
  /// 调用方须保证：
  /// - `self.allocator` 指向在 attach 期间存活的 `Allocator`（构造 visitor 时
  ///   由 `TypeAttacher` 传入的 SourceModule arena，见 `attach_type_data`）；
  /// - `tys` 中每个 `TypeId` 非空且指向类型 arena 中存活的 `Type` 节点
  ///   （即 `visit_type` 自身的 `ty` 契约）。
  unsafe fn rehydrate_types(&mut self, tys: &[TypeId]) -> AstArray<*mut AstType> {
    // Safety: 在存活的 arena 指针上 allocate  tys.len() 个指针元素的连续块
    //（bump 分配、非空、≥8 对齐满足指针尺寸）；i 来自 enumerate，data.add(i)
    // 恒在块内且每槽恰一次直接赋值，`*mut AstType` 无 Drop；visit_type 的入参
    // 为 tys 中的存活 arena TypeId；计数器 guard 借用本对象 count 字段。
    unsafe {
      let data = self
        .allocator_mut()
        .allocate(size_of::<*mut AstType>() * tys.len()) as *mut *mut AstType;
      for (i, &ty) in tys.iter().enumerate() {
        let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);
        *data.add(i) = self.visit_type(ty);
      }
      AstArray {
        data,
        size: tys.len(),
      }
    }
  }

  /// 尾随类型包 rehydrate；无尾随则空指针。
  ///
  /// B 型（落点字段布局契约）：cpp `TypeAttach.cpp:345/374` 同款
  /// `AstTypePack* argTailAnnotation = nullptr; if (argTail) ...`——返回值直存
  /// ulua-ast `AstTypeList.tail_type` 裸指针字段（含嵌于 `AstTypePackExplicit`
  /// 内的场合），空即「无尾随」（Ast.h:133 明示合法态），读取方经
  /// optional_node::node_opt / `AstTypeList::tail()` 门面折回 Option。
  fn rehydrate_tail(&mut self, tail: Option<TypePackId>) -> *mut AstTypePack {
    tail.map_or_else(null_mut, |tp| self.rehydrate(tp))
  }

  pub fn rehydrate_function(&mut self, ftv: &FunctionType) -> *mut AstType {
    // Safety: &mut self.count 是本对象字段的对齐有效指针，guard 贯穿整个
    // 函数体（先于 self 失效），增减 LIFO 与 C++ 一致。
    let _recursion_counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    if self.has_seen(ftv as *const FunctionType as *const ()) {
      // Safety: arena 指针构造期传入、visit 期间存活，独占借出分配 <Cycle>
      // 引用节点（alloc_named_reference 前提见该方法）。
      return self.alloc_named_reference(AstName::from_static(b"<Cycle>"));
    }

    // generics
    // 对应 cpp `TypeAttach.cpp:314-321`：`AstArray<AstGenericType*> generics` 先
    // 取容量、命中才逐项填充。「先塞 null 后接线」的占位改为 if/else 构造即
    // 定形——空泛型集即 cpp `{nullptr,0}` 的定形构造 `AstArray::EMPTY`。
    let generics_array = if ftv.generics.is_empty() {
      AstArray::EMPTY
    } else {
      // Safety: 重借存活 arena 申请 generics.len() 个 `*mut AstGenericType`
      // 元素的块；bump 分配返回非空、≥8 对齐，满足指针尺寸。
      let generics_ptr = self
        .allocator_mut()
        .allocate(size_of::<*mut AstGenericType>() * ftv.generics.len())
        as *mut *mut AstGenericType;
      let mut num_generics: usize = 0;
      for &gen_id in &ftv.generics {
        if let Some(r#gen) = get_type::<GenericType>(gen_id) {
          // `r#gen.name` is a Rust String (not NUL-terminated); copy it into
          // the AST allocator with a trailing NUL so AstName's borrowed
          // C-string pointer is safe to read (was UB: read past the bytes).
          // Safety: 临时借出 arena 分配 NUL 结尾拷贝，调用返回即归还借用；
          // r#gen 指向 arena 中存活的 GenericType 节点。
          let name_ptr = alloc_nul_string(self.allocator_mut(), &r#gen.name);
          let ast_gen =
            AstGenericType::new(Location::default(), AstName::ast_name_u8(name_ptr), None);
          // Safety: 上一借出（allocate_string 实参）已结束，重新独占借 arena
          // 分配 AstGenericType 节点。
          let allocator = self.allocator_mut();
          // Safety: num_generics 仅在命中 GenericType 时自增，恒 < generics.len()
          //（即块容量）；目标槽是未初始化的指针单元、无 Drop，直接赋值=单次 store。
          unsafe { *generics_ptr.add(num_generics) = allocator.alloc(ast_gen) };
          num_generics += 1;
        }
      }
      AstArray {
        data: generics_ptr,
        size: num_generics,
      }
    };

    // generic packs
    // 同 generics：占位 null 数组改 if/else 构造即定形（cpp
    // `TypeAttach.cpp:324-331`，空集即 `AstArray::EMPTY`）。
    let generic_packs_array = if ftv.generic_packs.is_empty() {
      AstArray::EMPTY
    } else {
      // Safety: 存活 arena 申请 generic_packs.len() 个指针元素的连续块，
      // 非空、对齐满足（同 generics_ptr 的论证）。
      let packs_ptr = self
        .allocator_mut()
        .allocate(size_of::<*mut AstGenericTypePack>() * ftv.generic_packs.len())
        as *mut *mut AstGenericTypePack;
      let mut num_packs: usize = 0;
      for &pack_id in &ftv.generic_packs {
        if let Some(pack) = get_type_pack::<GenericTypePack>(pack_id) {
          // Safety: 临时借出 arena 拷贝 pack 名（arena 节点上的 String 存活），
          // 实参借用随调用结束。
          let name_ptr = alloc_nul_string(self.allocator_mut(), &pack.name);
          let ast_pack =
            AstGenericTypePack::new(Location::default(), AstName::ast_name_u8(name_ptr), None);
          // Safety: 重新独占借 arena 分配 AstGenericTypePack 节点。
          let allocator = self.allocator_mut();
          // Safety: num_packs 只在命中时自增，不超过 packs_ptr 块容量；
          // 指针槽无 Drop，单次直接赋值完成写入。
          unsafe { *packs_ptr.add(num_packs) = allocator.alloc(ast_pack) };
          num_packs += 1;
        }
      }
      AstArray {
        data: packs_ptr,
        size: num_packs,
      }
    };

    // argument types
    let (arg_vector, arg_tail) = flatten_type_pack_id(ftv.arg_types);
    // Safety: arg_vector 各元素是 flatten 自 ftv.arg_types 的存活 arena TypeId；
    // arena 指针满足 rehydrate_types 的构造期契约。
    let arg_types_array = unsafe { self.rehydrate_types(&arg_vector) };
    let arg_tail_annotation = self.rehydrate_tail(arg_tail);

    // argument names
    // 同 generics：占位 null 数组改 if/else 构造即定形（cpp
    // `TypeAttach.cpp:349-358`，空集即 `{nullptr,0}` = `AstArray::EMPTY`）。
    let arg_names_array = if ftv.arg_names.is_empty() {
      AstArray::EMPTY
    } else {
      // Safety: 存活的 arena 指针申请 arg_names.len() 个
      // `Option<AstArgumentName>`（裸指针+坐标组成、无 Drop、对齐 ≤8）元素的
      // 连续块，尺寸与对齐均覆盖元素布局。
      let names_ptr = self
        .allocator_mut()
        .allocate(size_of::<Option<AstArgumentName>>() * ftv.arg_names.len())
        as *mut Option<AstArgumentName>;
      for (i, arg_opt) in ftv.arg_names.iter().enumerate() {
        let slot: Option<AstArgumentName> = if let Some(ref arg) = *arg_opt {
          // Safety: 短生命 arena 借出用于参数名 NUL 拷贝，实参借用即借即还。
          let name_ptr = alloc_nul_string(self.allocator_mut(), &arg.name);
          let name = AstName::ast_name_u8(name_ptr);
          Some((name, Location::default()))
        } else {
          None
        };
        // Safety: i 枚举向量元素、恒 < 块容量；槽位从未初始化，write 恰一次
        // 初始化，无旧值 double-drop。
        unsafe { write(names_ptr.add(i), slot) };
      }
      AstArray {
        data: names_ptr,
        size: ftv.arg_names.len(),
      }
    };

    // return types
    let (ret_vector, ret_tail) = flatten_type_pack_id(ftv.ret_types);
    // Safety: 同 arg_vector——ret_vector 元素皆为类型 arena 中的存活节点，
    // arena 指针由构造契约保证有效。
    let return_types_array = unsafe { self.rehydrate_types(&ret_vector) };

    let ret_tail_annotation = self.rehydrate_tail(ret_tail);

    // Safety: 收尾的 arena 借出连续用于分配返回类型表与最终 AstTypeFunction
    // 节点；借存续期间无其他 arena 借用与之交叠。
    let allocator = self.allocator_mut();
    let return_annotation = allocator
      .alloc(AstTypePackExplicit::new(
        Location::default(),
        AstTypeList {
          types: return_types_array,
          tail_type: ret_tail_annotation,
        },
      ))
      .cast::<AstTypePack>();

    let func_type = AstTypeFunction::ast_type_function_location_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
            Location::default(),
            generics_array,
            generic_packs_array,
            AstTypeList {
                types: arg_types_array,
                tail_type: arg_tail_annotation,
            },
            arg_names_array,
            return_annotation,
        );

    allocator.alloc(func_type).cast::<AstType>()
  }

  pub fn rehydrate_error(&mut self, _err: &ErrorType) -> *mut AstType {
    // Safety: arena 指针构造期由 TypeAttacher 传入、整程存活（alloc_named_reference
    // 前提见该方法），独占借出分配 Unifiable<Error> 引用节点。
    self.alloc_named_reference(AstName::from_static(b"Unifiable<Error>"))
  }

  #[inline]
  pub fn rehydrate_generic(&mut self, gtv: &GenericType) -> *mut AstType {
    // Safety: self.allocator 是构造时传入的 SourceModule arena 指针，rehydrate
    // 全程存活且仅本 visitor 访问；此处借出传给 getName 做泛型名的记忆化，块尾
    // 即归还，与随后 alloc_named_reference 的重借顺序串接、无并存借用。
    let allocator: &mut Allocator = self.allocator_mut();
    // Safety: self.synthetic_names 构造时由 `&mut ta.synthetic_names` 字段存入
    //（见 type_attacher 调用链），TypeAttacher 比 visitor 活得久；此刻仅此一处
    // 重借 &mut，getName 在该映射里做泛型名的记忆化。
    let synthetic_names: &mut SyntheticNames = alias(self.synthetic_names);
    let name_ptr = get_name_allocator_synthetic_names_generic_type(allocator, synthetic_names, gtv);
    self.alloc_named_reference(AstName::ast_name_u8(name_ptr))
  }

  /// C++ `AstType* operator()(const Unifiable::Bound<TypeId>& bound)` —
  /// `return Luau::visit(*this, bound.boundTo->ty);`.
  pub fn rehydrate_bound(&mut self, bound: &Bound<TypeId>) -> *mut AstType {
    // Safety: bound.bound_to 是 Bound 变体内存的目标 TypeId（visit_type 里由
    // 裸指针重建），指向 arena 中存活的解绑目标节点，非空。
    self.visit_type(bound.bound_to)
  }

  pub fn rehydrate_free(&mut self, _ft: &FreeType) -> *mut AstType {
    // Safety: 存活且独占的 arena 指针（alloc_named_reference 前提见该方法），
    // 借出分配 free 引用节点后随函数释放。
    self.alloc_named_reference(AstName::from_static(b"free"))
  }

  pub fn rehydrate_union(&mut self, uv: &UnionType) -> *mut AstType {
    let size = uv.options.len();
    // Safety: self.allocator_mut() 解引用构造期传入的存活 arena 指针，allocate
    // 返回 options.len() 个 `*mut AstType` 的非空 ≥8 对齐块。
    let data = self
      .allocator_mut()
      .allocate(size_of::<*mut AstType>() * size) as *mut *mut AstType;

    for (i, &option_ty) in uv.options.iter().enumerate() {
      // C++ `unionTypes.data[i] = Luau::visit(*this, uv.options[i]->ty);`
      // Safety: option_ty 借自 uv.options，是类型 arena 里稳定的非空节点地址。
      let rehydrated = self.visit_type(option_ty);
      // Safety: enumerate 保证 i < size，槽位在已申请块内；目标类型
      // `*mut AstType` 无 Drop，直接赋值为单次 store，完成初始化。
      unsafe { *data.add(i) = rehydrated };
    }

    let union_types = AstArray { data, size };

    // Safety: 重新独占借 arena 分配 AstTypeUnion 节点。
    let alloc = self.allocator_mut();
    alloc
      .alloc(AstTypeUnion::new(Location::default(), union_types))
      .cast::<AstType>()
  }

  pub fn rehydrate_intersection(&mut self, uv: &IntersectionType) -> *mut AstType {
    let size = uv.parts.len();
    // Safety: arena 指针存活、独占；allocate 申请 parts.len() 个指针元素块，
    // bump 分配非空且满足对齐。
    let data = self
      .allocator_mut()
      .allocate(size_of::<*mut AstType>() * size) as *mut *mut AstType;

    for (i, &part_ty) in uv.parts.iter().enumerate() {
      // C++ `intersectionTypes.data[i] = Luau::visit(*this, uv.parts[i]->ty);`
      // Safety: part_ty 为 uv.parts 保存的 arena 存活节点地址，非空。
      let ast_part = self.visit_type(part_ty);
      // Safety: i 由 enumerate 限制在块容量内，指针单元无 Drop，赋值即写入。
      unsafe { *data.add(i) = ast_part };
    }

    let intersection_types = AstArray { data, size };

    let location = Location::default();
    // Safety: 末次借 arena 分配 AstTypeIntersection。
    let alloc = self.allocator_mut();
    alloc
      .alloc(AstTypeIntersection::new(location, intersection_types))
      .cast::<AstType>()
  }

  #[inline]
  pub fn rehydrate_lazy(&mut self, ltv: &LazyType) -> *mut AstType {
    // C++ `if (TypeId unwrapped = ltv.unwrapped.load()) return Luau::visit(*this, unwrapped->ty);`
    let unwrapped: TypeId = ltv.unwrapped;
    if !unwrapped.is_null() {
      // Safety: 已通过判空（对应 C++ 原子 load 结果非空），unwrapped 是
      // LazyType 记录的已展开目标，指向 arena 存活节点。
      return self.visit_type(unwrapped);
    }

    // Safety: 存活 arena 指针的独占借出（alloc_named_reference 前提见该方法），
    // 分配 <Lazy?> 引用节点。
    self.alloc_named_reference(AstName::from_static(b"<Lazy?>"))
  }

  pub fn rehydrate_unknown(&mut self, _ttv: &UnknownType) -> *mut AstType {
    // cpp `UnknownType` 分支（TypeAttach.cpp:430-432）走 6 参 ctor，parameters
    // 默认 `{}`（Ast.h:1236）= 空数组定形构造，与骨架的 `AstArray::default()`
    // 同一值。Safety: 构造期 arena 指针存活且独占（alloc_named_reference 前提
    // 见该方法）。
    self.alloc_named_reference(AstName::from_static(b"unknown"))
  }

  #[inline]
  pub fn rehydrate_never(&mut self, _ttv: &NeverType) -> *mut AstType {
    // Safety: arena 重借合法（指针存活、无并存别名，alloc_named_reference
    // 前提见该方法），仅用于 never 引用节点。
    self.alloc_named_reference(AstName::from_static(b"never"))
  }

  #[inline]
  pub fn rehydrate_negation(&mut self, ntv: &NegationType) -> *mut AstType {
    // C++ `params.data[0] = AstTypeOrPack{Luau::visit(*this, ntv.ty->ty), nullptr};`
    // Safety: ntv.ty 是 NegationType 内嵌的被否定类型，arena 存活非空节点。
    let ty_rehydrated = self.visit_type(ntv.ty);

    // Safety: 借出存活的 arena 指针，覆盖 params 槽申请与末尾 AstTypeReference
    // 分配，整个区间仅此一借。
    let allocator: &mut Allocator = self.allocator_mut();

    let params_data: *mut AstTypeOrPack = allocator
      .allocate(size_of::<AstTypeOrPack>())
      .cast::<AstTypeOrPack>();
    // Safety: params_data 指向刚申请的单元素 AstTypeOrPack 块；元素无 Drop，
    // 对未初始化槽的直接赋值退化为一次 store（对应 C++ params.data[0] = ...）。
    // `from_type` 即 cpp `AstTypeOrPack{rehydrated, nullptr}` 的定形构造。
    unsafe {
      *params_data = AstTypeOrPack::from_type(ty_rehydrated);
    }

    let params = AstArray {
      data: params_data,
      size: 1,
    };

    let reference = AstTypeReference::new(
      Location::default(),
      None,
      AstName::from_static(b"negate"),
      None,
      Location::default(),
      true,
      params,
    );

    allocator.alloc(reference).cast::<AstType>()
  }

  pub fn rehydrate_type_function_instance(
    &mut self,
    tfit: &TypeFunctionInstanceType,
  ) -> *mut AstType {
    let name = tfit.user_func_name.unwrap_or_else(|| {
      // Safety: `tfit.function` 为 NonNull<TypeFunction>（类型不变式保证非空
      // 对齐），指向注册表/内置类型表持有的 TypeFunction 定义，其寿命覆盖持有
      // 该实例类型的 arena 节点；此处仅共享借用读取其 name 字段。
      let func = unsafe { &*tfit.function.as_ptr() };
      AstName::from_raw_parts(func.name.as_ptr(), func.name.len() as u32)
    });
    // Safety: 存活 arena 指针的重借（alloc_named_reference 前提见该方法），
    // 分配类型函数名引用节点。
    self.alloc_named_reference(name)
  }
}
