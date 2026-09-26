use alloc::{string::ToString, vec::Vec};
use core::mem::take;

use ulua_common::{fflag, fint, functions::escape::escape, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::table_state::TableState,
  functions::{
    begin_type::begin_union_type,
    begin_type_pack::begin,
    can_use_type_name_in_scope::can_use_type_name_in_scope,
    end_type_pack::end,
    follow_type::follow,
    get_type::get,
    is_empty::is_empty,
    is_overloaded_function::is_overloaded_function,
    is_prim::{is_nil, is_number},
  },
  records::{
    any_type::AnyType,
    blocked_type::BlockedType,
    element_result::ElementResult,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    intersection_type::IntersectionType,
    lazy_type::LazyType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType,
    primitive_type::{PrimitiveType, Type},
    singleton_type::SingletonType,
    table_type::TableType,
    to_string_span::ToStringSpan,
    type_function_instance_type::TypeFunctionInstanceType,
    type_stringifier::TypeStringifier,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{
    bound_type::BoundType, error_type::ErrorType, singleton_variant::SingletonVariant,
    type_id::TypeId,
  },
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId ty, const FreeType& ftv)`.
  pub fn stringify_free(&mut self, ty: TypeId, ftv: &FreeType) {
    // Safety: `self.state` 由入口 `to_string*` 函数以自身局部
    // `StringifierState` 的 `&mut` 转裸指针在构造期注入，`result` 同源于入口
    // 局部 `ToStringResult`；本方法调用期间二者是这两个位置的唯一访问路径。
    // `ftv` 借用自正在匹配的 arena 节点，其 lower/upper bound 经上方判空断言
    // 与 `follow` 后仍指向 arena 驻留 Type 节点，满足 `stringify_type_id` 契约。

    self.st().result_mut().invalid = true;

    // Free types are guaranteed to have upper and lower bounds now.
    LUAU_ASSERT!(!ftv.lower_bound.is_null());
    LUAU_ASSERT!(!ftv.upper_bound.is_null());
    let lower_bound = follow(ftv.lower_bound);
    let upper_bound = follow(ftv.upper_bound);
    if get::<NeverType>(lower_bound).is_some() && get::<UnknownType>(upper_bound).is_some() {
      self.st().emit("'");
      let name = self.st().get_name_type_id(ty);
      self.st().emit(name.as_str());
      if fint::DebugLuauVerboseTypeNames.get() >= 1 {
        self.st().emit_polarity(ftv.polarity);
      }
    } else {
      self.st().emit("(");
      if get::<NeverType>(lower_bound).is_none() {
        self.stringify_type_id(lower_bound);
        self.st().emit(" <: ");
      }
      self.st().emit("'");
      let name = self.st().get_name_type_id(ty);
      self.st().emit(name.as_str());

      if fint::DebugLuauVerboseTypeNames.get() >= 1 {
        self.st().emit_polarity(ftv.polarity);
      }

      if get::<UnknownType>(upper_bound).is_none() {
        self.st().emit(" <: ");
        self.stringify_type_id(upper_bound);
      }
      self.st().emit(")");
    }
  }

  pub fn stringify_bound(&mut self, _ty: TypeId, btv: &BoundType) {
    // btv.bound_to 为另一 arena Type 节点句柄；被调已降 safe，state 前提在其窄块证成。
    self.stringify_type_id(btv.bound_to);
  }

  /// C++ `void operator()(TypeId ty, const GenericType& gtv)`.
  pub fn stringify_generic(&mut self, ty: TypeId, gtv: &GenericType) {
    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit("gen-");
    }

    if gtv.explicit_name {
      self.st().used_names.insert(gtv.name.clone());
      *self.st().opts_mut().name_map.types.get_or_insert(ty) = gtv.name.clone();
      self.st().emit(gtv.name.as_str());
    } else {
      let name = self.st().get_name_type_id(ty);
      self.st().emit(name.as_str());
    }

    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit_polarity(gtv.polarity);
    }

    if fint::DebugLuauVerboseTypeNames.get() >= 2 {
      self.st().emit("-");
      // # Safety: `gtv.scope` 为 arena 节点接线、存活跨整个 stringify 会话的
      // `Scope` 句柄（可为空，emit_level 内 `as_ref` 收为 Option 后只读上溯）。
      unsafe { self.st().emit_level(gtv.scope) };
    }
  }

  /// C++ `void operator()(TypeId, const BlockedType& btv)`.
  pub fn stringify_blocked(&mut self, _ty: TypeId, btv: &BlockedType) {
    // Safety: 仅向 `self.state` emit——它指向入口函数活借用后裸化的
    // `StringifierState`，在本 stringifier 存活期间独占且非空；`btv.index`
    // 只是借用参数的整数读取。

    self.st().emit("*blocked-");
    self.st().emit(&btv.index);
    self.st().emit("*");
  }

  /// C++ `void operator()(TypeId ty, const PendingExpansionType& petv)`.
  pub fn stringify_pending_expansion(&mut self, _ty: TypeId, petv: &PendingExpansionType) {
    // Safety: 解引用只发生在 `self.state`（构造期由入口 `to_string*` 的局部
    // `StringifierState` 活借用注入，方法执行期间一直有效）；`petv.index`
    // 不引入新的指针生命周期。

    self.st().emit("*pending-expansion-");
    self.st().emit(&petv.index);
    self.st().emit("*");
  }

  /// C++ `void operator()(TypeId, const PrimitiveType& ptv)`.
  pub fn stringify_primitive(&mut self, _ty: TypeId, ptv: &PrimitiveType) {
    // state 借由 `TypeStringifier::st` 单点从裸句柄恢复为普通引用。
    let state = self.st();
    match ptv.r#type {
      Type::NilType => state.emit("nil"),
      Type::Boolean => state.emit("boolean"),
      Type::Number => state.emit("number"),
      Type::String => state.emit("string"),
      Type::Thread => state.emit("thread"),
      Type::Buffer => state.emit("buffer"),
      Type::Function => state.emit("function"),
      Type::Table => state.emit("table"),
      Type::Integer => {
        if fflag::LuauIntegerType2.get() {
          state.emit("integer");
        } else {
          // C++ [[fallthrough]] to: throw InternalCompilerError("Unknown primitive type")
          panic!("Unknown primitive type {:?}", ptv.r#type);
        }
      }
    }
  }

  pub fn stringify_singleton(&mut self, _ty: TypeId, stv: &SingletonType) {
    // Safety: `stv.variant` 借用自被匹配的 arena Singleton 节点（只读），
    // 块内仅向构造期注入的独占 `StringifierState` 写文本，`escape` 为安全
    // 纯函数。

    match stv.variant {
      SingletonVariant::V0(ref bs) => {
        if bs.value {
          self.st().emit_string("true");
        } else {
          self.st().emit_string("false");
        }
      }
      SingletonVariant::V1(ref ss) => {
        self.st().emit_string("\"");
        let escaped = escape(&ss.value, false);
        self.st().emit_string(&escaped);
        self.st().emit_string("\"");
      }
    }
  }

  /// C++ `void operator()(TypeId, const FunctionType& ftv)`.
  pub fn stringify_function(&mut self, _ty: TypeId, ftv: &FunctionType) {
    // state/opts/result 经 `st`/`opts_mut`/`result_mut` 单点收口；seen 集只把
    // 正在匹配 arena 节点的地址作身份键、从不解引用；`ftv.generics`/`arg_types`/
    // `ret_types` 借用自该 arena 节点，作为 `stringify_type_id` /
    // `stringify_type_pack_id_vector_optional_function_argument`（其 tpid/names
    // 契约）的实参成立。

    if self.st().has_seen(ftv) {
      self.st().result_mut().cycle = true;
      self.st().emit("*CYCLE*");
      return;
    }

    // We should not be respecting opts.hideNamedFunctionTypeParameters here.
    if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
      self.st().emit("<");
      let mut comma = false;
      for &g in ftv.generics.iter() {
        if comma {
          self.st().emit(", ");
        }
        comma = true;
        self.stringify_type_id(g);
      }
      for &gp in ftv.generic_packs.iter() {
        if comma {
          self.st().emit(", ");
        }
        comma = true;
        self.stringify_type_pack_id(gp);
      }
      self.st().emit(">");
    }

    if ftv.is_checked_function {
      self.st().emit("@checked ");
    }

    self.st().emit("(");

    if is_empty(ftv.arg_types) {
      // if we've got an empty argument pack, we're done.
    } else if self.st().opts_mut().function_type_arguments {
      self.stringify_type_pack_id_vector_optional_function_argument(ftv.arg_types, &ftv.arg_names);
    } else {
      self.stringify_type_pack_id(ftv.arg_types);
    }

    self.st().emit(") -> ");

    let mut plural = !is_empty(ftv.ret_types);

    let mut ret_begin = begin(ftv.ret_types);
    let ret_end = end(ftv.ret_types);
    if ret_begin != ret_end {
      ret_begin.advance();
      if ret_begin == ret_end && ret_begin.tail().is_none() {
        plural = false;
      }
    }

    if plural {
      self.st().emit("(");
    }

    self.stringify_type_pack_id(ftv.ret_types);

    if plural {
      self.st().emit(")");
    }

    self.st().unsee(ftv);
  }

  /// C++ `void operator()(TypeId ty, const TableType& ttv)`.
  pub fn stringify_table(&mut self, ty: TypeId, ttv: &TableType) {
    // Safety: 对 `state`/`opts`/`result` 的解引用依赖构造期注入不变量；
    // `opts.scope` 读出的是 clone 出的独立 `Arc<Scope>`，不借用 opts。
    // `ttv` 及其 `name`/`props`/`indexer`/indexer 内 TypeId 借用自正在匹配的
    // Table arena 节点，覆盖整个递归输出期；`has_seen`/`unsee` 只将该节点
    // 地址作身份键使用；`old_length` 起对 `result.name` 的读取均为语句级
    // 临时借用。

    if let Some(bound_to) = ttv.bound_to {
      return self.stringify_type_id(bound_to);
    }

    // if hide table alias expansions are enabled and there is a name found for the table, use it
    let show_name = !self.st().exhaustive || self.st().opts_mut().hide_table_alias_expansions;

    if show_name && let Some(name) = &ttv.name {
      // If scope if provided, add module name and check visibility
      if let Some(scope) = self.st().opts_mut().scope.clone() {
        let (success, module_name) = can_use_type_name_in_scope(scope, name);

        if !success {
          self.st().result_mut().invalid = true;
        }

        if let Some(module_name) = module_name {
          self.st().emit(module_name.as_str());
          self.st().emit(".");
        }
      }

      self.st().emit_and_record_span(name, ty);
      self.stringify_vector_type_id_vector_type_pack_id(
        &ttv.instantiated_type_params,
        &ttv.instantiated_type_pack_params,
      );
      return;
    }

    if !self.st().exhaustive
      && !self.st().ignore_synthetic_name
      && let Some(synthetic_name) = &ttv.synthetic_name
    {
      self.st().result_mut().invalid = true;
      self.st().emit_and_record_span(synthetic_name, ty);
      self.stringify_vector_type_id_vector_type_pack_id(
        &ttv.instantiated_type_params,
        &ttv.instantiated_type_pack_params,
      );
      return;
    }

    if self.st().has_seen(ttv) {
      self.st().result_mut().cycle = true;
      self.st().emit("*CYCLE*");
      return;
    }

    let effective_state = if self.st().opts_mut().hide_table_kind {
      TableState::Sealed
    } else {
      ttv.state
    };
    let (openbrace, closedbrace) = match effective_state {
      TableState::Sealed => ("{", "}"),
      TableState::Unsealed => {
        self.st().result_mut().invalid = true;
        ("{|", "|}")
      }
      TableState::Free => {
        self.st().result_mut().invalid = true;
        ("{-", "-}")
      }
      TableState::Generic => {
        self.st().result_mut().invalid = true;
        ("{+", "+}")
      }
    };

    // If this appears to be an array, we want to stringify it using the {T} syntax.
    if let Some(indexer) = &ttv.indexer
      && ttv.props.is_empty()
      && is_number(indexer.index_type)
    {
      self.st().emit("{");
      if indexer.is_read_only {
        self.st().emit("read ");
      }
      self.stringify_type_id(indexer.index_result_type);
      self.st().emit("}");

      self.st().unsee(ttv);
      return;
    }

    self.st().emit(openbrace);
    self.st().indent();

    let mut comma = false;
    if let Some(indexer) = &ttv.indexer {
      self.st().newline();
      if indexer.is_read_only {
        self.st().emit("read ");
      }
      self.st().emit("[");
      self.stringify_type_id(indexer.index_type);
      self.st().emit("]: ");
      self.stringify_type_id(indexer.index_result_type);
      comma = true;
    }

    let old_length = self.st().result_mut().name.len();
    for (index, (name, prop)) in ttv.props.iter().enumerate() {
      if comma {
        self.st().emit(",");
        self.st().newline();
      } else {
        self.st().newline();
      }

      let length = self.st().result_mut().name.len() - old_length;

      let max_table_length = self.st().opts_mut().max_table_length;
      if max_table_length > 0 && (length - 2 * index) >= max_table_length {
        self.st().emit("... ");
        self
          .st()
          .emit((ttv.props.len() - index).to_string().as_str());
        self.st().emit(" more ...");
        break;
      }

      self.stringify_string_property(name, prop);

      comma = true;
    }

    self.st().dedent();
    if comma {
      self.st().newline();
    } else {
      self.st().emit("  ");
    }
    self.st().emit(closedbrace);

    self.st().unsee(ttv);
  }

  /// C++ `void operator()(TypeId ty, const MetatableType& mtv)`.
  pub fn stringify_metatable(&mut self, ty: TypeId, mtv: &MetatableType) {
    // Safety: `state` 由构造期入口局部借用注入、`result` 写标志位同源有效；
    // `mtv.metatable`/`mtv.table` 是从被匹配 Metatable arena 节点读出的 arena
    // 驻留 TypeId，满足 `stringify_type_id` 契约。

    self.st().result_mut().invalid = true;
    if !self.st().exhaustive
      && let Some(synthetic_name) = &mtv.synthetic_name
    {
      self.st().emit_and_record_span(synthetic_name, ty);
      return;
    }

    if fflag::LuauBetterMetatableStringification.get() {
      self.st().emit("setmetatable<");
      self.stringify_type_id(mtv.table);
      self.st().emit(",");
      self.st().newline();
      self.stringify_type_id(mtv.metatable);
      self.st().emit(">");
    } else {
      self.st().emit("{ @metatable ");
      self.stringify_type_id(mtv.metatable);
      self.st().emit(",");
      self.st().newline();
      self.stringify_type_id(mtv.table);
      self.st().emit(" }");
    }
  }

  pub fn stringify_extern(&mut self, ty: TypeId, etv: &ExternType) {
    // state 借由 `TypeStringifier::st` 单点从裸句柄恢复为普通引用；
    // `etv.name` 借用自被匹配的 Extern arena 节点，仅只读。
    let state = self.st();
    state.emit_and_record_span(&etv.name, ty);
  }

  /// C++ `void operator()(TypeId, const AnyType&)`.
  pub fn stringify_any(&mut self, _ty: TypeId, _atv: &AnyType) {
    self.st().emit("any")
  }

  /// C++ `void operator()(TypeId, const NoRefineType&)`.
  pub fn stringify_no_refine(&mut self, _ty: TypeId, _nrtv: &NoRefineType) {
    // Safety: `state` 指针在整个 stringifier 生命周期内指向存活的入口局部
    // `StringifierState` 且仅此一路径可写，此处仅 emit 固定标记串。
    self.st().emit("*no-refine*")
  }

  /// C++ `void operator()(TypeId, const UnionType& uv)`.
  pub fn stringify_union(&mut self, _ty: TypeId, uv: &UnionType) {
    // `uv` 借用自被匹配 Union 节点，`begin_union_type(uv)` 展平产出的元素经
    // `follow` 后仍是 arena 驻留节点，满足 `stringify_type_id` 递归契约；
    // 对 `result.name` 的 take/回写与 `type_spans` 索引读均为块内顺序
    // 临时借用，`seen` 集只存 `uv` 节点地址作身份键。
    if self.st().has_seen(uv) {
      self.st().result_mut().cycle = true;
      self.st().emit("*CYCLE*");
      return;
    }

    LUAU_ASSERT!(uv.options.len() > 1);

    let mut optional = false;
    let mut has_non_nil_disjunct = false;

    // for (auto el : &uv) — UnionTypeIterator 展平嵌套 union，防环。
    let use_question_marks = self.st().opts_mut().use_question_marks;
    let els = begin_union_type(uv).map(follow).filter(|&el| {
      if use_question_marks && is_nil(el) {
        optional = true;
        false
      } else {
        has_non_nil_disjunct = true;
        true
      }
    });
    let (mut results, length_limit_hit) = self.collect_stringified_elements(els, JoinKind::Union);

    self.st().unsee(uv);

    if !length_limit_hit && !fflag::DebugLuauToStringNoLexicalSort.get() {
      results.sort_unstable_by(|a, b| a.str.cmp(&b.str));
    }

    if optional && results.len() > 1 {
      self.st().emit("(");
    }

    let should_place_on_newlines =
      results.len() > self.st().opts_mut().composite_types_single_line_limit;
    self.emit_stringified_elements(&results, "| ", should_place_on_newlines);

    if optional {
      let mut s = "?";
      if results.len() > 1 {
        s = ")?";
      }

      if !has_non_nil_disjunct {
        s = "nil";
      }

      self.st().emit(s);
    }
  }

  /// C++ `void operator()(TypeId ty, const IntersectionType& uv)`.
  /// NOTE: unlike the union arm, C++ iterates `uv.parts` directly here
  /// (no flattening iterator).
  pub fn stringify_intersection(&mut self, ty: TypeId, uv: &IntersectionType) {
    // Safety: 本块与 union 臂共用同一不变量——`state`/`opts`/`result` 由构造
    // 期活借用注入且顺序借用不重叠。`uv.parts` 逐项借用自被匹配
    // Intersection 节点，`follow` 后仍指向 arena Type 节点，递归调用
    // `stringify_type_id` 满足其契约；`is_overloaded_function(ty)` 只沿 `ty`
    // 自身（入口持有的 arena 节点）做只读 follow。

    if self.st().has_seen(uv) {
      self.st().result_mut().cycle = true;
      self.st().emit("*CYCLE*");
      return;
    }

    let (mut results, length_limit_hit) = self.collect_stringified_elements(
      uv.parts.iter().map(|&part| follow(part)),
      JoinKind::Intersection,
    );

    self.st().unsee(uv);

    if !length_limit_hit && !fflag::DebugLuauToStringNoLexicalSort.get() {
      results.sort_unstable_by(|a, b| a.str.cmp(&b.str));
    }

    let should_place_on_newlines = results.len()
      > self.st().opts_mut().composite_types_single_line_limit
      || is_overloaded_function(ty);
    self.emit_stringified_elements(&results, "& ", should_place_on_newlines);
  }

  pub fn stringify_error(&mut self, _ty: TypeId, tv: &ErrorType) {
    // Safety: `result` 指向入口函数存活的局部 `ToStringResult`（构造期活
    // 借用裸化），仅写 error 标志。

    self.st().result_mut().error = true;

    if let Some(synthetic) = &tv.synthetic {
      // Safety: `self.state` 为构造期注入的独占 `StringifierState` 活引用，
      // emit_string 仅追加文本。

      self.st().emit_string("*error-type<");

      // *synthetic 是 Error 节点携带的 arena 驻留 TypeId；被调已降 safe。
      self.stringify_type_id(*synthetic);
      // Safety: `self.state` 源自构造期由入口 `to_string*` 函数对自身局部
      // `StringifierState` 的活借用裸化，本方法执行期间存活且仅此借用访问该
      // 位置；emit_string 内部只按 `state.opts`（同源注入，仅长度限制判定，
      // 且自身对空 opts 有早退）判限后向 `state.result` 指向的 `name` 缓冲追加文本。

      self.st().emit_string(">*");
    } else {
      // Safety: 单一 emit_string 到构造期活借用注入的 `StringifierState`，
      // 本调用内无其他别名访问该位置。

      self.st().emit_string("*error-type*");
    }
  }

  pub fn stringify_lazy(&mut self, _ty: TypeId, ltv: &LazyType) {
    // Safety: `ltv.unwrapped` 先判非空——非空时它是 unwrapLazy 回写的同
    // arena 驻留节点地址，递归 `stringify_type_id` 契约成立；else 分支只
    // 解引用构造期注入、本调用独占的 `state`/`result`。

    if !ltv.unwrapped.is_null() {
      let unwrapped = ltv.unwrapped;
      self.stringify_type_id(unwrapped);
    } else {
      self.st().result_mut().invalid = true;
      self.st().emit("lazy?");
    }
  }

  /// C++ `void operator()(TypeId, const UnknownType&)`.
  pub fn stringify_unknown(&mut self, _ty: TypeId, _ttv: &UnknownType) {
    // Safety: 仅 emit 常量串到 `self.state`，该指针构造期由入口局部
    // `StringifierState` 的活借用注入，在本方法执行期间独占有效。
    self.st().emit("unknown")
  }

  /// C++ `void operator()(TypeId, const NeverType&)`.
  pub fn stringify_never(&mut self, _ty: TypeId, _ttv: &NeverType) {
    self.st().emit("never")
  }

  /// C++ `void operator()(TypeId, const NegationType& ntv)`.
  pub fn stringify_negation(&mut self, _ty: TypeId, ntv: &NegationType) {
    // Safety: `ntv.ty` 借用自被匹配 Negation arena 节点，`follow` 后仍指向
    // arena 驻留 Type 节点，`stringify_type_id(ntv.ty)` 满足其入参契约；
    // `self.st()` 经 arena_handle::alias 单点借用构造期接线的 `StringifierState`。

    self.st().emit("~");

    // The precedence of `~` should be less than `|` and `&`.
    let followed = follow(ntv.ty);
    let parens =
      get::<UnionType>(followed).is_some() || get::<IntersectionType>(followed).is_some();

    if parens {
      self.st().emit("(");
    }

    self.stringify_type_id(ntv.ty);

    if parens {
      self.st().emit(")");
    }
  }

  /// C++ `void operator()(TypeId, const TypeFunctionInstanceType& tfitv)`.
  pub fn stringify_type_function_instance(
    &mut self,
    _ty: TypeId,
    tfitv: &TypeFunctionInstanceType,
  ) {
    if let Some(user_func_name) = &tfitv.user_func_name {
      // Special stringification for user-defined type functions
      self.st().emit(user_func_name.as_str_or_empty());
    } else {
      // # Safety: `tfitv.function` 是实例节点持有的 `NonNull<TypeFunction>`，
      // 所指内建类型函数表在整个类型检查期内存活，此处仅只读其 `name`。
      let fname = unsafe { &tfitv.function.as_ref().name };
      self.st().emit(fname.as_str());
    }

    self.st().emit("<");

    let mut comma = false;
    for &ty in tfitv.type_arguments.iter() {
      if comma {
        self.st().emit(", ");
      }

      comma = true;
      self.stringify_type_id(ty);
    }

    for &tp in tfitv.pack_arguments.iter() {
      if comma {
        self.st().emit(", ");
      }

      comma = true;
      self.stringify_type_pack_id(tp);
    }

    self.st().emit(">");
  }
}

/// union / intersection 序列化的共通参数：括号包裹规则互为镜像
/// （union 的 Intersection|Function 成员加括号，intersection 的 Union|Function
/// 成员加括号——均对应 cpp ToString.cpp 两处逐字重复 lambda 的差异位）。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum JoinKind {
  Union,
  Intersection,
}

impl TypeStringifier {
  /// cpp union/intersection 两处重复的"逐元素捕获字符串 + spans"循环：
  /// 每个元素先摘走 `result.name`，按需加括号后递归序列化，再回收本次
  /// 产出的字符串与 spans（`drain` 取走后截断还原），累计长度命中
  /// `max_type_length` 即停。返回 `(元素结果, 是否命中长度上限)`。
  fn collect_stringified_elements(
    &mut self,
    els: impl Iterator<Item = TypeId>,
    kind: JoinKind,
  ) -> (Vec<ElementResult>, bool) {
    let mut results: Vec<ElementResult> = Vec::new();
    let mut results_length: usize = 0;
    let mut length_limit_hit = false;

    for el in els {
      let saved = take(&mut self.st().result_mut().name);
      let saved_spans_size = self.st().result_mut().type_spans.len();

      // 括号判定须逐元素现场求值：递归序列化可能中途登记 cycle name，
      // 提前批量计算会用过期状态。
      let need_parens = !self.st().cycle_names.contains(&el)
        && match kind {
          JoinKind::Union => {
            get::<IntersectionType>(el).is_some() || get::<FunctionType>(el).is_some()
          }
          JoinKind::Intersection => {
            get::<UnionType>(el).is_some() || get::<FunctionType>(el).is_some()
          }
        };

      if need_parens {
        self.st().emit("(");
      }

      self.stringify_type_id(el);

      if need_parens {
        self.st().emit(")");
      }

      // 本次新增 spans：drain 取走（原实现先按下标逐个 clone 再 truncate，
      // drain 单次完成同一动作）。
      let elem = ElementResult {
        str: take(&mut self.st().result_mut().name),
        spans: self.st().result_mut().type_spans.drain(saved_spans_size..).collect(),
      };

      results_length += elem.str.len();
      results.push(elem);

      self.st().result_mut().name = saved;

      let max_type_length = self.st().opts_mut().max_type_length;
      length_limit_hit = max_type_length > 0 && results_length > max_type_length;

      if length_limit_hit {
        break;
      }
    }

    (results, length_limit_hit)
  }

  /// cpp union/intersection 两处重复的"拼接已捕获元素"循环：非首元素前按
  /// `newlines` 决定换行或空格，再补 `op`（"| "/"& "）；元素落位时把其
  /// spans 平移 `base_pos` 后登记回 `type_spans`。
  fn emit_stringified_elements(&mut self, results: &[ElementResult], op: &str, newlines: bool) {
    let mut first = true;
    for elem in results.iter() {
      if !first {
        if newlines {
          self.st().newline();
        } else {
          self.st().emit(" ");
        }
        self.st().emit(op);
      }

      let base_pos = self.st().result_mut().name.len();
      self.st().emit(elem.str.as_str());
      for span in elem.spans.iter() {
        self.st().result_mut().type_spans.push(ToStringSpan {
          start_pos: base_pos + span.start_pos,
          end_pos: base_pos + span.end_pos,
          r#type: span.r#type,
        });
      }

      first = false;
    }
  }
}
