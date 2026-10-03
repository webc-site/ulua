//! `global_types` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::fflag;

use crate::{
  enums::{polarity::Polarity, solver_mode::SolverMode},
  functions::{
    as_mutable_type::as_mutable_type_id, freeze::freeze,
    make_string_metatable::make_string_metatable, persist_type::persist, shared_mut::shared_mut,
    unfreeze::unfreeze,
  },
  records::{
    arena_handle::{alias, alias_ref},
    builtin_types::BuiltinTypes,
    free_type_pack::FreeTypePack,
    generic_type::GenericType,
    generic_type_definition::GenericTypeDefinition,
    global_types::GlobalTypes,
    metatable_type::MetatableType,
    negation_type::NegationType,
    primitive_type::PrimitiveType,
    scope::Scope,
    scope_registry::register_scope,
    source_module::SourceModule,
    type_arena::TypeArena,
    type_fun::TypeFun,
    type_level::TypeLevel,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_variant::TypeVariant},
};

// GlobalTypes 自指针 `builtin_types` 的唯一解引用 chokepoint。
//
// 手法对齐 `BuiltinTypes::arena_handle`：收 `NonNull` 入参的静态转铸函数
// 承载 `# Safety` 契约（构造期尚未有 `Self` 可借用），实例方法仅转发到它。

impl GlobalTypes {
  /// 由 `builtin_types` 句柄取共享引用。
  ///
  /// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
  /// `builtin_types` 须满足 C++ `NotNull<BuiltinTypes>` 接线契约：非空、对齐，
  /// 指向比本 `GlobalTypes` 长寿的单例（`Frontend` 自持字段或其宿主 Box，
  /// 由 `GlobalTypes::new` 入参 / `Frontend::wire_self_pointers` 落位后布线）。
  /// 返回借用生命周期刻意不绑定 `&self`/入参，与迁移前各调用点
  /// `unsafe { ptr.as_ref() }` 的借用检查行为完全同构；单线程序列化驱动
  ///（lib.rs 不变量 1）下借用期内无并存可变别名（BuiltinTypes 单例字段
  /// 构造后只读，arena 改写一律经 `arena_handle` 契约）。
  pub(crate) fn builtin_types_of<'a>(builtin_types: NonNull<BuiltinTypes>) -> &'a BuiltinTypes {
    alias_ref(builtin_types.as_ptr())
  }

  /// 实例形态：解引用本对象自指针的受控读取（契约见
  /// [`GlobalTypes::builtin_types_of`]；未布线窗口内调用属上游构造序契约违例）。
  pub(crate) fn builtin_types_ref<'a>(&self) -> &'a BuiltinTypes {
    // Safety: `self.builtin_types` 由 `GlobalTypes::new` 入参（`&mut` 引用转铸
    // Some）接线、`Frontend::wire_self_pointers` 落位后重布线为存活单例地址。
    // `None` 仅理论不可达（入参 `&mut` 恒非空），仍显式 panic 拦为契约违例，
    // 满足 `builtin_types_of` 的非空前提后才入 unsafe 解引用。
    Self::builtin_types_of(self.builtins_handle())
  }

  /// 供 `builtin_types_ref` 复用的取址 helper：`None`（未接线，理论不可达——
  /// `GlobalTypes::new` 入参 `&mut` 恒转 `Some`）明确 panic 而非静默解引用。
  pub(crate) fn builtins_handle(&self) -> NonNull<BuiltinTypes> {
    self
      .builtin_types
      .expect("GlobalTypes::builtin_types 尚未接线（构造序契约违例）")
  }
}

impl GlobalTypes {
  pub fn global_scope(&self) -> ScopePtr {
    self.global_scope.clone()
  }
}

// C++ `GlobalTypes::GlobalTypes(NotNull<BuiltinTypes>, SolverMode)`
// (`Analysis/src/GlobalTypes.cpp:11`). Builds the shared global scope and the
// global type-function scope, registers the builtin type bindings, and wires
// up the string metatable.

impl GlobalTypes {
  /// C++ `GlobalTypes(NotNull<BuiltinTypes>, SolverMode)`。内建单例入参以
  /// `&mut BuiltinTypes` 受检引用承载（对应 C++ `NotNull` 的「非空、存活、
  /// 单一」语义），构造期内部即时转铸为 `NonNull` 句柄供 arena chokepoint 使用，
  /// 调用点免构造 `NonNull`。
  pub fn new(builtin_types: &mut BuiltinTypes, mode: SolverMode) -> Self {
    // `&mut` 引用天然非空且对齐，转铸免 unsafe；本函数体内 arena 改写
    // （`arena_handle`）与只读快照（`builtin_types_of`）一律复用此句柄。
    let builtin_types = NonNull::from(builtin_types);
    let mut global_types = TypeArena::default();

    // globalScope = std::make_shared<Scope>(globalTypes.addTypePack(TypePackVar{FreeTypePack{TypeLevel{}}}));
    let mut free_pack = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Default::default(),
    };
    free_pack.free_type_pack_type_level(TypeLevel::default());
    let global_scope_ret = global_types.add_type_pack_t(free_pack);

