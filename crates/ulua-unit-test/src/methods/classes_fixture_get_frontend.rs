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
  pub fn get_frontend(&mut self) -> &mut Frontend {
    let already_initialized = self.base.frontend.is_some();
    let frontend_ptr = self.base.get_frontend() as *mut Frontend;

    if !already_initialized {
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

      unsafe {
        unfreeze((*frontend_ptr).globals.global_types_mut());

        let target_scope = (*frontend_ptr).globals.global_scope();
        let globals_ptr = &mut (*frontend_ptr).globals as *mut _;
        let result = (*frontend_ptr).load_definition_file(
          &mut *globals_ptr,
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

        let require_ty = get_global_binding(&mut (*frontend_ptr).globals, "require");
        attach_require_magic(require_ty);
      }

      self.base.register_test_types();

      unsafe {
        freeze((*frontend_ptr).globals.global_types_mut());
      }
    }

    unsafe { &mut *frontend_ptr }
  }
}
