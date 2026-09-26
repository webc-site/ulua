use alloc::vec::Vec;
use core::ptr::NonNull;

use crate::{
  functions::get_mutable_type,
  records::{
    blocked_type::BlockedType, constraint::Constraint, constraint_solver::ConstraintSolver,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintSolver {
  pub fn unpack_and_assign(
    &mut self,
    dest_types: Vec<TypeId>,
    src_types: TypePackId,
    constraint: NonNull<Constraint>,
  ) -> NonNull<Constraint> {
    // Safety: constraint 为 NonNull 句柄，类型不变量保证指针非空；其指向的
    // Constraint 全部堆分配于 solver_constraints: Vec<Box<Constraint>> 的 Box
    // 内容（push_constraint 以 `&mut *c` 出裸地址，Vec 移动 Box 不影响堆地址），
    // 在 solver 存活期内稳定。此处仅 Copy 读取 scope 字段；该字段由
    // push_constraint 以 NonNull<Scope> 入参经 not_null 构造函数写入，必非空，
    // 故 NonNull::new(..).unwrap() 实不可达 panic 分支（即便触发也是 panic
    // 而非 UB）。
    let constraint_scope = unsafe { (*constraint.as_ptr()).scope };
    // Safety: 同上——constraint 指向 Box 堆上存活的 Constraint，location 为
    // Copy 字段的只读提取，无别名、无悬垂。
    let constraint_location = unsafe { (*constraint.as_ptr()).location };

    let c = self.push_constraint(
      NonNull::new(constraint_scope)
        .expect("Constraint.scope 按 cpp NotNull<Scope> 构造登记，恒非空"),
      constraint_location,
      ConstraintV::Unpack(UnpackConstraint {
        result_pack: dest_types.clone(),
        source_pack: src_types,
      }),
    );

    for t in dest_types {
      // C++ `LUAU_ASSERT(bt)` 后 `bt->replaceOwner(...)`：dest 均为 blocked。
      get_mutable_type::get_mutable::<BlockedType>(t)
        .expect("unpack dest must be blocked")
        .replace_owner(c.as_ptr());
    }

    c
  }
}
