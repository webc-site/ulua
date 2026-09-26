// native code allocator 模块/原型引用计数用例
// 移植自 `cpp/tests/SharedCodeAllocator.test.cpp`。
//
// 用例侧 unsafe 已收口进 [`native_codegen_api`] 门面；仅 SharedContextRef::drop 保留
// 一处 RAII 边界的 `destroy_shared_code_gen_context`。

use core::ptr::null;

use crate::common::functions::native_codegen_api as nca;

#[test]
fn shared_code_allocator_anonymous_module_lifetime() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  // cpp `SharedCodeAllocator::setCodeAllocator(CodeAllocator*)` 收非拥有裸指针：
  // 交出一个借用即可，`code_allocator` 声明在前保证其存活期覆盖 `allocator`。
  allocator.shared_code_allocator_code_allocator(&mut code_allocator);

  let data = [0u8; 8];
  let code = [0u8; 8];

  let mut native_protos = Vec::with_capacity(1);

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    nca::patch_proto(&native_proto, 1, null::<u8>(), [0, 4]);
    native_protos.push(native_proto);
  }

  let mut mod_ref = nca::insert_anonymous(&mut allocator, native_protos, &data, &code);
  assert!(!mod_ref.native_module_ref_empty());

  let module = mod_ref.native_module_ref_get();
  assert!(!nca::mod_base_address(module).is_null());
  assert!(!nca::mod_try_get_proto(module, 1).is_null());
  assert_eq!(1, nca::mod_refcount(module));

  nca::mod_add_ref(module);
  assert_eq!(2, nca::mod_refcount(module));

  mod_ref.native_module_ref_reset();
  assert_eq!(1, nca::mod_refcount(module));
  nca::mod_release(module);
}

