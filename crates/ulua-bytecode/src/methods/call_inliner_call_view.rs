use crate::records::{bc_call_fb::BcCallFB, bc_function::VmConst, call_inliner::CallInliner};

impl<'a> CallInliner<'a> {
  /// cpp `CallInliner::call`（`BcCallFB<VmConst>`）。被内联的 CALLFB 视图每次现取：
  /// `BcCallFB` 持有图的唯一可变借用，不能与 `self.caller` 长期共存，因此在语句
  /// 结束时即释放借用，后续 `self.caller` 的读写不受影响。
  pub(crate) fn call_view(&mut self) -> BcCallFB<'_, VmConst> {
    BcCallFB::from(self.caller, self.call_op)
  }
}
