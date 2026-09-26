use ulua_ast::records::ast_node::AstNode;

use crate::records::{
  arena_handle::Handle, non_strict_type_checker::NonStrictTypeChecker, stack_pusher::StackPusher,
};

impl NonStrictTypeChecker {
  pub fn push_stack(&mut self, node: *mut AstNode) -> Option<StackPusher> {
    if self.module.is_null() {
      return None;
    }

    // Safety: 上方 `self.module.is_null()` 守卫已确保 `module` 非 null——它是检查器持有、
    // 指向本次遍历期间存活 `Module` 的裸指针。经裸指针解引用得到的 `&Module` 不借用 `self`，
    // 故与下面对 `self.stack` 的可变借用分属不同分配（Module 与 checker 内嵌 Vec 各自独立），
    // 单线程内无真实别名冲突。
    let module = unsafe { &*self.module };
    // module.ast_scopes: DenseHashMap<*const AstNode, *mut Scope>
    // C++ lookup: module->astScopes.find(node)
    module
      .ast_scopes
      .find(&(node as *const AstNode))
      // Safety: `StackPusher::new` 是 `unsafe fn`，契约要求 `stack`/`scope` 有效。`&mut self.stack`
      // 转成的 `*mut Vec<Handle<Scope>>` 是对本 checker 独占栈字段的借用（单线程、此刻无其它存活
      // 借用）；`scope` 经 `Handle::from_ptr` 由 `ast_scopes` 命中值 `*mut Scope` 建立（仅复制
      // 句柄值、不解引用，null 属登记契约违例即 panic），该 Scope 与 Module 同为遍历期间存活的
      // arena 对象。
      .map(|scope_ptr| unsafe {
        StackPusher::new(&mut self.stack, Handle::from_ptr(*scope_ptr))
      })
  }
}
