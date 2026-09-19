//! `cpp/tests/SharedCodeAllocator.test.cpp` 的 Rust 移植。
//!
//! 放置于 `ulua-unit-test/tests/` 而非 `ulua-code-gen/tests/` 的理由：
//! `SharedAllocation` 用例依赖 `ulua-compiler`（luau_compile）与 `ulua-vm`
//! （luaL_newstate / luau_load），`ulua-unit-test` 已统一依赖上述 crate，
//! 无需为 `ulua-code-gen` 新增 dev-dependencies；且既有移植测试
//! （`code_allocator.rs` 等）均集中于此。
//!
//! 注意：cpp 原文实际包含 5 个 TEST_CASE（NativeModuleRefRefcounting、
//! NativeProtoRefcounting、NativeProtoState、AnonymousModuleLifetime、
//! SharedAllocation），并非任务描述中的 6 个。
//!
//! C++ 的拷贝赋值 / 移动语义在 Rust 中的对应：
//! - 拷贝构造 / 拷贝赋值 → `Clone::clone` + `native_module_ref_operator_assign`
//! - 移动构造 / 移动赋值 → `native_module_ref_native_module_ref_mut`（掏空源引用）
//! - 自移动赋值在 Rust 借用规则下非法，按 cpp 的
//!   `#if defined(__linux__) && defined(__GNUC__)` 条件，仅在非 linux
//!   目标以裸指针绕过借用检查复现 moved-from 语义。

use ulua_code_gen::type_aliases::module_id::ModuleId;

/// 构造以 `first_byte` 开头、其余为 0 的 16 字节 ModuleId（cpp `ModuleId{0x0a}`）。
fn module_id(first_byte: u8) -> ModuleId {
  let mut module_id = [0; 16];
  module_id[0] = first_byte;
  module_id
}

mod native_module_ref_refcounting {
  //! cpp TEST_CASE("NativeModuleRefRefcounting")（源 28-251 行）。
  use ulua_code_gen::{
    functions::luau_codegen_supported::luau_codegen_supported,
    records::{
      code_allocator::CodeAllocator, native_module_ref::NativeModuleRef,
      shared_code_allocator::SharedCodeAllocator,
    },
  };

  use super::module_id;

