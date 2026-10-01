//! `TypeRehydrationVisitor` 的逐变体 rehydrate 实现（cpp `TypeAttach.cpp` 的
//! `Luau::visit` 重载族）。
//!
//! 返回值与中间槽位统一为 ulua-ast 的 arena 节点指针形态（`*mut AstType`、
//! `AstArray<*mut AstType>`、`AstTypeList.tail_type`）——这是 AST 存储面的
//! 既定数据模型（见 `ulua-ast` 的 `AstArray`/`Allocator` 文档：bump arena、
//! 节点地址在 attach 全程稳定、null 哨兵经 `opt_node` 单点写出）。本模块只
//! 产出/装填这些槽位：分配经 `allocator_mut`（arena 句柄收口点）、可空槽经
//! `opt_node(None)`、环检测键经 `has_seen(&T)`（`VisitKey` 地址身份），全程
//! 安全代码。

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  functions::optional_node::opt_node,
  records::{
    allocator::Allocator,
    ast_array::{AstArray, AstArrayBuilder},
    ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack,
    ast_name::AstName,
    ast_table_indexer::AstTableIndexer,
    ast_table_prop::AstTableProp,
    ast_type::AstType,
    ast_type_function::AstTypeFunction,
    ast_type_intersection::AstTypeIntersection,
    ast_type_list::AstTypeList,
    ast_type_or_pack::AstTypeOrPack,
    ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit,
    ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString,
    ast_type_table::AstTypeTable,
    ast_type_union::AstTypeUnion,
    location::Location,
    node_handle::{Node, OptNode},
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

    self.alloc_named_reference(name)
  }

  /// 分配「默认 location、无 parameters、非展开形态」的命名 `AstTypeReference`
  /// 引用节点——`*blocked*`/`free`/`<Lazy?>` 等十余个原子 `rehydrate_*` 分支
  /// 共用的骨架（cpp 侧各分支手抄的同款 ctor 实参列表）。
  ///
  /// arena 存活前提：`self.allocator` 是构造期由 TypeAttacher 存入的
  /// SourceModule AST bump arena，整个 attach 期间存活且仅本 visitor 独占
  /// （契约收口在 [`Self::allocator_mut`]）。
  fn alloc_named_reference(&mut self, name: AstName) -> *mut AstType {
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
    self.alloc_named_reference(AstName::from_static(b"*blocked*"))
  }

  pub fn rehydrate_pending_expansion(&mut self, _petv: &PendingExpansionType) -> *mut AstType {
    self.alloc_named_reference(AstName::from_static(b"*pending-expansion*"))
  }

  pub fn rehydrate_singleton(&mut self, stv: &SingletonType) -> *mut AstType {
    if let Some(bs) = get_singleton_type::<BooleanSingleton>(stv) {
      let location = Location::default();
      let allocator = self.allocator_mut();
      return allocator
        .alloc(AstTypeSingletonBool::new(location, bs.value))
        .cast::<AstType>();
    }
    if let Some(ss) = get_singleton_type::<StringSingleton>(stv) {
      let location = Location::default();
      // 借用视图构造（`AstArray::from_slice`）：字节域借用自 `ss.value`（类型
      // arena 里的 String，节点存活期不移动），与 C++ `ss->value.c_str()` 同
      // 寿命语义；空串按单源约定落 `AstArray::EMPTY`，读取端 `c_slice` 对空
      // 区间折出同一空切片。
      let value = AstArray::from_slice(ss.value.as_bytes());
      let allocator = self.allocator_mut();
      return allocator
        .alloc(AstTypeSingletonString::new(location, value))
        .cast::<AstType>();
    }
    // 未知单例变体：cpp `TypeAttach.cpp:142-143` 同款 `else return nullptr`——
    // rehydration 对该分支的契约就是「无可重建节点」；空槽经 `opt_node(None)`
    // 单点写出，消费方（attach_type_data 与各 visit 分支）把结果直存进
    // ulua-ast 结点的可空字段/AstArray 槽位，读取侧经 node_opt 折回 Option。
    opt_node(None)
  }

  #[inline]
  pub fn rehydrate_any(&mut self, _any: &AnyType) -> *mut AstType {
    self.alloc_named_reference(AstName::new())
  }

  #[inline]
  pub fn rehydrate_no_refine(&mut self, _no_refine: &NoRefineType) -> *mut AstType {
    self.alloc_named_reference(AstName::from_static(b"*no-refine*"))
  }

  pub fn rehydrate_table(&mut self, ttv: &TableType) -> *mut AstType {
    // 计数 guard 借用本对象 count 字段，嵌套 guard 以 LIFO 进出（等价 C++
    // `RecursionCounter counter(&count)`）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    if let Some(ref name) = ttv.name
      && !self.options.banned_names.contains_str(name.as_str())
    {
      // cpp `TypeAttach.cpp:158-178`：先取 `instantiatedTypeParams + PackParams`
      // 总容量，再按序逐槽写入。槽位申请/写入收口在 `AstArrayBuilder`
      // （容量界与初始化契约见其 push），此处不再手写裸块记账。
      let params_size =
        ttv.instantiated_type_params.len() + ttv.instantiated_type_pack_params.len();
      let mut parameters = AstArrayBuilder::new(self.allocator_mut(), params_size);

      for &ty_param in &ttv.instantiated_type_params {
        let rehydrated = self.visit_type(ty_param);
        // `from_type` 即 cpp `parameters.data[i] = {rehydrated, {}}` 的定形构造。
        parameters.push(AstTypeOrPack::from_type(rehydrated));
      }

      for &tp_param in &ttv.instantiated_type_pack_params {
        let rehydrated = self.rehydrate(tp_param);
        // `from_type_pack` 即 cpp `parameters.data[i] = {{}, rehydrated}` 的定形构造。
        parameters.push(AstTypeOrPack::from_type_pack(rehydrated));
      }

      let allocator = self.allocator_mut();
      let name_ast = alloc_name(allocator, name);

      let parameters = parameters.finish();

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

    // 环检测：以 ttv 的地址身份（VisitKey）判已访问。
    if self.has_seen(ttv) {
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

    // cpp `TypeAttach.cpp:192-237`：容量取 `ttv.props.size()`，每属性恰占一槽
    // （isShared 走 ReadWrite 单槽；非共享按 Luau 不变式 read/write 互斥、至多
    // 一槽），定形 size = 实际写入数。槽位收口在 `AstArrayBuilder`。
    let mut props_builder = AstArrayBuilder::new(self.allocator_mut(), ttv.props.len());

    for (prop_name, prop) in &ttv.props {
      // 嵌套计数 guard 同借 count 字段，先于外层 _counter 释放（LIFO）。
      let _counter_inner = RecursionCounter::recursion_counter_i32(&mut self.count);

      let name_ast = {
        let allocator = self.allocator_mut();
        alloc_name(allocator, prop_name)
      };

      if prop.is_shared() {
        // is_shared() 蕴含 read_ty 为 Some（Luau Property 不变式）。
        let read_ty_rehydrated = self.visit_type(
          prop
            .read_ty
            .expect("is_shared() 蕴含 read_ty 为 Some（Luau Property 不变式）"),
        );
        props_builder.push(AstTableProp {
          name: name_ast,
          location: Location::default(),
          r#type: read_ty_rehydrated,
          access: AstTableAccess::ReadWrite,
          access_location: None,
        });
      } else {
        if let Some(read_ty) = prop.read_ty {
          let read_ty_rehydrated = self.visit_type(read_ty);
          props_builder.push(AstTableProp {
            name: name_ast,
            location: Location::default(),
            r#type: read_ty_rehydrated,
            access: AstTableAccess::Read,
            access_location: None,
          });
        }

        if let Some(write_ty) = prop.write_ty {
          let write_ty_rehydrated = self.visit_type(write_ty);
          props_builder.push(AstTableProp {
            name: name_ast,
            location: Location::default(),
            r#type: write_ty_rehydrated,
            access: AstTableAccess::Write,
            access_location: None,
          });
        }
      }
    }

    let indexer = if let Some(ref indexer_ref) = ttv.indexer {
      // 嵌套 RAII 计数 guard，随块尾释放。
      let _counter_indexer = RecursionCounter::recursion_counter_i32(&mut self.count);

      let index_type = self.visit_type(indexer_ref.index_type);
      let result_type = self.visit_type(indexer_ref.index_result_type);

      let indexer_node = AstTableIndexer {
        // visit_type 的返回值即 arena 分配产物（恒非空），from_raw 收非空句柄。
        index_type: Node::from_raw(index_type),
        result_type: Node::from_raw(result_type),
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      };

      let allocator = self.allocator_mut();
      OptNode::from_ptr(allocator.alloc(indexer_node))
    } else {
      // cpp `TypeAttach.cpp:229` 同款 `AstTableIndexer* indexer = nullptr;`：
      // 仅当 `ttv.indexer` 存在才建节点；落点 `AstTypeTable.indexer` 是可空
      // 句柄槽，空槽即 `None`。
      OptNode::default()
    };

    let props_array = props_builder.finish();

    let table_node = AstTypeTable::new(Location::default(), props_array, indexer);

    let allocator = self.allocator_mut();
    allocator.alloc(table_node).cast::<AstType>()
  }

  /// C++ `AstType* operator()(const MetatableType& mtv)` —
  /// `return Luau::visit(*this, mtv.table->ty);`.
  pub fn rehydrate_metatable(&mut self, mtv: &MetatableType) -> *mut AstType {
    // mtv.table() 返回 MetatableType 内嵌的 TypeId（记录注释：恒指向存活的
    // TableType 节点）。
    self.visit_type(mtv.table())
  }

  pub fn rehydrate_extern(&mut self, etv: &ExternType) -> *mut AstType {
    // 计数 guard 借 count 字段，嵌套增减 LIFO（C++ RecursionCounter(&count)）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    let name = {
      let allocator = self.allocator_mut();
      alloc_name(allocator, &etv.name)
    };

    if !self.options.expand_extern_type_props || self.has_seen(etv) || self.count > 1 {
      return self.alloc_named_reference(name);
    }

    // cpp `TypeAttach.cpp:244-294`：容量取 `etv.props.size()`，槽位记账与
    // 定形 size = 实际写入数（原 `size: idx`）。收口在 `AstArrayBuilder`。
    let mut props_builder = AstArrayBuilder::new(self.allocator_mut(), etv.props.len());

    for (prop_name, prop) in &etv.props {
      let name = {
        let allocator = self.allocator_mut();
        alloc_nul_string(allocator, prop_name)
      };

      if prop.is_shared() {
        // is_shared() 蕴含 read_ty Some（Luau Property 不变式）。
        let read_type_ptr = self.visit_type(
          prop
            .read_ty
            .expect("is_shared() 蕴含 read_ty 为 Some（Luau Property 不变式）"),
        );
        props_builder.push(AstTableProp {
          name: AstName::ast_name_u8(name),
          location: Location::default(),
          r#type: read_type_ptr,
          access: AstTableAccess::ReadWrite,
          access_location: None,
        });
      } else {
        if let Some(read_ty) = prop.read_ty {
          let read_type_ptr = self.visit_type(read_ty);
          props_builder.push(AstTableProp {
            name: AstName::ast_name_u8(name),
            location: Location::default(),
            r#type: read_type_ptr,
            access: AstTableAccess::Read,
            access_location: None,
          });
        }

        if let Some(write_ty) = prop.write_ty {
          let write_type_ptr = self.visit_type(write_ty);
          props_builder.push(AstTableProp {
            name: AstName::ast_name_u8(name),
            location: Location::default(),
            r#type: write_type_ptr,
            access: AstTableAccess::Write,
            access_location: None,
          });
        }
      }
    }

    let indexer = if let Some(ref indexer_data) = etv.indexer {
      // 嵌套计数 guard，随 if 块尾先于外层释放。
      let _inner_counter = RecursionCounter::recursion_counter_i32(&mut self.count);

      let index_type = self.visit_type(indexer_data.index_type);
      let result_type = self.visit_type(indexer_data.index_result_type);

      let allocator = self.allocator_mut();
      OptNode::from_ptr(allocator.alloc(AstTableIndexer {
        // visit_type 的返回值即 arena 分配产物（恒非空），from_raw 收非空句柄。
        index_type: Node::from_raw(index_type),
        result_type: Node::from_raw(result_type),
        location: Location::default(),
        access: AstTableAccess::ReadWrite,
        access_location: None,
      }))
    } else {
      // cpp `TypeAttach.cpp:294` 同款 `AstTableIndexer* indexer = nullptr;`
      //（ExternType 分支）：句柄槽空态即 `None`。
      OptNode::default()
    };

    let props = props_builder.finish();

    let table = AstTypeTable::new(Location::default(), props, indexer);
    let allocator = self.allocator_mut();
    allocator.alloc(table).cast::<AstType>()
  }

  /// 将扁平化后的类型向量逐个 rehydrate，写入 AST 分配器的原始数组。
  ///
  /// 类型前提（visit_type 自身契约）：`tys` 中每个 `TypeId` 指向类型 arena 中
  /// 存活的 `Type` 节点。
  fn rehydrate_types(&mut self, tys: &[TypeId]) -> AstArray<*mut AstType> {
    // 槽位申请与逐槽写入收口在 `AstArrayBuilder`（容量 tys.len()、每元素恰一
    // 槽，对应 C++ `argTypes.data[i] = Luau::visit(...)`）；计数器 guard 在
    // 循环体内创建、迭代尾释放，与 C++ 逐元素 RecursionCounter 同进出时序。
    let mut slots = AstArrayBuilder::new(self.allocator_mut(), tys.len());
    for &ty in tys {
      let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);
      slots.push(self.visit_type(ty));
    }
    slots.finish()
  }

  /// 尾随类型包 rehydrate；无尾随则空槽。
  ///
  /// cpp `TypeAttach.cpp:345/374` 同款 `AstTypePack* argTailAnnotation =
  /// nullptr; if (argTail) ...`——返回值直存 ulua-ast `AstTypeList.tail_type`
  /// 可空槽位（Ast.h:133 明示合法态），空槽经 `opt_node(None)` 单点写出，
  /// 读取方经 `AstTypeList::tail()` / node_opt 门面折回 Option。
  fn rehydrate_tail(&mut self, tail: Option<TypePackId>) -> *mut AstTypePack {
    tail.map_or_else(|| opt_node(None), |tp| self.rehydrate(tp))
  }

  pub fn rehydrate_function(&mut self, ftv: &FunctionType) -> *mut AstType {
    // 计数 guard 贯穿整个函数体，嵌套增减 LIFO 与 C++ 一致。
    let _recursion_counter = RecursionCounter::recursion_counter_i32(&mut self.count);

    // 环检测：以 ftv 的地址身份（VisitKey）判已访问。
    if self.has_seen(ftv) {
      return self.alloc_named_reference(AstName::from_static(b"<Cycle>"));
    }

    // generics
    // 对应 cpp `TypeAttach.cpp:314-321`：先取容量、命中才逐项填充；空泛型集
    // 即 cpp `{nullptr,0}` 的定形构造 `AstArray::EMPTY`。容量界与槽位写入由
    // `AstArrayBuilder` 兑现，定形 size = 命中数（= 旧 num_generics 记账）。
    let generics_array = if ftv.generics.is_empty() {
      AstArray::EMPTY
    } else {
      let mut generics = AstArrayBuilder::new(self.allocator_mut(), ftv.generics.len());
      for &gen_id in &ftv.generics {
        if let Some(r#gen) = get_type::<GenericType>(gen_id) {
          // `r#gen.name` is a Rust String (not NUL-terminated); copy it into
          // the AST allocator with a trailing NUL so AstName's borrowed
          // C-string pointer is safe to read (was UB: read past the bytes).
          let name_ptr = alloc_nul_string(self.allocator_mut(), &r#gen.name);
          let ast_gen =
            AstGenericType::new(Location::default(), AstName::ast_name_u8(name_ptr), None);
          let allocator = self.allocator_mut();
          generics.push(allocator.alloc(ast_gen));
        }
      }
      generics.finish()
    };

    // generic packs
    // 同 generics（cpp `TypeAttach.cpp:324-331`，空集即 `AstArray::EMPTY`）。
    let generic_packs_array = if ftv.generic_packs.is_empty() {
      AstArray::EMPTY
    } else {
      let mut packs = AstArrayBuilder::new(self.allocator_mut(), ftv.generic_packs.len());
      for &pack_id in &ftv.generic_packs {
        if let Some(pack) = get_type_pack::<GenericTypePack>(pack_id) {
          let name_ptr = alloc_nul_string(self.allocator_mut(), &pack.name);
          let ast_pack =
            AstGenericTypePack::new(Location::default(), AstName::ast_name_u8(name_ptr), None);
          let allocator = self.allocator_mut();
          packs.push(allocator.alloc(ast_pack));
        }
      }
      packs.finish()
    };

    // argument types
    let (arg_vector, arg_tail) = flatten_type_pack_id(ftv.arg_types);
    let arg_types_array = self.rehydrate_types(&arg_vector);
    let arg_tail_annotation = self.rehydrate_tail(arg_tail);

    // argument names
    // 同 generics（cpp `TypeAttach.cpp:349-358`，空集即 `AstArray::EMPTY`）；
    // 每元素恰一槽，`Option<AstArgumentName>` 为 None 的槽同样占位（与 cpp
    // placement-new 空 optional 一致）。
    let arg_names_array = if ftv.arg_names.is_empty() {
      AstArray::EMPTY
    } else {
      let mut names = AstArrayBuilder::new(self.allocator_mut(), ftv.arg_names.len());
      for arg_opt in ftv.arg_names.iter() {
        let slot: Option<AstArgumentName> = if let Some(ref arg) = *arg_opt {
          let name_ptr = alloc_nul_string(self.allocator_mut(), &arg.name);
          let name = AstName::ast_name_u8(name_ptr);
          Some((name, Location::default()))
        } else {
          None
        };
        names.push(slot);
      }
      names.finish()
    };

    // return types
    let (ret_vector, ret_tail) = flatten_type_pack_id(ftv.ret_types);
    let return_types_array = self.rehydrate_types(&ret_vector);

    let ret_tail_annotation = self.rehydrate_tail(ret_tail);

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
            // return_annotation 为上方现场分配的显式空 pack（cpp TypeAttach.cpp:379
            // `returnAnnotation` 同源），恒非空，Node::from_raw 建槽。
            Node::from_raw(return_annotation),
        );

    allocator.alloc(func_type).cast::<AstType>()
  }

  pub fn rehydrate_error(&mut self, _err: &ErrorType) -> *mut AstType {
    self.alloc_named_reference(AstName::from_static(b"Unifiable<Error>"))
  }

  #[inline]
  pub fn rehydrate_generic(&mut self, gtv: &GenericType) -> *mut AstType {
    let allocator: &mut Allocator = self.allocator_mut();
    // synthetic_names 经记录层 chokepoint 物化（构造期由 `&mut ta.synthetic_names`
    // 字段存入，TypeAttacher 比 visitor 活得久），业务侧不再触碰裸字段。
    let synthetic_names: &mut SyntheticNames = self.synthetic_names_mut();
    let name_ptr = get_name_allocator_synthetic_names_generic_type(allocator, synthetic_names, gtv);
    self.alloc_named_reference(AstName::ast_name_u8(name_ptr))
  }

  /// C++ `AstType* operator()(const Unifiable::Bound<TypeId>& bound)` —
  /// `return Luau::visit(*this, bound.boundTo->ty);`.
  pub fn rehydrate_bound(&mut self, bound: &Bound<TypeId>) -> *mut AstType {
    // bound.bound_to 是 Bound 变体内存的目标 TypeId，指向 arena 中存活的
    // 解绑目标节点。
    self.visit_type(bound.bound_to)
  }

  pub fn rehydrate_free(&mut self, _ft: &FreeType) -> *mut AstType {
    self.alloc_named_reference(AstName::from_static(b"free"))
  }

  pub fn rehydrate_union(&mut self, uv: &UnionType) -> *mut AstType {
    // cpp `TypeAttach.cpp:398-407`：`unionTypes.data[i] = Luau::visit(...)`，
    // 每选项恰一槽；槽位收口在 `AstArrayBuilder`。
    let mut union_slots = AstArrayBuilder::new(self.allocator_mut(), uv.options.len());
    for &option_ty in uv.options.iter() {
      let rehydrated = self.visit_type(option_ty);
      union_slots.push(rehydrated);
    }
    let union_types = union_slots.finish();

    let alloc = self.allocator_mut();
    alloc
      .alloc(AstTypeUnion::new(Location::default(), union_types))
      .cast::<AstType>()
  }

  pub fn rehydrate_intersection(&mut self, uv: &IntersectionType) -> *mut AstType {
    // cpp `TypeAttach.cpp:409-418`：`intersectionTypes.data[i] = Luau::visit(...)`
    // 的同款记账，收口在 `AstArrayBuilder`。
    let mut part_slots = AstArrayBuilder::new(self.allocator_mut(), uv.parts.len());
    for &part_ty in uv.parts.iter() {
      let ast_part = self.visit_type(part_ty);
      part_slots.push(ast_part);
    }
    let intersection_types = part_slots.finish();

    let location = Location::default();
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
      // 已通过判空（对应 C++ 原子 load 结果非空），unwrapped 是 LazyType
      // 记录的已展开目标，指向 arena 存活节点。
      return self.visit_type(unwrapped);
    }

    self.alloc_named_reference(AstName::from_static(b"<Lazy?>"))
  }

  pub fn rehydrate_unknown(&mut self, _ttv: &UnknownType) -> *mut AstType {
    // cpp `UnknownType` 分支（TypeAttach.cpp:430-432）走 6 参 ctor，parameters
    // 默认 `{}`（Ast.h:1236）= 空数组定形构造，与骨架的 `AstArray::default()`
    // 同一值。
    self.alloc_named_reference(AstName::from_static(b"unknown"))
  }

  #[inline]
  pub fn rehydrate_never(&mut self, _ttv: &NeverType) -> *mut AstType {
    self.alloc_named_reference(AstName::from_static(b"never"))
  }

  #[inline]
  pub fn rehydrate_negation(&mut self, ntv: &NegationType) -> *mut AstType {
    // C++ `params.data[0] = AstTypeOrPack{Luau::visit(*this, ntv.ty->ty), nullptr};`
    // ntv.ty 是 NegationType 内嵌的被否定类型（arena 存活节点）。
    let ty_rehydrated = self.visit_type(ntv.ty);

    let allocator: &mut Allocator = self.allocator_mut();

    // 单槽参数表：容量 1、恰一 push（C++ `params.size = 1` 同款）；槽位写入
    // 收口在 `AstArrayBuilder::push`。
    let mut params_builder = AstArrayBuilder::new(allocator, 1);
    // `from_type` 即 cpp `AstTypeOrPack{rehydrated, nullptr}` 的定形构造。
    params_builder.push(AstTypeOrPack::from_type(ty_rehydrated));
    let params = params_builder.finish();

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
      // `tfit.function()` 经契约访问器共享借用：所指 TypeFunction 定义
      // 先于实例节点存活、覆盖整个分析会话。
      let func = tfit.function();
      AstName::from_raw_parts(func.name.as_ptr(), func.name.len() as u32)
    });
    self.alloc_named_reference(name)
  }
}
