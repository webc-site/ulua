use crate::{
  functions::{get_type_alt_j::get_type_id, is_subclass_type::is_subclass_extern_type_extern_type},
  records::{extern_type::ExternType, normalized_extern_type::NormalizedExternType},
  type_aliases::type_id::TypeId,
};

pub fn are_normalized_extern_types(tys: &NormalizedExternType) -> bool {
  let extern_types = &tys.extern_types;

  for (ty, negations) in extern_types.iter() {
    if get_type_id::<ExternType>(*ty).is_none() {
      return false;
    }

    for &negation in &negations.order {
      if get_type_id::<ExternType>(negation).is_none() {
        return false;
      }

      let etv = get_type_id::<ExternType>(*ty).unwrap();
      let nctv = get_type_id::<ExternType>(negation).unwrap();

      if !is_subclass_extern_type_extern_type(nctv, etv) {
        return false;
      }
    }

    for (other_ty, other_negations) in extern_types.iter() {
      if *other_ty == *ty {
        continue;
      }

      if get_type_id::<ExternType>(*other_ty).is_none() {
        return false;
      }

      let etv = get_type_id::<ExternType>(*ty).unwrap();
      let octv = get_type_id::<ExternType>(*other_ty).unwrap();

      if is_subclass_extern_type_extern_type(etv, octv) {
        let iss = |t: TypeId| -> bool {
          let c = get_type_id::<ExternType>(t).unwrap();
          is_subclass_extern_type_extern_type(etv, c)
        };

        if !other_negations.order.iter().any(|&t| iss(t)) {
          return false;
        }
      }
    }
  }

  true
}
