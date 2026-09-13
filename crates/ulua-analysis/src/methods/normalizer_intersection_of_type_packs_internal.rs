use alloc::vec::Vec;

use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id, end_type_pack::end_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    normalizer::Normalizer, type_pack::TypePack, type_pack_iterator::TypePackIterator,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Normalizer {
  pub fn intersection_of_type_packs_internal(
    &mut self,
    here: TypePackId,
    there: TypePackId,
  ) -> Option<TypePackId> {
    self.consume_fuel();

    if here == there {
      return Some(here);
    }

    let mut head: Vec<TypeId> = Vec::new();
    let mut tail: Option<TypePackId> = None;

    let mut here_sub_there = true;
    let mut there_sub_here = true;

    let mut ith = begin_type_pack_id(here);
    let mut itt = begin_type_pack_id(there);
    let end_ith = end_type_pack_id(here);
    let end_itt = end_type_pack_id(there);

    while ith.operator_ne(&end_ith) && itt.operator_ne(&end_itt) {
      let hty = *ith.operator_deref();
      let tty = *itt.operator_deref();
      let ty = self.intersection_type(hty, tty);
      if ty != hty {
        here_sub_there = false;
      }
      if ty != tty {
        there_sub_here = false;
      }
      head.push(ty);
      ith.operator_inc();
      itt.operator_inc();
    }

    let mut deal_with_different_arities = |ith: &mut TypePackIterator,
                                           itt: TypePackIterator,
                                           here: TypePackId,
                                           _there: TypePackId,
                                           here_sub_there: &mut bool,
                                           there_sub_here: &mut bool|
     -> bool {
      if ith.operator_ne(&end_type_pack_id(here)) {
        let tty = match itt.tail().and_then(get_type_pack_id::<VariadicTypePack>) {
          Some(vtp) => vtp.ty,
          None => return false,
        };

        while ith.operator_ne(&end_type_pack_id(here)) {
          let hty = *ith.operator_deref();
          let ty = self.intersection_type(hty, tty);
          if ty != hty {
            *here_sub_there = false;
          }
          if ty != tty {
            *there_sub_here = false;
          }
          head.push(ty);
          ith.operator_inc();
        }
      }
      true
    };

    if !deal_with_different_arities(
      &mut ith,
      itt.clone(),
      here,
      there,
      &mut here_sub_there,
      &mut there_sub_here,
    ) {
      return None;
    }

    if !deal_with_different_arities(
      &mut itt,
      ith.clone(),
      there,
      here,
      &mut there_sub_here,
      &mut here_sub_there,
    ) {
      return None;
    }

    let htail = ith.tail();
    let ttail = itt.tail();

    if let Some(htail_val) = htail {
      if let Some(ttail_val) = ttail {
        if htail_val == ttail_val {
          tail = Some(htail_val);
        } else {
          let hvtp = get_type_pack_id::<VariadicTypePack>(htail_val);
          let tvtp = get_type_pack_id::<VariadicTypePack>(ttail_val);

          if let (Some(hvtp), Some(tvtp)) = (hvtp, tvtp) {
            let ty = self.intersection_type(hvtp.ty, tvtp.ty);
            if ty != hvtp.ty {
              there_sub_here = false;
            }
            if ty != tvtp.ty {
              here_sub_there = false;
            }
            let hidden = hvtp.hidden & tvtp.hidden;
            tail = Some(unsafe { (*self.arena).add_type_pack_t(VariadicTypePack { ty, hidden }) });
          } else {
            return None;
          }
        }
      } else {
        let hvtp = get_type_pack_id::<VariadicTypePack>(htail_val);
        if hvtp.is_some() {
          here_sub_there = false;
        } else {
          return None;
        }
      }
    } else if let Some(ttail_val) = ttail {
      let tvtp = get_type_pack_id::<VariadicTypePack>(ttail_val);
      if tvtp.is_some() {
        there_sub_here = false;
      } else {
        return None;
      }
    }

    if here_sub_there {
      return Some(here);
    } else if there_sub_here {
      return Some(there);
    }

    if !head.is_empty() {
      Some(unsafe { (*self.arena).add_type_pack_t(TypePack { head, tail }) })
    } else if let Some(t) = tail {
      Some(t)
    } else {
      Some(unsafe {
        (*self.arena).add_type_pack_t(TypePack {
          head: Vec::new(),
          tail: None,
        })
      })
    }
  }
}
