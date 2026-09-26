use ulua_ast::records::ast_node::AstNode;

use crate::records::{
  arena_handle::Handle, stack_pusher::StackPusher, type_checker_2::TypeChecker2,
};

impl TypeChecker2 {
  // C++ `std::optional<StackPusher> TypeChecker2::pushStack(AstNode* node)`
  // (TypeChecker2.cpp:476): if the node has a recorded scope, push it onto the
  // scope stack for the lifetime of the returned guard; otherwise nullopt.
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    if self.module.is_null() {
      return None;
    }

    // Safety: self.module 是 checker 为当前模块接线的 *mut Module，上方已判空（module.is_null
    // 早返回），非空且指向在整次检查期内存活的模块对象；此处仅取共享借用读取其 ast_scopes 映射。
    let module = unsafe { &*self.module };
    module
      .ast_scopes
      .find(&(node as *const AstNode))
      // Safety: 满足 StackPusher::new 的 pub-unsafe 契约——self.stack 由 checker 独占（&mut
      // self），单线程检查期内无其它借用同时改写；scope 经 `Handle::from_ptr` 由
      // ast_scopes 登记的 `*mut Scope` 值建立（C++ NotNull 表登记语义：null 属契约
      // 违例即确定性 panic），目标 Scope 为 arena 分配的模块作用域，其生命周期覆盖
      // 返回守卫的整个作用域。
      .map(|scope_ptr| unsafe {
        StackPusher::new(&mut self.stack, Handle::from_ptr(*scope_ptr))
      })
  }
}
