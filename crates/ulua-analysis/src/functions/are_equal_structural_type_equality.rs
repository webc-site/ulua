use crate::{
  functions::{
    are_seen::are_seen, begin_type_pack::begin, end_type_pack::end,
    get_type_pack::type_pack_variant_of,
  },
  records::{
    any_type::AnyType, arena_handle::alias_ref, free_type::FreeType, free_type_pack::FreeTypePack,
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    metatable_type::MetatableType, primitive_type::PrimitiveType, table_type::TableType,
    r#type::Type, type_pack_var::TypePackVar, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type::BoundType, bound_type_pack::BoundTypePack, error_type::ErrorType,
    seen_set_structural_type_equality::SeenSet, type_pack_variant::TypePackVariantMember,
    type_variant::TypeVariantMember,
  },
};

pub fn are_equal_seen_set_type_pack_var_type_pack_var(
  seen: &mut SeenSet,
  lhs: &TypePackVar,
  rhs: &TypePackVar,
) -> bool {
  let lhs_id = lhs as *const TypePackVar;
  let rhs_id = rhs as *const TypePackVar;

  let mut lhs_iter = begin(lhs_id);
  let mut rhs_iter = begin(rhs_id);
  let lhs_end = end(lhs_id);
  let rhs_end = end(rhs_id);

  while lhs_iter != lhs_end && rhs_iter != rhs_end {
    let l = *lhs_iter.current();
    let r = *rhs_iter.current();
    if !are_equal_seen_set_type_item_type_item(seen, alias_ref(l), alias_ref(r)) {
      return false;
    }
    lhs_iter.advance();
    rhs_iter.advance();
  }

  if lhs_iter != lhs_end || rhs_iter != rhs_end {
    return false;
  }

  if lhs_iter.tail().is_none() && rhs_iter.tail().is_none() {
    return true;
  }
  if lhs_iter.tail().is_none() || rhs_iter.tail().is_none() {
    return false;
  }

  // Safety: 上方两处 is_none 判定均已早返，两侧 tail 至此必为 Some。
  let lhs_tail = lhs_iter
    .tail()
    .expect("上方双侧 is_none 判定均已早返，至此必为 Some");
  let rhs_tail = rhs_iter
    .tail()
    .expect("上方双侧 is_none 判定均已早返，至此必为 Some");

  // `lhs_tail`/`rhs_tail` 的变体读取收口在 `type_pack_variant_of`（arena 节点
  // 有效性契约同 C++ `get(TypePackId)`）。分支内的 `lb.bound_to`/`rb.bound_to`
  // （`Bound<TypePackId>`）与 `lv.ty`/`rv.ty`（`VariadicTypePack` 元素
  // `TypeId`）同为 arena 节点句柄（C++ `*lb->boundTo`、`*lv->ty`），本次结构
  // 相等比较对它们是纯只读遍历，`seen` 是调用方独占的本地集合，不产生可变
  // 别名冲突。
  {
    let lf = FreeTypePack::get_if(type_pack_variant_of(lhs_tail));
    let rf = FreeTypePack::get_if(type_pack_variant_of(rhs_tail));
    if let (Some(lf), Some(rf)) = (lf, rf) {
      return lf.index == rf.index;
    }
  }

  {
    let lb = BoundTypePack::get_if(type_pack_variant_of(lhs_tail));
    let rb = BoundTypePack::get_if(type_pack_variant_of(rhs_tail));
    if let (Some(lb), Some(rb)) = (lb, rb) {
      return are_equal_seen_set_type_pack_var_type_pack_var(
        seen,
        alias_ref(lb.bound_to),
        alias_ref(rb.bound_to),
      );
    }
  }

  {
    let lg = GenericTypePack::get_if(type_pack_variant_of(lhs_tail));
    let rg = GenericTypePack::get_if(type_pack_variant_of(rhs_tail));
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.index == rg.index;
    }
  }

  {
    let lv = VariadicTypePack::get_if(type_pack_variant_of(lhs_tail));
    let rv = VariadicTypePack::get_if(type_pack_variant_of(rhs_tail));
    if let (Some(lv), Some(rv)) = (lv, rv) {
      return are_equal_seen_set_type_item_type_item(seen, alias_ref(lv.ty), alias_ref(rv.ty));
    }
  }

  false
}

pub fn are_equal_seen_set_function_type_function_type(
  seen: &mut SeenSet,
  lhs: &FunctionType,
  rhs: &FunctionType,
) -> bool {
  if are_seen(
    seen,
    lhs as *const FunctionType as *const (),
    rhs as *const FunctionType as *const (),
  ) {
    return true;
  }

  let lhs_arg_types = alias_ref(lhs.arg_types);
  let rhs_arg_types = alias_ref(rhs.arg_types);
  if !are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_arg_types, rhs_arg_types) {
    return false;
  }

  let lhs_ret_types = alias_ref(lhs.ret_types);
  let rhs_ret_types = alias_ref(rhs.ret_types);
  are_equal_seen_set_type_pack_var_type_pack_var(seen, lhs_ret_types, rhs_ret_types)
}

