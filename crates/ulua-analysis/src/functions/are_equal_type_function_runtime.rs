use ulua_common::{fflag::LuauTypeFunctionRobustness, records::variant::Variant2};

use crate::{
  functions::seen_set_contains::seen_set_contains,
  records::{
    are_equal_state::AreEqualState, recursion_limiter::RecursionLimiter,
    type_function_any_type::TypeFunctionAnyType, type_function_extern_type::TypeFunctionExternType,
    type_function_function_type::TypeFunctionFunctionType,
    type_function_generic_type::TypeFunctionGenericType,
    type_function_generic_type_pack::TypeFunctionGenericTypePack,
    type_function_intersection_type::TypeFunctionIntersectionType,
    type_function_negation_type::TypeFunctionNegationType,
    type_function_never_type::TypeFunctionNeverType,
    type_function_primitive_type::TypeFunctionPrimitiveType,
    type_function_singleton_type::TypeFunctionSingletonType,
    type_function_table_type::TypeFunctionTableType, type_function_type::TypeFunctionType,
    type_function_type_pack::TypeFunctionTypePack,
    type_function_type_pack_var::TypeFunctionTypePackVar,
    type_function_union_type::TypeFunctionUnionType,
    type_function_unknown_type::TypeFunctionUnknownType,
    type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
  },
  type_aliases::{
    type_function_type_pack_id::TypeFunctionTypePackId,
    type_function_type_pack_variant::TypeFunctionTypePackVariantMember,
    type_function_type_variant::TypeFunctionTypeVariantMember,
  },
};

/// cpp `areEqual` 各变体入口共用的环检测守卫：同一对 `(lhs, rhs)` 地址若已在
/// `seen` 中比较过，[`seen_set_contains`] 判定相等并短路递归。泛型 `<T>` 把
/// `lhs/rhs as *const T as *const ()` 的裸指针身份转换样板收敛到单点
/// （此处裸指针是「按对象地址去重」的固有身份语义，非可 Rust 化的借用）。
fn already_seen<T>(seen: &mut AreEqualState, lhs: &T, rhs: &T) -> bool {
  seen_set_contains(
    seen,
    lhs as *const T as *const (),
    rhs as *const T as *const (),
  )
}

pub fn are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionSingletonType,
  rhs: &TypeFunctionSingletonType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  match (&lhs.variant, &rhs.variant) {
    (Variant2::V0(lp), Variant2::V0(rp)) => {
      return lp.value == rp.value;
    }
    (Variant2::V1(lp), Variant2::V1(rp)) => {
      return lp.value == rp.value;
    }
    _ => {}
  }

  false
}

pub fn are_equal_are_equal_state_type_function_union_type_type_function_union_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionUnionType,
  rhs: &TypeFunctionUnionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  let mut l_iter = lhs.components.iter();
  let mut r_iter = rhs.components.iter();

  while let (Some(l), Some(r)) = (l_iter.next(), r_iter.next()) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: `l` 是 `lhs.components: Vec<TypeFunctionTypeId>` 的元素借用，
      // 解出的 `*const TypeFunctionType` 由构造该 union 时 `allocate_type_function_type`
      // 写入 runtime 的 `type_arena: TypedAllocator<TypeFunctionType>`——块只追加、
      // 地址稳定。`lhs`（`&`）存活即其 arena 分量存活，本次比较全程只读，无并存 `&mut`。
      unsafe { &**l },
      // Safety: `r` 同构来自 `rhs.components`，指向 rhs 所属 arena 的分量节点；
      // 由 `&rhs` 借用保证有效，仅生成共享引用。
      unsafe { &**r },
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionIntersectionType,
  rhs: &TypeFunctionIntersectionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.components.len() != rhs.components.len() {
    return false;
  }

  // components 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.components.iter().zip(&rhs.components) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: 解构后 `l: TypeFunctionTypeId = *const TypeFunctionType`，是
      // `lhs.components` 里由 `allocate_type_function_type` 落在 `type_arena` 的
      // 分量指针；arena 节点地址稳定，`&lhs` 借用期内必存活，且此处只做只读解引用。
      unsafe { &*l },
      // Safety: `r` 为 `rhs.components` 对应下标的 arena 分量指针，由 `&rhs`
      // 保证有效，共享引用不产生别名冲突。
      unsafe { &*r },
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionNegationType,
  rhs: &TypeFunctionNegationType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  // Safety: `lhs.type_id`/`rhs.type_id` 均为构造 negation 时经
  // `allocate_type_function_type` 写入 runtime `type_arena` 的类型指针（非空、
  // 地址稳定）。两侧 `&` 借用存活即各自 arena 分量存活，本次比较只读、无别名冲突，
  // 故对整个递归调用期间解引用的两个共享引用均成立。
  unsafe {
    are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      &*lhs.type_id,
      &*rhs.type_id,
    )
  }
}