    let mut free_pack_fn = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Default::default(),
    };
    free_pack_fn.free_type_pack_type_level(TypeLevel::default());
    let global_type_function_scope_ret = global_types.add_type_pack_t(free_pack_fn);

    // Build the scope locally so we can register the builtin bindings before
    // sharing it via Arc (C++ mutates the freshly-constructed shared scope).
    let mut global_scope = Scope::scope_type_pack_id(global_scope_ret);
    let global_type_function_scope = Scope::scope_type_pack_id(global_type_function_scope_ret);

    // Snapshot the builtin TypeIds (raw `*const` copies) so the later
    // mutable borrows of `builtinTypes->arena` don't conflict.
    let (
      any_type,
      nil_type,
      number_type,
      integer_type,
      string_type,
      boolean_type,
      thread_type,
      buffer_type,
      unknown_type,
      never_type,
      object_type,
      class_type,
    ) = {
      // 解引用集中于 `GlobalTypes::builtin_types_of` chokepoint：入参 NonNull
      // 按 C++ `NotNull<BuiltinTypes>` 契约由构造方保证非空且指向比本次
      // GlobalTypes 构建长寿的单例；此处只读常量 TypeId 快照（均为既有 arena
      // 节点指针的拷贝）。
      let builtins = Self::builtin_types_of(builtin_types);
      (
        builtins.any_type,
        builtins.nil_type,
        builtins.number_type,
        builtins.integer_type,
        builtins.string_type,
        builtins.boolean_type,
        builtins.thread_type,
        builtins.buffer_type,
        builtins.unknown_type,
        builtins.never_type,
        builtins.object_type,
        builtins.class_type,
      )
    };

    global_scope.add_builtin_type_binding("any", &TypeFun::type_fun_type_id(any_type));
    global_scope.add_builtin_type_binding("nil", &TypeFun::type_fun_type_id(nil_type));
    global_scope.add_builtin_type_binding("number", &TypeFun::type_fun_type_id(number_type));
    if fflag::LuauIntegerType2.get() {
      global_scope.add_builtin_type_binding("integer", &TypeFun::type_fun_type_id(integer_type));
    }
    global_scope.add_builtin_type_binding("string", &TypeFun::type_fun_type_id(string_type));
    global_scope.add_builtin_type_binding("boolean", &TypeFun::type_fun_type_id(boolean_type));
    global_scope.add_builtin_type_binding("thread", &TypeFun::type_fun_type_id(thread_type));
    global_scope.add_builtin_type_binding("buffer", &TypeFun::type_fun_type_id(buffer_type));
    global_scope.add_builtin_type_binding("unknown", &TypeFun::type_fun_type_id(unknown_type));
    global_scope.add_builtin_type_binding("never", &TypeFun::type_fun_type_id(never_type));
    if fflag::DebugLuauUserDefinedClasses.get() {
      global_scope.add_builtin_type_binding("object", &TypeFun::type_fun_type_id(object_type));
      global_scope.add_builtin_type_binding("class", &TypeFun::type_fun_type_id(class_type));
    }

    let global_scope: Arc<Scope> = Arc::new(global_scope);
    register_scope(&global_scope);
    let global_type_function_scope: Arc<Scope> = Arc::new(global_type_function_scope);
    register_scope(&global_type_function_scope);

    // unfreeze(*builtinTypes->arena);
    // Safety: builtin_types 为 NotNull 契约接线的存活单例，其 arena 字段是 Box 独占
    // 堆分配的 TypeArena（唯一对象、地址稳定）；构造序列单线程执行，unfreeze 期间
    // 无其他借用指向该 arena，满足 `BuiltinTypes::arena_handle` 的 `# Safety` 契约。
    unfreeze(unsafe { BuiltinTypes::arena_handle(builtin_types) }.get_mut());

    let string_metatable_ty = make_string_metatable(builtin_types, mode);

    // asMutable(builtinTypes->string_type)->ty.emplace<PrimitiveType>(PrimitiveType::STRING, stringMetatableTy);
    // string_type 取自上方 builtins 快照，是 builtin arena（bump 块，地址不移动）
    // 中存活 Type 节点的拷贝指针；上一步已 unfreeze、下一步即 freeze，此窗口内
    // 构造序列是唯一写者，覆写 ty 与 C++ emplace 同语义（附 new 的 string 元表，
    // 节点自此持久化）。
    alias(as_mutable_type_id(string_type)).ty = TypeVariant::Primitive(PrimitiveType {
      r#type: PrimitiveType::STRING,
      metatable: Some(string_metatable_ty),
    });

    persist(string_metatable_ty);

    // freeze(*builtinTypes->arena);
    // Safety: 与上方 unfreeze 分支同一论证——Box 独占的 arena 对象、构造序列
    // 单线程且此刻无并发借用，句柄物化不引入别名。
    freeze(unsafe { BuiltinTypes::arena_handle(builtin_types) }.get_mut());

    Self {
      builtin_types: Some(builtin_types),
      global_types,
      global_names: SourceModule::new(),
      global_scope,
      global_type_function_scope,
      mode,
      retained_modules: Vec::new(),
    }
  }
}

