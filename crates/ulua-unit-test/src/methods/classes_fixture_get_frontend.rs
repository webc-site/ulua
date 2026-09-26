use alloc::string::String;

use ulua_analysis::{
  functions::{
    attach_require_magic::attach_require_magic, freeze::freeze,
    get_global_binding::get_global_binding, unfreeze::unfreeze,
  },
  records::frontend::Frontend,
};

use crate::records::classes_fixture::ClassesFixture;

impl ClassesFixture {
  /// C++ `ClassesFixture::getFrontend`（`TypeInfer.classes.test.cpp:34-52`）：首次调用时
  /// 装载 class 相关 definition、给 `require` 挂 magic，再 `registerTestTypes()`、重冻。
  ///
  /// 别名形态：cpp 里 `f.loadDefinitionFile(f.globals, f.globals.globalScope, ..)` 在同一个
  /// `Frontend&` 上把自身字段借出去。Rust 侧 `load_definition_file` 已改目标表选择器闭包
  /// （两阶段拆分，字段借用只在检查完成后的第二阶段物化），整段回到普通借用：
  /// `&mut Frontend` 一借到底，`target_scope` 是 `ScopePtr`（Arc 克隆，不借生命周期），
  /// `register_test_types()` 之后重新取借用做 `freeze`。
  pub fn get_frontend(&mut self) -> &mut Frontend {
    if self.base.frontend.is_some() {
      return self.base.get_frontend();
    }

    let definitions = String::from(
      r#"
@checked declare function require(target: any): any
declare function sqrt(n: number): number
declare function tostring<T>(value: T): string

declare class: {
    isinstance: @checked (o: unknown, c: class) -> boolean,
    classof: @checked (o: unknown) -> class?
}
"#,
    );

    let frontend = self.base.get_frontend();
    unfreeze(frontend.globals.global_types_mut());

    let target_scope = frontend.globals.global_scope();
    let result = frontend.load_definition_file(
      |frontend| &mut frontend.globals,
      target_scope,
      &definitions,
      String::from("@test"),
      false,
      false,
    );
    assert!(
      result.success,
      "ClassesFixture: unable to load definition file: {:?}",
      result.module.as_ref().map(|module| &module.errors)
    );

    let require_ty = get_global_binding(&mut frontend.globals, "require");
    attach_require_magic(require_ty);

    self.base.register_test_types();

    let frontend = self.base.get_frontend();
    freeze(frontend.globals.global_types_mut());
    frontend
  }
}