pub fn are_equal_are_equal_state_type_function_table_type_type_function_table_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTableType,
  rhs: &TypeFunctionTableType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.props.len() != rhs.props.len() {
    return false;
  }

  if (lhs.indexer.is_some()) != (rhs.indexer.is_some()) {
    return false;
  }

  if let (Some(l_indexer), Some(r_indexer)) = (&lhs.indexer, &rhs.indexer) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: `key_type: TypeFunctionTypeId` 是 table 构造时对 key 调用
      // `allocate_type_function_type` 得到的 arena 类型指针（非空、稳定）；`l_indexer`
      // 借自 `&lhs`，借用期内其 indexer 分量必存活，仅取共享引用。
      unsafe { &*l_indexer.key_type },
      // Safety: 同上，`r_indexer.key_type` 指向 rhs 侧 arena 的 key 类型节点。
      unsafe { &*r_indexer.key_type },
    ) {
      return false;
    }

    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: `value_type` 为构造 indexer 时 `allocate_type_function_type`
      // 落在 `type_arena` 的 value 类型指针（非空、稳定），由 `&lhs`/`&rhs` 借用保证存活。
      unsafe { &*l_indexer.value_type },
      // Safety: 对应 rhs 侧 `value_type`，同一 arena 存活不变量下只读解引用。
      unsafe { &*r_indexer.value_type },
    ) {
      return false;
    }
  }

  let mut l_iter = lhs.props.iter();
  let mut r_iter = rhs.props.iter();

  while let (Some((l_key, l_prop)), Some((r_key, r_prop))) = (l_iter.next(), r_iter.next()) {
    let _ = l_key;
    let _ = r_key;

    if (l_prop.read_ty.is_some() && r_prop.read_ty.is_none())
      || (l_prop.read_ty.is_none() && r_prop.read_ty.is_some())
    {
      return false;
    }

    if let (Some(l_read_ty), Some(r_read_ty)) = (&l_prop.read_ty, &r_prop.read_ty)
      && !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        // Safety: `read_ty: Option<TypeFunctionTypeId>`，`Some` 已排除空指针；其内层
        // `*const TypeFunctionType` 由建属性时对 read 类型 `allocate_type_function_type`
        // 落入 `type_arena`（地址稳定），`&lhs`/`&rhs` 借用期内分量存活，仅共享解引用。
        unsafe { &**l_read_ty },
        // Safety: rhs 侧 `read_ty` 同理，Some 保证非空、arena 保证存活。
        unsafe { &**r_read_ty },
      )
    {
      return false;
    }

    if (l_prop.write_ty.is_some() && r_prop.write_ty.is_none())
      || (l_prop.write_ty.is_none() && r_prop.write_ty.is_some())
    {
      return false;
    }

    if let (Some(l_write_ty), Some(r_write_ty)) = (&l_prop.write_ty, &r_prop.write_ty)
      && !are_equal_are_equal_state_type_function_type_type_function_type(
        seen,
        // Safety: `write_ty` 与 read_ty 同族——Option 内 Some 排除 null，指向写类型的
        // arena 节点（`allocate_type_function_type` 稳定分配），随 prop 借用存活。
        unsafe { &**l_write_ty },
        // Safety: 对应 rhs 的 `write_ty` arena 指针，只读解引用无别名。
        unsafe { &**r_write_ty },
      )
    {
      return false;
    }
  }

  true
}

