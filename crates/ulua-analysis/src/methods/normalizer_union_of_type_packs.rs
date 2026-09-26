use alloc::vec::Vec;

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end_type_pack_id, get_type_pack},
  records::{
    normalizer::Normalizer, type_pack::TypePack, type_pack_iterator::TypePackIterator,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Normalizer {
  pub fn union_of_type_packs(&mut self, here: TypePackId, there: TypePackId) -> Option<TypePackId> {
    self.consume_fuel();

    if here == there {
      return Some(here);
    }

    let mut head: Vec<TypeId> = Vec::new();
    let mut tail: Option<TypePackId> = None;

    let mut here_sub_there = true;
    let mut there_sub_here = true;

    let mut ith = begin(here);
    let mut itt = begin(there);
    let end_ith = end_type_pack_id(here);
    let end_itt = end_type_pack_id(there);

    while ith != end_ith && itt != end_itt {
      let hty = *ith.current();
      let tty = *itt.current();
      let ty = self.union_type(hty, tty);
      if ty != hty {
        there_sub_here = false;
      }
      if ty != tty {
        here_sub_there = false;
      }
      head.push(ty);
      ith.advance();
      itt.advance();
    }

    let mut deal_with_different_arities = |ith: &mut TypePackIterator,
                                           itt: TypePackIterator,
                                           here: TypePackId,
                                           _there: TypePackId,
                                           here_sub_there: &mut bool,
                                           there_sub_here: &mut bool|
     -> bool {
      if *ith != end_type_pack_id(here) {
        let Some(ttail) = itt.tail() else {
          return false;
        };
        let Some(p) = get_type_pack::get::<VariadicTypePack>(ttail) else {
          return false;
        };
        let tty = p.ty;

        while *ith != end_type_pack_id(here) {
          let hty = *ith.current();
          let ty = self.union_type(hty, tty);
          if ty != hty {
            *there_sub_here = false;
          }
          if ty != tty {
            *here_sub_there = false;
          }
          head.push(ty);
          ith.advance();
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
          let (Some(hvtp), Some(tvtp)) = (
            get_type_pack::get::<VariadicTypePack>(htail_val),
            get_type_pack::get::<VariadicTypePack>(ttail_val),
          ) else {
            return None;
          };

          let ty = self.union_type(hvtp.ty, tvtp.ty);
          if ty != hvtp.ty {
            there_sub_here = false;
          }
          if ty != tvtp.ty {
            here_sub_there = false;
          }
          let hidden = hvtp.hidden & tvtp.hidden;
          // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程无并存别名。
          tail = Some(
            self
              .wired_arena_mut()
              .add_type_pack_t(VariadicTypePack { ty, hidden }),
          );
        }
      } else {
        get_type_pack::get::<VariadicTypePack>(htail_val)?;
        here_sub_there = false;
        tail = Some(htail_val);
      }
    } else if let Some(ttail_val) = ttail {
      get_type_pack::get::<VariadicTypePack>(ttail_val)?;
      there_sub_here = false;
      tail = htail;
    }

    if here_sub_there {
      return Some(there);
    } else if there_sub_here {
      return Some(here);
    }

    if !head.is_empty() {
      // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程无并存别名。
      Some(
        self
          .wired_arena_mut()
          .add_type_pack_t(TypePack::new(head, tail)),
      )
    } else if let Some(t) = tail {
      Some(t)
    } else {
      // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程无并存别名。
      Some(self.wired_arena_mut().add_type_pack_t(TypePack::empty()))
    }
  }
}