#[test]
fn shared_code_allocator_native_module_ref_refcounting() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::luau_codegen_supported::luau_codegen_supported,
    records::{
      code_allocator::CodeAllocator, native_module_ref::NativeModuleRef,
      shared_code_allocator::SharedCodeAllocator,
    },
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
  const FAKE_CODE: [u8; 1] = [0x00];

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  // cpp `SharedCodeAllocator::setCodeAllocator(CodeAllocator*)` 收非拥有裸指针：
  // 交出一个借用即可，`code_allocator` 声明在前保证其存活期覆盖 `allocator`。
  allocator.shared_code_allocator_code_allocator(&mut code_allocator);

  let refcount = |module_ref: &NativeModuleRef| nca::refcount_of(module_ref);

  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );

  let mut mod_ref_a = nca::get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    Vec::new(),
    &[],
    &FAKE_CODE,
  );
  assert!(!mod_ref_a.native_module_ref_empty());

  assert_eq!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert_eq!(
    nca::get_or_insert(
      &mut allocator,
      &module_id(0x0a),
      Vec::new(),
      &[],
      &FAKE_CODE
    )
    .native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert!(
    allocator
      .try_get_native_module(&module_id(0x0b))
      .native_module_ref_empty()
  );

  let mod_ref_b = nca::get_or_insert(
    &mut allocator,
    &module_id(0x0b),
    Vec::new(),
    &[],
    &FAKE_CODE,
  );
  assert!(!mod_ref_b.native_module_ref_empty());
  assert_ne!(
    mod_ref_b.native_module_ref_get(),
    mod_ref_a.native_module_ref_get()
  );

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = mod_ref_a.clone();
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = NativeModuleRef::default();
    let mod_ref2 = mod_ref1.clone();
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    mod_ref1.native_module_ref_operator_assign(mod_ref_a.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = NativeModuleRef::default();
    let mut mod_ref2 = NativeModuleRef::default();
    mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    mod_ref1.native_module_ref_operator_assign(mod_ref1.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(3, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = NativeModuleRef::default();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    let mut mod_ref2 = NativeModuleRef::default();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert!(mod_ref2.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  #[cfg(not(target_os = "linux"))]
  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mod_ref1_ptr: *mut NativeModuleRef = &mut mod_ref1;
    nca::self_move_assign(mod_ref1_ptr);
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref2.native_module_ref_operator_assign(moved);
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = NativeModuleRef::default();
    mod_ref1.native_module_ref_reset();
    assert!(mod_ref1.native_module_ref_empty());
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    mod_ref1.native_module_ref_reset();
    assert!(mod_ref1.native_module_ref_empty());
    assert_eq!(1, refcount(&mod_ref_a));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  {
    let mut mod_ref1 = mod_ref_a.clone();
    let mut mod_ref2 = mod_ref_b.clone();
    mod_ref1.native_module_ref_swap(&mut mod_ref2);
    assert_eq!(
      mod_ref1.native_module_ref_get(),
      mod_ref_b.native_module_ref_get()
    );
    assert_eq!(
      mod_ref2.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );
    assert_eq!(2, refcount(&mod_ref_a));
    assert_eq!(2, refcount(&mod_ref_b));
  }

  assert_eq!(1, refcount(&mod_ref_a));
  assert_eq!(1, refcount(&mod_ref_b));

  mod_ref_a.native_module_ref_reset();
  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );
}

#[test]
fn shared_code_allocator_native_proto_refcounting() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
  const FAKE_CODE: [u8; 1] = [0x00];

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  // cpp `SharedCodeAllocator::setCodeAllocator(CodeAllocator*)` 收非拥有裸指针：
  // 交出一个借用即可，`code_allocator` 声明在前保证其存活期覆盖 `allocator`。
  allocator.shared_code_allocator_code_allocator(&mut code_allocator);

  let mut native_protos = Vec::with_capacity(1);
  let native_proto = create_native_proto_exec_data_u32_u32(0, 0);
  nca::patch_proto_bytecode_id(&native_proto, 0x01);
  native_protos.push(native_proto);

  let mut mod_ref_a = nca::get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    native_protos,
    &[],
    &FAKE_CODE,
  );
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(1, nca::refcount_of(&mod_ref_a));

  nca::mod_add_ref(mod_ref_a.native_module_ref_get());
  assert_eq!(2, nca::refcount_of(&mod_ref_a));

  nca::mod_add_refs(mod_ref_a.native_module_ref_get(), 2);
  assert_eq!(4, nca::refcount_of(&mod_ref_a));

  nca::mod_release(mod_ref_a.native_module_ref_get());
  assert_eq!(3, nca::refcount_of(&mod_ref_a));

  nca::mod_release(mod_ref_a.native_module_ref_get());
  assert_eq!(2, nca::refcount_of(&mod_ref_a));

  mod_ref_a.native_module_ref_reset();

  mod_ref_a = allocator.try_get_native_module(&module_id(0x0a));
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(2, nca::refcount_of(&mod_ref_a));

  let raw_mod_a = mod_ref_a.native_module_ref_get();

  mod_ref_a.native_module_ref_reset();
  nca::mod_release(raw_mod_a);
  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );
}

#[test]
fn shared_code_allocator_native_proto_state() {
  use alloc::vec::Vec;

  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use crate::common::functions::shared_code_allocator_module_id::shared_code_allocator_module_id as module_id;

  if luau_codegen_supported() == 0 {
    return;
  }

  const K_BLOCK_SIZE: usize = 1024 * 1024;
  const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

  let mut allocator = SharedCodeAllocator::default();
  // cpp `SharedCodeAllocator::setCodeAllocator(CodeAllocator*)` 收非拥有裸指针：
  // 交出一个借用即可，`code_allocator` 声明在前保证其存活期覆盖 `allocator`。
  allocator.shared_code_allocator_code_allocator(&mut code_allocator);

  let data = [0u8; 16];
  let code = [0u8; 16];

  let mut native_protos = Vec::with_capacity(2);

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    nca::patch_proto(&native_proto, 1, null::<u8>(), [0, 4]);
    native_protos.push(native_proto);
  }

  {
    let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
    nca::patch_proto(&native_proto, 3, 0x08usize as *const u8, [8, 12]);
    native_protos.push(native_proto);
  }

  let mod_ref_a = nca::get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    native_protos,
    &data,
    &code,
  );
  assert!(!mod_ref_a.native_module_ref_empty());

  let module = mod_ref_a.native_module_ref_get();
  let module_base_address = nca::mod_base_address(module);
  assert!(!module_base_address.is_null());

  let proto1 = nca::mod_try_get_proto(module, 1);
  assert!(!proto1.is_null());
  assert_eq!(1, nca::proto_bytecode_id(proto1));
  assert_eq!(
    nca::base_add(module_base_address, 0x00),
    nca::proto_entry(proto1)
  );
  assert_eq!(0, nca::proto_word(proto1, 0));
  assert_eq!(4, nca::proto_word(proto1, 1));

  let proto3 = nca::mod_try_get_proto(module, 3);
  assert!(!proto3.is_null());
  assert_eq!(3, nca::proto_bytecode_id(proto3));
  assert_eq!(
    nca::base_add(module_base_address, 0x08),
    nca::proto_entry(proto3)
  );
  assert_eq!(8, nca::proto_word(proto3, 0));
  assert_eq!(12, nca::proto_word(proto3, 1));

  assert!(nca::mod_try_get_proto(module, 0).is_null());
  assert!(nca::mod_try_get_proto(module, 2).is_null());
  assert!(nca::mod_try_get_proto(module, 4).is_null());
}

