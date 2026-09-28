use ulua_analysis::{
  functions::find_expected_type_at_position::find_expected_type_at_position,
  records::module::Module, type_aliases::type_id::TypeId,
};
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn find_expected_type_at_position(&mut self, position: Position) -> Option<TypeId> {
    let module: *mut Module = self.get_main_module(false);
    let source_module = self
      .get_main_source_module()
      .expect("findExpectedTypeAtPosition: main source module must exist after check");
    // Safety: module 为 check 流程后 resolver 容器保有的存活 Module，&* 物化
    // 只读借用透传查询；source_module 为 Handle 交付的共享只读借用
    //（契约见 arena_handle 模块）。借用随调用结束，不逃逸。
    unsafe { find_expected_type_at_position(&*module, source_module.get(), position) }
  }
}
