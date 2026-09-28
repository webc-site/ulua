//! `unionOfTypePacks`（Normalize.cpp:1796）与 `intersectionOfTypePacks_INTERNAL`
//! （Normalize.cpp:3319 之后）的同形骨架合一：两侧仅「组合算子」与「包含标志的
//! 清除方向」对偶（cpp 两函数逐行镜像），公共遍历抽此处，参数化 Meet/Join。
use alloc::vec::Vec;

use crate::{
  functions::{begin_type_pack::begin, end_type_pack::end_type_pack_id, get_type_pack},
  records::{
    normalizer::Normalizer, type_pack::TypePack, type_pack_iterator::TypePackIterator,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

/// 类型包上的半格算子：Meet = 交（GLB），Join = 并（LUB）。
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum PackOp {
  Meet,
  Join,
}

impl PackOp {
  fn is_join(self) -> bool {
    self == PackOp::Join
  }

  /// cpp `unionType`/`intersectionType` 的分派点。
  fn combine(self, n: &mut Normalizer, a: TypeId, b: TypeId) -> TypeId {
    match self {
      PackOp::Meet => n.intersection_type(a, b),
      PackOp::Join => n.union_type(a, b),
    }
  }

  /// head 段（含 dealWithDifferentArities）的标志方向随算子对偶：Meet 时
  /// `ty != hty ⇒ here⊄there`；Join 对偶为 `⇒ there⊄here`。
  fn note_diff_head(
    self,
    ty: TypeId,
    hty: TypeId,
    tty: TypeId,
    here_sub_there: &mut bool,
    there_sub_here: &mut bool,
  ) {
    if ty != hty {
      if self.is_join() {
        *there_sub_here = false;
      } else {
        *here_sub_there = false;
      }
    }
    if ty != tty {
      if self.is_join() {
        *here_sub_there = false;
      } else {
        *there_sub_here = false;
      }
    }
  }
}

impl Normalizer {
  /// union_of_type_packs / intersection_of_type_packs_internal 的公共骨架。
  pub(crate) fn combine_type_packs(
    &mut self,
    op: PackOp,
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

    let mut ith = begin(here);
    let mut itt = begin(there);
    let end_ith = end_type_pack_id(here);
    let end_itt = end_type_pack_id(there);

    while ith != end_ith && itt != end_itt {
      let hty = *ith.current();
      let tty = *itt.current();
      let ty = op.combine(self, hty, tty);
      op.note_diff_head(ty, hty, tty, &mut here_sub_there, &mut there_sub_here);
      head.push(ty);
      ith.advance();
      itt.advance();
    }

    // cpp `dealWithDifferentArities`：一侧先耗尽 head 时，用另一侧的
    // VariadicTypePack 尾元类型继续逐个组合；无尾或尾非 variadic 即不可比。
    fn deal_with_different_arities(
      n: &mut Normalizer,
      op: PackOp,
      head: &mut Vec<TypeId>,
      ith: &mut TypePackIterator,
      itt: TypePackIterator,
      here: TypePackId,
      here_sub_there: &mut bool,
      there_sub_here: &mut bool,
    ) -> bool {
      if *ith == end_type_pack_id(here) {
        return true;
      }
      let Some(tail) = itt.tail() else {
        // Type packs of different arities are incomparable
        return false;
      };
      let Some(vtp) = get_type_pack::get::<VariadicTypePack>(tail) else {
        // Luau doesn't have unions/intersections of type pack variables
        return false;
      };
      let tty = vtp.ty;

      while *ith != end_type_pack_id(here) {
        let hty = *ith.current();
        let ty = op.combine(n, hty, tty);
        op.note_diff_head(ty, hty, tty, here_sub_there, there_sub_here);
        head.push(ty);
        ith.advance();
      }
      true
    }

    if !deal_with_different_arities(
      self,
      op,
      &mut head,
      &mut ith,
      itt.clone(),
      here,
      &mut here_sub_there,
      &mut there_sub_here,
    ) {
      return None;
    }

    if !deal_with_different_arities(
      self,
      op,
      &mut head,
      &mut itt,
      ith.clone(),
      there,
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
            // Luau doesn't have unions/intersections of type pack variables
            return None;
          };

          let ty = op.combine(self, hvtp.ty, tvtp.ty);
          // 尾元对组合段两 cpp 实现都用 join 方向的标志映射；intersection
          // 实现此处与其 head 段方向相反（上游 Normalize.cpp 原样，faithful 保留）。
          PackOp::Join.note_diff_head(
            ty,
            hvtp.ty,
            tvtp.ty,
            &mut here_sub_there,
            &mut there_sub_here,
          );
          let hidden = hvtp.hidden & tvtp.hidden;
          // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程无并存别名。
          tail = Some(
            self
              .wired_arena_mut()
              .add_type_pack_t(VariadicTypePack { ty, hidden }),
          );
        }
      } else if get_type_pack::get::<VariadicTypePack>(htail_val).is_some() {
        here_sub_there = false;
        // cpp 仅 union 实现在此回填 tail（intersection 实现不写，保持 None）。
        if op.is_join() {
          tail = Some(htail_val);
        }
      } else {
        // Luau doesn't have unions/intersections of type pack variables
        return None;
      }
    } else if let Some(ttail_val) = ttail {
      if get_type_pack::get::<VariadicTypePack>(ttail_val).is_some() {
        there_sub_here = false;
        // cpp union 实现的 `tail = htail` 此处 htail 恒 None，等价不写。
      } else {
        // Luau doesn't have unions/intersections of type pack variables
        return None;
      }
    }

    // here ⊆ there 时：交取 here，并取 there（cpp 两实现同款对偶）。
    if here_sub_there {
      return Some(if op.is_join() { there } else { here });
    } else if there_sub_here {
      return Some(if op.is_join() { here } else { there });
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
      // 契约：同上；TODO(cpp): Add an emptyPack to singleton types
      Some(self.wired_arena_mut().add_type_pack_t(TypePack::empty()))
    }
  }
}
