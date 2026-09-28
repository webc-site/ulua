use ulua_analysis::records::builtin_types::BuiltinTypes;

use crate::records::fixture::Fixture;

impl Fixture {
  pub fn get_builtins(&mut self) -> &mut BuiltinTypes {
    // `Frontend` owns `builtin_types_` inline and exposes `builtin_types` as a
    // self-referential pointer at it (see `Frontend::new_boxed`). The frontend
    // lives in a `Box`, so its heap address — and with it `builtin_types_` —
    // stays put even when the test `Fixture` is handed back by value
    // (e.g. `SimplifyFixture::default()` moving the whole struct into the
    // caller's slot); the cached `self.builtin_types` no longer dangles on a
    // move.
    //
    // We still route every access through `get_frontend`, which re-derives the
    // cached handle from the wired self-pointer (`builtin_types_handle`) rather
    // than trusting a value cached before frontend construction.
    self.get_frontend();

    // Safety: 上方 get_frontend() 收尾把 self.builtin_types 刷新为 Frontend.builtin_types.as_ptr()（指向 Frontend Box 内 builtin_types_ 字段的稳定非空地址）；&mut 物化独占借用返回给调用方登记，借用期内 fixture 不再重建 Frontend，无第二 &mut。
    unsafe { &mut *self.builtin_types }
  }
}