/// 可空 arena pack 指针对（如函数签名 `arg_types`/`ret_types`）的相等判定：
/// 两侧同空视为相等，一空一非空视为不等，皆非空时递归 [`are_equal_
/// are_equal_state_type_function_type_pack_var_type_function_type_pack_var`]。
/// 收敛 function/variadic 等入口逐字重复的两段判空守卫。
fn are_equal_optional_pack(
  seen: &mut AreEqualState,
  lhs: TypeFunctionTypePackId,
  rhs: TypeFunctionTypePackId,
) -> bool {
  if lhs.is_null() != rhs.is_null() {
    return false;
  }
  if lhs.is_null() {
    return true;
  }
  // Safety: 已双侧 `!is_null()` 守卫，二者均为 `allocate_type_function_type_pack`
  // 落在 `type_pack_arena` 的地址稳定存活 pack；随外层 `&lhs`/`&rhs` 借用存续，只读解引用无别名。
  unsafe {
    are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      seen, &*lhs, &*rhs,
    )
  }
}

pub fn are_equal_are_equal_state_type_function_function_type_type_function_function_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionFunctionType,
  rhs: &TypeFunctionFunctionType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  if lhs.generics.len() != rhs.generics.len() {
    return false;
  }

  // generics 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.generics.iter().zip(&rhs.generics) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: `generics: Vec<TypeFunctionTypeId>`，元素 `l` 是构造泛型签名时
      // `allocate_type_function_type` 落在 `type_arena` 的类型指针（非空、块内地址稳定）；
      // `&lhs` 借用存续即该 arena 分量存续，仅共享解引用。
      unsafe { &*l },
      // Safety: `r` 来自 rhs.generics，同一 arena 存活不变量下的只读解引用。
      unsafe { &*r },
    ) {
      return false;
    }
  }

  if lhs.generic_packs.len() != rhs.generic_packs.len() {
    return false;
  }

  for (&l, &r) in lhs.generic_packs.iter().zip(&rhs.generic_packs) {
    if !are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
      seen,
      // Safety: `generic_packs: Vec<TypeFunctionTypePackId>`，解构后 `l` 为
      // `*const TypeFunctionTypePackVar`，由 `allocate_type_function_type_pack` 落在
      // runtime 的 `type_pack_arena`（地址稳定）；随 `&lhs` 借用存活，只读无别名。
      unsafe { &*l },
      // Safety: rhs 侧 pack 变量指针，同一 type_pack_arena 不变量。
      unsafe { &*r },
    ) {
      return false;
    }
  }

  if !are_equal_optional_pack(seen, lhs.arg_types, rhs.arg_types) {
    return false;
  }

  if !are_equal_optional_pack(seen, lhs.ret_types, rhs.ret_types) {
    return false;
  }

  true
}

pub fn are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionExternType,
  rhs: &TypeFunctionExternType,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  lhs.extern_ty == rhs.extern_ty
}

/// 对照 C++ `get<T>(&tv)`（TypeFunctionRuntime.cpp:275-281）：双侧变体一次
/// 下转成引用对。直接走安全 trait `T::get_if`，tv 由 `&TypeFunctionType`
/// 引用保证非空，语义与 C++ `get<T>` 完全一致，免去 unsafe。
fn pair<'a, 'b, T: TypeFunctionTypeVariantMember>(
  lhs: &'a TypeFunctionType,
  rhs: &'b TypeFunctionType,
) -> Option<(&'a T, &'b T)> {
  Some((T::get_if(&lhs.type_variant)?, T::get_if(&rhs.type_variant)?))
}