  #[test]
  fn native_module_ref_refcounting() {
    use core::ptr::null;

    if luau_codegen_supported() == 0 {
      return;
    }

    const K_BLOCK_SIZE: usize = 1024 * 1024;
    const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
    // cpp: static const uint8_t fakeCode[1] = {0x00};
    const FAKE_CODE: [u8; 1] = [0x00];

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    // cpp: modRef->getRefcount()
    let refcount = |module_ref: &NativeModuleRef| unsafe {
      (*module_ref.native_module_ref_get()).native_module_get_refcount()
    };

    assert!(
      allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty()
    );

    let mut mod_ref_a = unsafe {
      allocator.get_or_insert_native_module(
        &module_id(0x0a),
        Vec::new(),
        null(),
        0,
        FAKE_CODE.as_ptr(),
        FAKE_CODE.len(),
      )
    }
    .0;
    assert!(!mod_ref_a.native_module_ref_empty());

    // 再次获取同一模块，应返回同一个模块：
    assert_eq!(
      allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );

    // 再次插入同一模块，应返回已存在的模块：
    assert_eq!(
      unsafe {
        allocator.get_or_insert_native_module(
          &module_id(0x0a),
          Vec::new(),
          null(),
          0,
          FAKE_CODE.as_ptr(),
          FAKE_CODE.len(),
        )
      }
      .0
      .native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );

    // 查找不同模块，不应返回已有模块：
    assert!(
      allocator
        .try_get_native_module(&module_id(0x0b))
        .native_module_ref_empty()
    );

    // （插入第二个模块，供下方验证使用）
    let mod_ref_b = unsafe {
      allocator.get_or_insert_native_module(
        &module_id(0x0b),
        Vec::new(),
        null(),
        0,
        FAKE_CODE.as_ptr(),
        FAKE_CODE.len(),
      )
    }
    .0;
    assert!(!mod_ref_b.native_module_ref_empty());
    assert_ne!(
      mod_ref_b.native_module_ref_get(),
      mod_ref_a.native_module_ref_get()
    );

    // 验证 NativeModuleRef 引用计数：
    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 非空拷贝构造：
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

    // NativeModuleRef 空拷贝构造：
    {
      let mod_ref1 = NativeModuleRef::default();
      let mod_ref2 = mod_ref1.clone();
      assert!(mod_ref1.native_module_ref_empty());
      assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 非空移动构造：
    {
      let mut mod_ref1 = mod_ref_a.clone();
      let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
      // 验证 moved-from 状态：
      assert!(mod_ref1.native_module_ref_empty());
      assert_eq!(
        mod_ref2.native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
      );
      assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 空移动构造：
    {
      let mut mod_ref1 = NativeModuleRef::default();
      let mod_ref2 = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
      assert!(mod_ref1.native_module_ref_empty());
      assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 空 → 非空拷贝赋值：
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

    // NativeModuleRef 空 → 空拷贝赋值：
    {
      let mod_ref1 = NativeModuleRef::default();
      let mut mod_ref2 = NativeModuleRef::default();
      mod_ref2.native_module_ref_operator_assign(mod_ref1.clone());
      assert!(mod_ref1.native_module_ref_empty());
      assert!(mod_ref2.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 自拷贝赋值：
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

    // NativeModuleRef 非空 → 非空拷贝赋值：
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

    // NativeModuleRef 空 → 非空移动赋值：
    {
      let mut mod_ref1 = mod_ref_a.clone();
      let mut mod_ref2 = NativeModuleRef::default();
      let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
      mod_ref2.native_module_ref_operator_assign(moved);
      // 验证 moved-from 状态：
      assert!(mod_ref1.native_module_ref_empty());
      assert_eq!(
        mod_ref2.native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
      );
      assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 空 → 空移动赋值：
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

    // cpp: #if defined(__linux__) && defined(__GNUC__)
    // #else —— 自移动赋值仅在非 linux 目标验证（借用规则下需借裸指针复现）：
    #[cfg(not(target_os = "linux"))]
    {
      let mut mod_ref1 = mod_ref_a.clone();
      let mod_ref1_ptr: *mut NativeModuleRef = &mut mod_ref1;
      unsafe {
        let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut *mod_ref1_ptr);
        (*mod_ref1_ptr).native_module_ref_operator_assign(moved);
      }
      assert_eq!(
        mod_ref1.native_module_ref_get(),
        mod_ref_a.native_module_ref_get()
      );
      assert_eq!(2, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 非空 → 非空移动赋值：
    {
      let mut mod_ref1 = mod_ref_a.clone();
      let mut mod_ref2 = mod_ref_b.clone();
      let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
      mod_ref2.native_module_ref_operator_assign(moved);
      // 验证 moved-from 状态：
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

    // NativeModuleRef 空引用 reset：
    {
      let mut mod_ref1 = NativeModuleRef::default();
      mod_ref1.native_module_ref_reset();
      assert!(mod_ref1.native_module_ref_empty());
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef 非空引用 reset：
    {
      let mut mod_ref1 = mod_ref_a.clone();
      mod_ref1.native_module_ref_reset();
      assert!(mod_ref1.native_module_ref_empty());
      assert_eq!(1, refcount(&mod_ref_a));
    }

    assert_eq!(1, refcount(&mod_ref_a));
    assert_eq!(1, refcount(&mod_ref_b));

    // NativeModuleRef swap：
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

    // 释放模块的最后一个引用，模块应被销毁：
    mod_ref_a.native_module_ref_reset();
    assert!(
      allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty()
    );
  }
}

mod native_proto_refcounting {
  //! cpp TEST_CASE("NativeProtoRefcounting")（源 253-302 行）。
  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use super::module_id;

  #[test]
  fn native_proto_refcounting() {
    use core::ptr::null;

    if luau_codegen_supported() == 0 {
      return;
    }

    const K_BLOCK_SIZE: usize = 1024 * 1024;
    const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
    const FAKE_CODE: [u8; 1] = [0x00];

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    let mut native_protos = Vec::with_capacity(1);
    let native_proto = create_native_proto_exec_data_u32_u32(0, 0);
    unsafe {
      (*get_native_proto_exec_data_header_mut(native_proto.as_ptr())).bytecode_id = 0x01;
    }
    native_protos.push(native_proto);

    let mut mod_ref_a = unsafe {
      allocator.get_or_insert_native_module(
        &module_id(0x0a),
        native_protos,
        null(),
        0,
        FAKE_CODE.as_ptr(),
        FAKE_CODE.len(),
      )
    }
    .0;
    assert!(!mod_ref_a.native_module_ref_empty());
    assert_eq!(1, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    // 验证 addRef 行为：
    unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_add_ref();
    }
    assert_eq!(2, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    // 验证 addRefs 行为：
    unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_add_refs(2);
    }
    assert_eq!(4, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    // 撤销两次 addRef 之一：
    unsafe {
      (*mod_ref_a.native_module_ref_get()).release();
    }
    assert_eq!(3, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    unsafe {
      (*mod_ref_a.native_module_ref_get()).release();
    }
    assert_eq!(2, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    // 释放 NativeModuleRef 后，模块应由我们持有的引用继续保活：
    mod_ref_a.native_module_ref_reset();

    mod_ref_a = allocator.try_get_native_module(&module_id(0x0a));
    assert!(!mod_ref_a.native_module_ref_empty());
    assert_eq!(2, unsafe {
      (*mod_ref_a.native_module_ref_get()).native_module_get_refcount()
    });

    // 若最后一次 release 经由 releaseOwningPointerToInstructionOffsets 语义
    // （此处为裸指针上的 release），模块应被成功销毁：
    let raw_mod_a = mod_ref_a.native_module_ref_get();

    mod_ref_a.native_module_ref_reset();
    unsafe {
      (*raw_mod_a).release();
    }
    assert!(
      allocator
        .try_get_native_module(&module_id(0x0a))
        .native_module_ref_empty()
    );
  }
}

mod native_proto_state {
  //! cpp TEST_CASE("NativeProtoState")（源 304-361 行）。
  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      get_native_proto_exec_data_header_native_proto_exec_data_alt_b::get_native_proto_exec_data_header,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  use super::module_id;

  #[test]
  fn native_proto_state() {
    use core::ptr::null;

    if luau_codegen_supported() == 0 {
      return;
    }

    const K_BLOCK_SIZE: usize = 1024 * 1024;
    const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    let data = [0_u8; 16];
    let code = [0_u8; 16];

    let mut native_protos = Vec::with_capacity(2);

    {
      let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).bytecode_id = 1;
        // cpp: reinterpret_cast<const uint8_t*>(0x00)
        (*header).entry_offset_or_address = null::<u8>();
        *native_proto.as_ptr().add(0) = 0;
        *native_proto.as_ptr().add(1) = 4;
      }
      native_protos.push(native_proto);
    }

    {
      let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).bytecode_id = 3;
        // cpp: reinterpret_cast<const uint8_t*>(0x08)
        (*header).entry_offset_or_address = 0x08_usize as *const u8;
        *native_proto.as_ptr().add(0) = 8;
        *native_proto.as_ptr().add(1) = 12;
      }
      native_protos.push(native_proto);
    }

    let mod_ref_a = unsafe {
      allocator.get_or_insert_native_module(
        &module_id(0x0a),
        native_protos,
        data.as_ptr(),
        data.len(),
        code.as_ptr(),
        code.len(),
      )
    }
    .0;
    assert!(!mod_ref_a.native_module_ref_empty());

    let module = unsafe { &*mod_ref_a.native_module_ref_get() };
    let module_base_address = module.native_module_get_module_base_address();
    assert!(!module_base_address.is_null());

    let proto1 = module.native_module_try_get_native_proto(1);
    assert!(!proto1.is_null());
    unsafe {
      let header = get_native_proto_exec_data_header(proto1);
      assert_eq!(1, (*header).bytecode_id);
      assert_eq!(
        module_base_address.add(0x00),
        (*header).entry_offset_or_address
      );
      assert_eq!(0, *proto1.add(0));
      assert_eq!(4, *proto1.add(1));
    }

    let proto3 = module.native_module_try_get_native_proto(3);
    assert!(!proto3.is_null());
    unsafe {
      let header = get_native_proto_exec_data_header(proto3);
      assert_eq!(3, (*header).bytecode_id);
      assert_eq!(
        module_base_address.add(0x08),
        (*header).entry_offset_or_address
      );
      assert_eq!(8, *proto3.add(0));
      assert_eq!(12, *proto3.add(1));
    }

    // 不存在的 native proto 不应被找到：
    assert!(module.native_module_try_get_native_proto(0).is_null());
    assert!(module.native_module_try_get_native_proto(2).is_null());
    assert!(module.native_module_try_get_native_proto(4).is_null());
  }
}

mod anonymous_module_lifetime {
  //! cpp TEST_CASE("AnonymousModuleLifetime")（源 363-408 行）。
  use ulua_code_gen::{
    functions::{
      create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
      get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{code_allocator::CodeAllocator, shared_code_allocator::SharedCodeAllocator},
  };

  #[test]
  fn anonymous_module_lifetime() {
    use core::ptr::null;

    if luau_codegen_supported() == 0 {
      return;
    }

    const K_BLOCK_SIZE: usize = 1024 * 1024;
    const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;

    let mut code_allocator = CodeAllocator::default();
    code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);

    let mut allocator = SharedCodeAllocator::default();
    allocator.shared_code_allocator_code_allocator(&mut code_allocator as *mut _);

    let data = [0_u8; 8];
    let code = [0_u8; 8];

    let mut native_protos = Vec::with_capacity(1);

    {
      let native_proto = create_native_proto_exec_data_u32_u32(2, 0);
      unsafe {
        let header = get_native_proto_exec_data_header_mut(native_proto.as_ptr());
        (*header).bytecode_id = 1;
        (*header).entry_offset_or_address = null::<u8>();
        *native_proto.as_ptr().add(0) = 0;
        *native_proto.as_ptr().add(1) = 4;
      }
      native_protos.push(native_proto);
    }

    let mut mod_ref = unsafe {
      allocator.insert_anonymous_native_module(
        native_protos,
        data.as_ptr(),
        data.len(),
        code.as_ptr(),
        code.len(),
      )
    };
    assert!(!mod_ref.native_module_ref_empty());

    let module = mod_ref.native_module_ref_get();
    unsafe {
      assert!(!(*module).native_module_get_module_base_address().is_null());
      assert!(!(*module).native_module_try_get_native_proto(1).is_null());
      assert_eq!(1, (*module).native_module_get_refcount());
    }

    // 获取一个引用（模拟绑定到 Luau VM Proto）：
    unsafe {
      (*module).native_module_add_ref();
      assert_eq!(2, (*module).native_module_get_refcount());
    }

    // 释放我们的"持有"引用：
    mod_ref.native_module_ref_reset();
    unsafe {
      assert_eq!(1, (*module).native_module_get_refcount());
    }

    // 释放我们新增的引用（模拟 Luau VM Proto 被 GC）：
    unsafe {
      (*module).release();
    }

    // 函数返回、SharedCodeAllocator 析构时，Drop 中的断言会验证
    // 没有遗留的匿名 NativeModule。
  }
}

mod shared_allocation {
  //! cpp TEST_CASE("SharedAllocation")（源 410-459 行）。
  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      compile_internal::compile_internal, create_code_gen_context_alt_d::create,
      create_shared_code_gen_context_code_gen_context::create_shared_code_gen_context,
      destroy_shared_code_gen_context::destroy_shared_code_gen_context,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      compilation_options::CompilationOptions, compilation_stats::CompilationStats,
      shared_code_gen_context::SharedCodeGenContext,
    },
    type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  };

  #[test]
  fn shared_allocation() {
    use core::{
      ffi::{c_char, c_void},
      ptr::{NonNull, null_mut},
    };

    use ulua_compiler::functions::luau_compile::luau_compile;
    use ulua_vm::{
      functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
      records::lua_state::lua_State,
    };

    use super::module_id;

    unsafe extern "C" {
      fn free(ptr: *mut c_void);
    }

    /// 对应 cpp `std::unique_ptr<lua_State, void (*)(lua_State*)>` 的 RAII 包装。
    struct StateRef(NonNull<lua_State>);

    impl StateRef {
      fn new(state: *mut lua_State) -> Option<Self> {
        NonNull::new(state).map(Self)
      }

      fn as_ptr(&self) -> *mut lua_State {
        self.0.as_ptr()
      }
    }

    impl Drop for StateRef {
      fn drop(&mut self) {
        unsafe {
          lua_close(self.as_ptr());
        }
      }
    }

    /// 对应 cpp `std::unique_ptr<SharedCodeGenContext>` + 显式析构的 RAII 包装。
    struct SharedContextRef(UniqueSharedCodeGenContext);

    impl SharedContextRef {
      fn as_ptr(&self) -> *mut SharedCodeGenContext {
        self.0.as_ptr()
      }
    }

    impl Drop for SharedContextRef {
      fn drop(&mut self) {
        unsafe {
          destroy_shared_code_gen_context(self.as_ptr());
        }
      }
    }

    if luau_codegen_supported() == 0 {
      return;
    }

    let shared_code_gen_context = SharedContextRef(create_shared_code_gen_context());

    let state1 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let state2 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l1 = state1.as_ptr();
    let l2 = state2.as_ptr();

    unsafe {
      create(l1, shared_code_gen_context.as_ptr());
      create(l2, shared_code_gen_context.as_ptr());
    }

    let source = r#"
        function add(x, y) return x + y end
        function sub(x, y) return x - y end
    "#;

    let mut bytecode_size = 0_usize;
    let bytecode = unsafe {
      luau_compile(
        source.as_ptr() as *const c_char,
        source.len(),
        null_mut(),
        &mut bytecode_size,
      )
    };
    assert!(!bytecode.is_null());

    unsafe {
      let load_result1 = luau_load(l1, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
      let load_result2 = luau_load(l2, c"=Functions".as_ptr(), bytecode, bytecode_size, 0);
      // cpp: bytecode.reset()
      free(bytecode as *mut c_void);

      assert_eq!(0, load_result1);
      assert_eq!(0, load_result2);
    }

    let module_id = module_id(0x01);

    let options = CompilationOptions {
      flags: CodeGenFlags::CodeGenColdFunctions as u32,
      ..Default::default()
    };
    let mut native_stats1 = CompilationStats::default();
    let mut native_stats2 = CompilationStats::default();
    let code_gen_result1 = unsafe {
      compile_internal(
        &Some(module_id),
        l1,
        -1,
        &options,
        &mut native_stats1 as *mut CompilationStats,
      )
    };
    let code_gen_result2 = unsafe {
      compile_internal(
        &Some(module_id),
        l2,
        -1,
        &options,
        &mut native_stats2 as *mut CompilationStats,
      )
    };

    assert_eq!(CodeGenCompilationResult::Success, code_gen_result1.result);
    assert_eq!(CodeGenCompilationResult::Success, code_gen_result2.result);

    // 两次编译都应识别出全部三个函数：
    assert_eq!(3, native_stats1.functions_total);
    assert_eq!(3, native_stats2.functions_total);

    // 三个函数应只在第一次被编译：
    assert_eq!(3, native_stats1.functions_compiled);
    assert_eq!(0, native_stats2.functions_compiled);

    // 两次都应绑定全部三个函数：
    assert_eq!(3, native_stats1.functions_bound);
    assert_eq!(3, native_stats2.functions_bound);
  }
}