impl GlobalTypes {
  pub fn global_types_mut(&mut self) -> &mut TypeArena {
    &mut self.global_types
  }
}

impl GlobalTypes {
  pub fn register_hidden_test_types(&mut self) {
    unfreeze(&mut self.global_types);

    let t = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        "T",
        Polarity::Mixed,
      ));
    let generic_t = GenericTypeDefinition {
      ty: t,
      default_value: None,
    };

    let u = self
      .global_types
      .add_type(GenericType::generic_type_name_polarity(
        "U",
        Polarity::Mixed,
      ));
    let generic_u = GenericTypeDefinition {
      ty: u,
      default_value: None,
    };

    let not_type = self.global_types.add_type(NegationType::new(t));
    let mt_type = self.global_types.add_type(MetatableType {
      table: t,
      metatable: u,
      synthetic_name: None,
    });

    // 解引用集中于 `builtin_types_ref` chokepoint：`self.builtin_types` 是
    // `GlobalTypes::new` 以 NonNull 契约接线（并由 `Frontend::wire_self_pointers`
    // 落位重布线）的会话级 `BuiltinTypes` 表，只读四个 Copy TypeId 字段。
    let builtins = self.builtin_types_ref();
    let (function_type, extern_type, error_type, table_type) = (
      builtins.function_type,
      builtins.extern_type,
      builtins.error_type,
      builtins.table_type,
    );

    let scope = shared_mut(&self.global_scope);
    // Safety: `scope` 经 `shared_mut` 取自 `self.global_scope`（本方法 `&mut self`
    // 独占的 Arc<Scope>，函数体内全程存活），裸指针仅作写穿句柄；单线程序列化下
    // 此刻无人持有对同一 Scope 的并存借用，多条 insert 均独占改写
    // `exported_type_bindings`，与 C++ `globalTypes` 注册路径同形。
    {
      scope.exported_type_bindings.insert(
        String::from("Not"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t],
          not_type,
          None,
        ),
      );
      scope.exported_type_bindings.insert(
        String::from("Mt"),
        TypeFun::type_fun_vector_generic_type_definition_type_id_optional_location(
          vec![generic_t, generic_u],
          mt_type,
          None,
        ),
      );
      scope.exported_type_bindings.insert(
        String::from("fun"),
        TypeFun::type_fun_type_id(function_type),
      );
      scope
        .exported_type_bindings
        .insert(String::from("cls"), TypeFun::type_fun_type_id(extern_type));
      scope
        .exported_type_bindings
        .insert(String::from("err"), TypeFun::type_fun_type_id(error_type));
      scope
        .exported_type_bindings
        .insert(String::from("tbl"), TypeFun::type_fun_type_id(table_type));
    }

    freeze(&mut self.global_types);
  }
}
// ==== tstr21 尾台账（String::from 收口波，票94候选）====
// 让 6 枚（:64/:72/:80/:85/:88/:91 `String::from("Not"/"Mt"/"fun"/"cls"/"err"/"tbl")`）：
//   消费口为 std 裸 `HashMap<Name, TypeFun>` 直插（records/scope.rs:34，
//   type_aliases/name_type.rs:2 `pub type Name = String`），键按值接收，剥为 &str
//   即 E0308——同面先例实证于 r7-tstr11（unit-test type_infer_core.rs 尾台账）。
//   本文件非自有 record insert 口（对比 SourceTable::insert 双 Into 先例），加
//   Scope 包装法走 Into<String> 仍 1 malloc=纯剥壳零真省（tprops1 NO-GO 裁定）；
//   且 exported_type_bindings 为 pub 字段、全域直插 >5 调用面，签名/键型外溢
//   须待 Name→HipStr 换型大票。另本路径为一次性测试类型注册（cpp 对拍
//   tests/Fixture.cpp:977-982 六键语义逐枚属实），rc-5 运行期口径下 alloc 已
//   记非热路径。解锁条件：Name 换型/键位开 Into 专化口（生产票）后随行收。

impl GlobalTypes {
  pub fn set_global_scope(&mut self, global_scope: ScopePtr) {
    self.global_scope = global_scope;
  }
}
