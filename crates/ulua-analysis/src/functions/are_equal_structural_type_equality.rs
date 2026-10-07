use core::ptr::from_ref;

use crate::{
  functions::{are_seen::are_seen, begin_type_pack::begin, get_type_pack::type_pack_variant_of},
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
  // `from_ref`：`&TypePackVar → TypePackId`（裸指针别名）的既有收口形态。
  let mut lhs_iter = begin(from_ref(lhs));
  let mut rhs_iter = begin(from_ref(rhs));

  // 双侧逐项同步推进：一侧耗尽而另一侧尚有元素即长度不齐，直接判不等。
  loop {
    match (lhs_iter.next(), rhs_iter.next()) {
      (Some(l), Some(r)) => {
        if !are_equal_seen_set_type_item_type_item(seen, alias_ref(l), alias_ref(r)) {
          return false;
        }
      }
      (None, None) => break,
      _ => return false,
    }
  }

  let (lhs_tail, rhs_tail) = match (lhs_iter.tail(), rhs_iter.tail()) {
    (None, None) => return true,
    (Some(l), Some(r)) => (l, r),
    // 单侧有尾包：形状不等。
    _ => return false,
  };

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
  if are_seen(seen, lhs, rhs) {
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
  // 节点地址身份比较已收口进 `are_seen`（泛型引用形参）。
  if are_seen(seen, lhs, rhs) {
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
  // 节点地址身份比较已收口进 `are_seen`（泛型引用形参）。
  if are_seen(seen, lhs, rhs) {
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

  // DELIBERATE DEVIATION: cpp `StructuralTypeEquality.cpp:143-156` 因复制粘贴
  // 存在两段完全相同的 GenericType 比较（第二段永不可达），Rust 侧合并为一段，
  // 行为逐位等价。
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