#[test]
fn shared_code_allocator_shared_allocation() {
  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      create_shared_code_gen_context_code_gen_context::create_shared_code_gen_context,
      destroy_shared_code_gen_context::destroy_shared_code_gen_context,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      compilation_options::CompilationOptions, shared_code_gen_context::SharedCodeGenContext,
    },
    type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  };

  use crate::common::functions::{
    new_state::new_state,
    shared_code_allocator_module_id::shared_code_allocator_module_id as module_id,
  };

  struct SharedContextRef(UniqueSharedCodeGenContext);

  impl SharedContextRef {
    fn as_ptr(&self) -> *mut SharedCodeGenContext {
      self.0.as_ptr()
    }
  }

  impl Drop for SharedContextRef {
    fn drop(&mut self) {
      // Safety: RAII 边界——本 `SharedContextRef` 独占持有 `create_shared_code_gen_context`
      // 交回的上下文，drop 时其仍存活，销毁一次且不双重释放。
      unsafe {
        destroy_shared_code_gen_context(self.as_ptr());
      }
    }
  }

  if luau_codegen_supported() == 0 {
    return;
  }

  let shared_code_gen_context = SharedContextRef(create_shared_code_gen_context());

  let state1 = new_state();
  let state2 = new_state();
  let l1 = state1.as_ptr();
  let l2 = state2.as_ptr();

  nca::codegen_create_shared(l1, shared_code_gen_context.as_ptr());
  nca::codegen_create_shared(l2, shared_code_gen_context.as_ptr());

  let source = r#"
        function add(x, y) return x + y end
        function sub(x, y) return x - y end
    "#;

  let bytes = nca::compile_bytecode(source.as_bytes());

  let load_result1 = nca::load_bytes(l1, "=Functions", &bytes);
  let load_result2 = nca::load_bytes(l2, "=Functions", &bytes);

  assert_eq!(0, load_result1);
  assert_eq!(0, load_result2);

  let module_id = module_id(0x01);

  let options = CompilationOptions {
    flags: CodeGenFlags::CodeGenColdFunctions as u32,
    ..Default::default()
  };
  let (code_gen_result1, native_stats1) =
    nca::compile_native_with_stats(l1, &module_id, -1, &options);
  let (code_gen_result2, native_stats2) =
    nca::compile_native_with_stats(l2, &module_id, -1, &options);

  assert_eq!(CodeGenCompilationResult::Success, code_gen_result1.result);
  assert_eq!(CodeGenCompilationResult::Success, code_gen_result2.result);

  assert_eq!(3, native_stats1.functions_total);
  assert_eq!(3, native_stats2.functions_total);

  assert_eq!(3, native_stats1.functions_compiled);
  assert_eq!(0, native_stats2.functions_compiled);

  assert_eq!(3, native_stats1.functions_bound);
  assert_eq!(3, native_stats2.functions_bound);
}