pub fn are_equal_are_equal_state_type_function_type_type_function_type(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionType,
  rhs: &TypeFunctionType,
) -> bool {
  let mut _ra: Option<RecursionLimiter> = None;
  if LuauTypeFunctionRobustness.get() {
    _ra = Some(RecursionLimiter::new(
      "areEqual",
      &mut seen.recursion_count,
      100,
    ));
  }

  if lhs.type_variant.index() != rhs.type_variant.index() {
    return false;
  }

  // 对照 C++ areEqual 主体（TypeFunctionRuntime.cpp:2404-2447）：逐变体下转，
  // 顺序与 C++ 完全一致，全部经由安全 get_if，免裸指针。
  if let Some((lp, rp)) = pair::<TypeFunctionPrimitiveType>(lhs, rhs) {
    return lp.r#type == rp.r#type;
  }

  if pair::<TypeFunctionAnyType>(lhs, rhs).is_some() {
    return true;
  }

  if pair::<TypeFunctionUnknownType>(lhs, rhs).is_some() {
    return true;
  }

  if pair::<TypeFunctionNeverType>(lhs, rhs).is_some() {
    return true;
  }

  if let Some((lf, rf)) = pair::<TypeFunctionSingletonType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionUnionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_union_type_type_function_union_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionIntersectionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionNegationType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
      seen, lf, rf,
    );
  }

  if let Some((lt, rt)) = pair::<TypeFunctionTableType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_table_type_type_function_table_type(
      seen, lt, rt,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionFunctionType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_function_type_type_function_function_type(
      seen, lf, rf,
    );
  }

  if let Some((lf, rf)) = pair::<TypeFunctionExternType>(lhs, rhs) {
    return are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
      seen, lf, rf,
    );
  }

  // C++（TypeFunctionRuntime.cpp:2456-2462）：Generic 分支逐字段比较。
  if let Some((lg, rg)) = pair::<TypeFunctionGenericType>(lhs, rhs) {
    return lg.is_named == rg.is_named && lg.is_pack == rg.is_pack && lg.name == rg.name;
  }

  false
}

pub fn are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePack,
  rhs: &TypeFunctionTypePack,
) -> bool {
  if lhs.head.len() != rhs.head.len() {
    return false;
  }

  // head 按下标一一对应，zip 替代索引遍历
  for (&l, &r) in lhs.head.iter().zip(&rhs.head) {
    if !are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      // Safety: `head: Vec<TypeFunctionTypeId>`，元素 `l` 为 pack 构造时
      // `allocate_type_function_type` 写入 `type_arena` 的头类型指针（非空、地址稳定），
      // 随 `&lhs` 借用存活，仅共享解引用。
      unsafe { &*l },
      // Safety: rhs.head 对应元素，同一 type_arena 存活不变量下的只读解引用。
      unsafe { &*r },
    ) {
      return false;
    }
  }

  true
}

pub fn are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionVariadicTypePack,
  rhs: &TypeFunctionVariadicTypePack,
) -> bool {
  if already_seen(seen, lhs, rhs) {
    return true;
  }

  // Safety: `lhs.type_id`/`rhs.type_id` 是变长 pack 构造时对元素类型
  // `allocate_type_function_type` 落入 `type_arena` 的类型指针（非空、地址稳定）；两侧
  // `&` 借用存续即 arena 分量存续，递归比较全程只读，故两个共享解引用均成立。
  unsafe {
    are_equal_are_equal_state_type_function_type_type_function_type(
      seen,
      &*lhs.type_id,
      &*rhs.type_id,
    )
  }
}

/// C++ `bool areEqual(AreEqualState& seen, const TypeFunctionTypePackVar& lhs,
/// const TypeFunctionTypePackVar& rhs)` — the pack-variant dispatcher.
pub fn are_equal_are_equal_state_type_function_type_pack_var_type_function_type_pack_var(
  seen: &mut AreEqualState,
  lhs: &TypeFunctionTypePackVar,
  rhs: &TypeFunctionTypePackVar,
) -> bool {
  {
    let lb = TypeFunctionTypePack::get_if(&lhs.type_variant);
    let rb = TypeFunctionTypePack::get_if(&rhs.type_variant);
    if let (Some(lb), Some(rb)) = (lb, rb) {
      return are_equal_are_equal_state_type_function_type_pack_type_function_type_pack(
        seen, lb, rb,
      );
    }
  }

  {
    let lv = TypeFunctionVariadicTypePack::get_if(&lhs.type_variant);
    let rv = TypeFunctionVariadicTypePack::get_if(&rhs.type_variant);
    if let (Some(lv), Some(rv)) = (lv, rv) {
      return are_equal_are_equal_state_type_function_variadic_type_pack_type_function_variadic_type_pack(
                seen, lv, rv,
            );
    }
  }

  {
    let lg = TypeFunctionGenericTypePack::get_if(&lhs.type_variant);
    let rg = TypeFunctionGenericTypePack::get_if(&rhs.type_variant);
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.is_named == rg.is_named && lg.name == rg.name;
    }
  }

  false
}
