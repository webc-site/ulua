use crate::{
  functions::{
    are_equal_structural_type_equality_alt_e::are_equal_seen_set_type_item_type_item,
    begin_type_pack::begin, end_type_pack::end,
  },
  records::{
    free_type_pack::FreeTypePack, generic_type_pack::GenericTypePack, type_pack_var::TypePackVar,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, seen_set_structural_type_equality::SeenSet,
    type_pack_variant::TypePackVariantMember,
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

  while lhs_iter.operator_ne(&lhs_end) && rhs_iter.operator_ne(&rhs_end) {
    let l = *lhs_iter.operator_deref();
    let r = *rhs_iter.operator_deref();
    if !are_equal_seen_set_type_item_type_item(seen, unsafe { &*l }, unsafe { &*r }) {
      return false;
    }
    lhs_iter.operator_inc();
    rhs_iter.operator_inc();
  }

  if lhs_iter.operator_ne(&lhs_end) || rhs_iter.operator_ne(&rhs_end) {
    return false;
  }

  if lhs_iter.tail().is_none() && rhs_iter.tail().is_none() {
    return true;
  }
  if lhs_iter.tail().is_none() || rhs_iter.tail().is_none() {
    return false;
  }

  let lhs_tail = lhs_iter.tail().unwrap();
  let rhs_tail = rhs_iter.tail().unwrap();

  unsafe {
    {
      let lf = FreeTypePack::get_if(&(*lhs_tail).ty);
      let rf = FreeTypePack::get_if(&(*rhs_tail).ty);
      if let (Some(lf), Some(rf)) = (lf, rf) {
        return lf.index == rf.index;
      }
    }

    {
      let lb = BoundTypePack::get_if(&(*lhs_tail).ty);
      let rb = BoundTypePack::get_if(&(*rhs_tail).ty);
      if let (Some(lb), Some(rb)) = (lb, rb) {
        return are_equal_seen_set_type_pack_var_type_pack_var(seen, &*lb.bound_to, &*rb.bound_to);
      }
    }

    {
      let lg = GenericTypePack::get_if(&(*lhs_tail).ty);
      let rg = GenericTypePack::get_if(&(*rhs_tail).ty);
      if let (Some(lg), Some(rg)) = (lg, rg) {
        return lg.index == rg.index;
      }
    }

    {
      let lv = VariadicTypePack::get_if(&(*lhs_tail).ty);
      let rv = VariadicTypePack::get_if(&(*rhs_tail).ty);
      if let (Some(lv), Some(rv)) = (lv, rv) {
        return are_equal_seen_set_type_item_type_item(seen, &*lv.ty, &*rv.ty);
      }
    }
  }

  false
}