pub fn are_equal_seen_set_table_type_table_type(
  seen: &mut SeenSet,
  lhs: &TableType,
  rhs: &TableType,
) -> bool {
  // are_seen expects BTreeSet<(*const (), *const ())>; 这里用 `as *const ()`
  // 指针转型即匹配该签名（非 transmute，对应 C++ 侧的 const 性转换注释）。
  if are_seen(
    seen,
    lhs as *const TableType as *const (),
    rhs as *const TableType as *const (),
  ) {
    return true;
  }

  if lhs.state != rhs.state {
    return false;
  }

  if lhs.props.len() != rhs.props.len() {
    return false;
  }

  if (lhs.indexer.is_some()) != (rhs.indexer.is_some()) {
    return false;
  }

  if let (Some(l_indexer), Some(r_indexer)) = (&lhs.indexer, &rhs.indexer) {
    if l_indexer.is_read_only != r_indexer.is_read_only {
      return false;
    }

    if !are_equal_seen_set_type_item_type_item(
      seen,
      alias_ref(l_indexer.index_type),
      alias_ref(r_indexer.index_type),
    ) {
      return false;
    }

    if !are_equal_seen_set_type_item_type_item(
      seen,
      alias_ref(l_indexer.index_result_type),
      alias_ref(r_indexer.index_result_type),
    ) {
      return false;
    }
  }

  let mut l_iter = lhs.props.iter();
  let mut r_iter = rhs.props.iter();

  while let (Some((l_key, l_prop)), Some((r_key, r_prop))) = (l_iter.next(), r_iter.next()) {
    if l_key != r_key {
      return false;
    }

    if let (Some(l_read), Some(r_read)) = (&l_prop.read_ty, &r_prop.read_ty) {
      if !are_equal_seen_set_type_item_type_item(seen, alias_ref(*l_read), alias_ref(*r_read)) {
        return false;
      }
    } else if l_prop.read_ty.is_some() || r_prop.read_ty.is_some() {
      return false;
    }

    if let (Some(l_write), Some(r_write)) = (&l_prop.write_ty, &r_prop.write_ty) {
      if !are_equal_seen_set_type_item_type_item(seen, alias_ref(*l_write), alias_ref(*r_write)) {
        return false;
      }
    } else if l_prop.write_ty.is_some() || r_prop.write_ty.is_some() {
      return false;
    }
  }

  true
}

pub fn are_equal_seen_set_metatable_type_metatable_type(
  seen: &mut SeenSet,
  lhs: &MetatableType,
  rhs: &MetatableType,
) -> bool {
  // The SeenSet in this context is BTreeSet<(*const (), *const ())>.
  // are_seen expects BTreeSet<(*const (), *const ())>.
  // We must cast the seen set pointer to match the expected mutability of the are_seen signature.
  if are_seen(
    seen,
    lhs as *const MetatableType as *const (),
    rhs as *const MetatableType as *const (),
  ) {
    return true;
  }

  are_equal_seen_set_type_item_type_item(seen, alias_ref(lhs.table), alias_ref(rhs.table))
    && are_equal_seen_set_type_item_type_item(
      seen,
      alias_ref(lhs.metatable),
      alias_ref(rhs.metatable),
    )
}

pub fn are_equal_seen_set_type_item_type_item(seen: &mut SeenSet, lhs: &Type, rhs: &Type) -> bool {
  if let Some(bound) = BoundType::get_if(&lhs.ty) {
    return are_equal_seen_set_type_item_type_item(seen, alias_ref(bound.bound_to), rhs);
  }

  if let Some(bound) = BoundType::get_if(&rhs.ty) {
    return are_equal_seen_set_type_item_type_item(seen, lhs, alias_ref(bound.bound_to));
  }

  if lhs.ty.index() != rhs.ty.index() {
    return false;
  }

  {
    let lf = FreeType::get_if(&lhs.ty);
    let rf = FreeType::get_if(&rhs.ty);
    if let (Some(lf), Some(rf)) = (lf, rf) {
      return lf.index == rf.index;
    }
  }

  {
    let lg = GenericType::get_if(&lhs.ty);
    let rg = GenericType::get_if(&rhs.ty);
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.index == rg.index;
    }
  }

  {
    let lp = PrimitiveType::get_if(&lhs.ty);
    let rp = PrimitiveType::get_if(&rhs.ty);
    if let (Some(lp), Some(rp)) = (lp, rp) {
      return lp.r#type == rp.r#type;
    }
  }

  {
    let lg = GenericType::get_if(&lhs.ty);
    let rg = GenericType::get_if(&rhs.ty);
    if let (Some(lg), Some(rg)) = (lg, rg) {
      return lg.index == rg.index;
    }
  }

  {
    let le = ErrorType::get_if(&lhs.ty);
    let re = ErrorType::get_if(&rhs.ty);
    if let (Some(le), Some(re)) = (le, re) {
      return le.index == re.index;
    }
  }

  {
    let lf = FunctionType::get_if(&lhs.ty);
    let rf = FunctionType::get_if(&rhs.ty);
    if let (Some(lf), Some(rf)) = (lf, rf) {
      return are_equal_seen_set_function_type_function_type(seen, lf, rf);
    }
  }

  {
    let lt = TableType::get_if(&lhs.ty);
    let rt = TableType::get_if(&rhs.ty);
    if let (Some(lt), Some(rt)) = (lt, rt) {
      return are_equal_seen_set_table_type_table_type(seen, lt, rt);
    }
  }

  {
    let lmt = MetatableType::get_if(&lhs.ty);
    let rmt = MetatableType::get_if(&rhs.ty);
    if let (Some(lmt), Some(rmt)) = (lmt, rmt) {
      return are_equal_seen_set_metatable_type_metatable_type(seen, lmt, rmt);
    }
  }

  AnyType::get_if(&lhs.ty).is_some() && AnyType::get_if(&rhs.ty).is_some()
}
