//! `cpp/tests/SharedCodeAllocator.test.cpp` 的 Rust 移植 —— 行为钉：
//! 每个用例的输入→期望输出与 cpp oracle 逐条一致。
//!
//! 放置于 `ulua-unit-test/tests/` 而非 `ulua-code-gen/tests/` 的理由：
//! `SharedAllocation` 用例依赖 `ulua-compiler`（compile）与 `ulua-vm`
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
//! - 自移动赋值 → 先经移动助手掏空、再赋回同一对象（与 cpp `x = std::move(x)`
//!   的观测结果一致），按 cpp 的 `#if defined(__linux__) && defined(__GNUC__)`
//!   条件仅在非 linux 目标验证。
//!
//! unsafe 收口说明：`SharedCodeAllocator` 的 `(指针, 长度)` 入参与
//! `NativeModuleRef` 裸句柄由各安全封装助手（`get_or_insert`、`make_proto`、
//! `module_of` 等）统一物化；合理保留的 `unsafe` 仅在真实 C ABI 边界
//! （`luau_load` / `lua_close` / `create` / `compile_internal`）
//! 与跨 reset 的模块裸句柄投影处。

use core::ptr::null;

use ulua_code_gen::{
  functions::{
    create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32,
    get_native_proto_exec_data_header_native_proto_exec_data::{
      get_native_proto_exec_data_header, get_native_proto_exec_data_header_mut,
    },
  },
  records::{
    code_allocator::CodeAllocator, native_module::NativeModule, native_module_ref::NativeModuleRef,
    shared_code_allocator::SharedCodeAllocator,
  },
  type_aliases::{module_id::ModuleId, native_proto_exec_data_ptr::NativeProtoExecDataPtr},
};

/// 构造以 `first_byte` 开头、其余为 0 的 16 字节 ModuleId（cpp `ModuleId{0x0a}`）。
fn module_id(first_byte: u8) -> ModuleId {
  let mut module_id = [0; 16];
  module_id[0] = first_byte;
  module_id
}

/// cpp `static const size_t blockSize = 1MB; size_t maxTotalSize = 1MB;`。
const K_BLOCK_SIZE: usize = 1024 * 1024;
const K_MAX_TOTAL_SIZE: usize = 1024 * 1024;
/// cpp: `static const uint8_t fakeCode[1] = {0x00};`
const FAKE_CODE: [u8; 1] = [0x00];

/// cpp oracle 的 `(ptr, size)` 形参形态：空切片传 null 配 0。
fn ptr_len(data: &[u8]) -> (*const u8, usize) {
  if data.is_empty() {
    (null(), 0)
  } else {
    (data.as_ptr(), data.len())
  }
}

/// 接线 `code_allocator` 构造 `SharedCodeAllocator`。
///
/// 测试基建契约（对应 cpp `SharedCodeAllocator(&ca)`）：callee 保管裸句柄，
/// `code_allocator` 必须比返回值长寿且不再移动——本文件各用例在同一作用域内
/// 先后声明两者、按引用接线。
fn shared_allocator(code_allocator: &mut CodeAllocator) -> SharedCodeAllocator {
  let mut allocator = SharedCodeAllocator::default();
  allocator.shared_code_allocator_code_allocator(code_allocator as *mut CodeAllocator);
  allocator
}

/// `SharedCodeAllocator::get_or_insert_native_module` 的安全封装
/// （C++ oracle 的 `(ptr, size)` 双缓冲形参在此物化为切片）。
fn get_or_insert(
  allocator: &mut SharedCodeAllocator,
  module_id: &ModuleId,
  native_protos: Vec<NativeProtoExecDataPtr>,
  data: &[u8],
  code: &[u8],
) -> (NativeModuleRef, bool) {
  let (data_ptr, data_len) = ptr_len(data);
  let (code_ptr, code_len) = ptr_len(code);
  // Safety: 两对指针/长度由本帧存活切片物化（或 null 配 0），callee 契约仅按长度喂给
  // CodeAllocator::allocate 读取；native_protos 所有权移入、allocator 独占可变借用。
  unsafe {
    allocator.get_or_insert_native_module(
      module_id,
      native_protos,
      data_ptr,
      data_len,
      code_ptr,
      code_len,
    )
  }
}

/// `SharedCodeAllocator::insert_anonymous_native_module` 的安全封装。
fn insert_anonymous(
  allocator: &mut SharedCodeAllocator,
  native_protos: Vec<NativeProtoExecDataPtr>,
  data: &[u8],
  code: &[u8],
) -> NativeModuleRef {
  let (data_ptr, data_len) = ptr_len(data);
  let (code_ptr, code_len) = ptr_len(code);
  // Safety: 同 `get_or_insert`。
  unsafe {
    allocator.insert_anonymous_native_module(native_protos, data_ptr, data_len, code_ptr, code_len)
  }
}

/// cpp `modRef->` 的等价：非空 `NativeModuleRef` 必指向其引用计数保有的存活模块
/// （`NativeModuleRef` 的类型不变量），以此把解引用收口到本函数一处。
fn module_of(module_ref: &NativeModuleRef) -> &NativeModule {
  let module = module_ref.native_module_ref_get();
  assert!(!module.is_null(), "dereferenced empty NativeModuleRef");
  // Safety: 非空句柄按上述不变量指向存活模块；`&self` 借用排除本帧窗口内的 reset。
  unsafe { &*module }
}

/// 跨 `NativeModuleRef` reset 投影裸句柄（cpp 局部 `NativeModule*` 场景）。
///
/// # Safety
/// `module` 非空，且指向的模块由本测试路径上已完成的显式 `native_module_add_ref`
/// 保活（引用计数覆盖返回引用的全部使用时窗）。
unsafe fn module_from_raw<'a>(module: *const NativeModule) -> &'a NativeModule {
  // Safety: 由本函数 `# Safety` 契约保证。
  unsafe { &*module }
}

/// 按 cpp oracle 布局写 exec data 头部（header 紧邻 u32 数组之前的分配段）。
fn set_proto_bytecode_id(proto: NativeProtoExecDataPtr, bytecode_id: u32) {
  // Safety: proto 为 create_native_proto_exec_data_u32_u32 产出的存活 execdata 块，
  // get_native_proto_exec_data_header_mut 的地址推导落在同一分配内（crate 布局契约）。
  unsafe { (*get_native_proto_exec_data_header_mut(proto.as_ptr())).bytecode_id = bytecode_id };
}

/// 按 cpp 形态构造 exec data：写 header 的 bytecode_id / entry_offset_or_address
/// 并覆写前两个指令字。
fn make_proto(bytecode_id: u32, entry: *const u8, words: [u32; 2]) -> NativeProtoExecDataPtr {
  let proto = create_native_proto_exec_data_u32_u32(words.len() as u32, 0);
  // Safety: proto 为本帧新建的 2 指令字 execdata 块；header 地址推导按 crate 布局
  // 契约落在分配内；两字写入界内（bytecode_instruction_count == words.len()）。
  unsafe {
    let header = &mut *get_native_proto_exec_data_header_mut(proto.as_ptr());
    header.bytecode_id = bytecode_id;
    header.entry_offset_or_address = entry;
    for (index, word) in words.iter().enumerate() {
      proto.as_ptr().add(index).write(*word);
    }
  }
  proto
}

/// exec data 读回视图（供断言，不再让裸指针外泄到用例正文）。
#[derive(Debug, PartialEq, Eq)]
struct ProtoView {
  bytecode_id: u32,
  entry_offset_or_address: usize,
  words: [u32; 2],
}

/// 读回模块交付的 exec data（header + 前两指令字）。
fn inspect_proto(proto: *const u32) -> ProtoView {
  assert!(!proto.is_null(), "dereferenced null native proto");
  // Safety: 契约由调用点保证——proto 来自 NativeModule::try_get_native_proto
  // 且已通过上方判空；模块存活期间该 execdata 分配（header 前置 + ≥2 u32）有效。
  unsafe {
    let header = &*get_native_proto_exec_data_header(proto);
    ProtoView {
      bytecode_id: header.bytecode_id,
      entry_offset_or_address: header.entry_offset_or_address as usize,
      words: [proto.read(), proto.add(1).read()],
    }
  }
}

// cpp TEST_CASE("NativeModuleRefRefcounting")（源 28-251 行）。
#[test]
fn native_module_ref_refcounting() {
  use ulua_code_gen::{
    functions::luau_codegen_supported::luau_codegen_supported,
    records::native_module_ref::NativeModuleRef,
  };

  use crate::{
    CodeAllocator, FAKE_CODE, K_BLOCK_SIZE, K_MAX_TOTAL_SIZE, get_or_insert, module_id, module_of,
    shared_allocator,
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);
  let mut allocator = shared_allocator(&mut code_allocator);

  let refcount = |module_ref: &NativeModuleRef| module_of(module_ref).native_module_get_refcount();

  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );

  let mut mod_ref_a = get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    Vec::new(),
    &[],
    &FAKE_CODE,
  )
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
    get_or_insert(
      &mut allocator,
      &module_id(0x0a),
      Vec::new(),
      &[],
      &FAKE_CODE
    )
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
  let mod_ref_b = get_or_insert(
    &mut allocator,
    &module_id(0x0b),
    Vec::new(),
    &[],
    &FAKE_CODE,
  )
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
  // #else —— 自移动赋值仅在非 linux 目标验证：先掏空、再赋回同一对象，
  // 观测结果与 cpp `x = std::move(x)`（gcc 之外的实现）一致。
  #[cfg(not(target_os = "linux"))]
  {
    let mut mod_ref1 = mod_ref_a.clone();
    let moved = NativeModuleRef::native_module_ref_native_module_ref_mut(&mut mod_ref1);
    mod_ref1.native_module_ref_operator_assign(moved);
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

// cpp TEST_CASE("NativeProtoRefcounting")（源 253-302 行）。
#[test]
fn native_proto_refcounting() {
  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::{
    CodeAllocator, FAKE_CODE, K_BLOCK_SIZE, K_MAX_TOTAL_SIZE,
    create_native_proto_exec_data_u32_u32, get_or_insert, module_from_raw, module_id, module_of,
    set_proto_bytecode_id, shared_allocator,
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);
  let mut allocator = shared_allocator(&mut code_allocator);

  let mut native_protos = Vec::with_capacity(1);
  let native_proto = create_native_proto_exec_data_u32_u32(0, 0);
  set_proto_bytecode_id(native_proto, 0x01);
  native_protos.push(native_proto);

  let mut mod_ref_a = get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    native_protos,
    &[],
    &FAKE_CODE,
  )
  .0;
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(1, module_of(&mod_ref_a).native_module_get_refcount());

  // 验证 addRef 行为：
  module_of(&mod_ref_a).native_module_add_ref();
  assert_eq!(2, module_of(&mod_ref_a).native_module_get_refcount());

  // 验证 addRefs 行为：
  module_of(&mod_ref_a).native_module_add_refs(2);
  assert_eq!(4, module_of(&mod_ref_a).native_module_get_refcount());

  // 撤销两次 addRef 之一：
  module_of(&mod_ref_a).release();
  assert_eq!(3, module_of(&mod_ref_a).native_module_get_refcount());

  module_of(&mod_ref_a).release();
  assert_eq!(2, module_of(&mod_ref_a).native_module_get_refcount());

  // 释放 NativeModuleRef 后，模块应由我们持有的引用继续保活：
  mod_ref_a.native_module_ref_reset();

  mod_ref_a = allocator.try_get_native_module(&module_id(0x0a));
  assert!(!mod_ref_a.native_module_ref_empty());
  assert_eq!(2, module_of(&mod_ref_a).native_module_get_refcount());

  // 若最后一次 release 经由 releaseOwningPointerToInstructionOffsets 语义
  // （此处为裸句柄上的 release，模拟 cpp 跨 reset 持有的 NativeModule*）：
  let raw_module = mod_ref_a.native_module_ref_get();
  mod_ref_a.native_module_ref_reset();
  // Safety: raw_module 为 reset 前取到的非空模块句柄，其存活由上方 addRef 净增的
  // 引用计数保证（此刻 refcount == 2，本行 release 后归 1 再由 try_get 路径收敛）。
  let module = unsafe { module_from_raw(raw_module) };
  module.release();
  assert!(
    allocator
      .try_get_native_module(&module_id(0x0a))
      .native_module_ref_empty()
  );
}

// cpp TEST_CASE("NativeProtoState")（源 304-361 行）。
#[test]
fn native_proto_state() {
  use core::ptr::null;

  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::{
    CodeAllocator, K_BLOCK_SIZE, K_MAX_TOTAL_SIZE, get_or_insert, inspect_proto, make_proto,
    module_id, module_of, shared_allocator,
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);
  let mut allocator = shared_allocator(&mut code_allocator);

  let data = [0_u8; 16];
  let code = [0_u8; 16];

  let native_protos = vec![
    make_proto(1, null::<u8>(), [0, 4]),
    // cpp: reinterpret_cast<const uint8_t*>(0x08)
    make_proto(3, 0x08_usize as *const u8, [8, 12]),
  ];

  let mod_ref_a = get_or_insert(
    &mut allocator,
    &module_id(0x0a),
    native_protos,
    &data,
    &code,
  )
  .0;
  assert!(!mod_ref_a.native_module_ref_empty());

  let module = module_of(&mod_ref_a);
  let module_base_address = module.native_module_get_module_base_address();
  assert!(!module_base_address.is_null());

  let proto1 = module.native_module_try_get_native_proto(1);
  assert!(!proto1.is_null());
  let view1 = inspect_proto(proto1);
  assert_eq!(1, view1.bytecode_id);
  assert_eq!(module_base_address as usize, view1.entry_offset_or_address);
  assert_eq!([0, 4], view1.words);

  let proto3 = module.native_module_try_get_native_proto(3);
  assert!(!proto3.is_null());
  let view3 = inspect_proto(proto3);
  assert_eq!(3, view3.bytecode_id);
  assert_eq!(
    module_base_address as usize + 0x08,
    view3.entry_offset_or_address
  );
  assert_eq!([8, 12], view3.words);

  // 不存在的 native proto 不应被找到：
  assert!(module.native_module_try_get_native_proto(0).is_null());
  assert!(module.native_module_try_get_native_proto(2).is_null());
  assert!(module.native_module_try_get_native_proto(4).is_null());
}

// cpp TEST_CASE("AnonymousModuleLifetime")（源 363-408 行）。
#[test]
fn anonymous_module_lifetime() {
  use core::ptr::null;

  use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;

  use crate::{
    CodeAllocator, K_BLOCK_SIZE, K_MAX_TOTAL_SIZE, insert_anonymous, make_proto, module_from_raw,
    shared_allocator,
  };

  if luau_codegen_supported() == 0 {
    return;
  }

  let mut code_allocator = CodeAllocator::default();
  code_allocator.code_allocator_usize_usize(K_BLOCK_SIZE, K_MAX_TOTAL_SIZE);
  let mut allocator = shared_allocator(&mut code_allocator);

  let data = [0_u8; 8];
  let code = [0_u8; 8];

  let native_protos = vec![make_proto(1, null::<u8>(), [0, 4])];

  let mut mod_ref = insert_anonymous(&mut allocator, native_protos, &data, &code);
  assert!(!mod_ref.native_module_ref_empty());

  let module_ptr = mod_ref.native_module_ref_get();
  // Safety: module_ptr 为 reset 前取到的非空句柄，mod_ref 的引用计数保证其存活；
  // 后续跨 reset 的使用由下方显式 native_module_add_ref 继续保活（cpp 同款场景）。
  let module = unsafe { module_from_raw(module_ptr) };
  assert!(!module.native_module_get_module_base_address().is_null());
  assert!(!module.native_module_try_get_native_proto(1).is_null());
  assert_eq!(1, module.native_module_get_refcount());

  // 获取一个引用（模拟绑定到 Luau VM Proto）：
  module.native_module_add_ref();
  assert_eq!(2, module.native_module_get_refcount());

  // 释放我们的"持有"引用：
  mod_ref.native_module_ref_reset();
  assert_eq!(1, module.native_module_get_refcount());

  // 释放我们新增的引用（模拟 Luau VM Proto 被 GC）：
  module.release();

  // 函数返回、SharedCodeAllocator 析构时，Drop 中的断言会验证
  // 没有遗留的匿名 NativeModule。
}

mod shared_allocation {
  //! cpp TEST_CASE("SharedAllocation")（源 410-459 行）。
  use core::ptr::NonNull;

  use ulua_ast::records::parse_options::ParseOptions;
  use ulua_bytecode::records::bytecode_encoder::NoopEncoder;
  use ulua_code_gen::{
    enums::{code_gen_compilation_result::CodeGenCompilationResult, code_gen_flags::CodeGenFlags},
    functions::{
      compile_internal::compile_internal, create_code_gen_context::create,
      create_shared_code_gen_context_code_gen_context::create_shared_code_gen_context,
      destroy_shared_code_gen_context::destroy_shared_code_gen_context,
      luau_codegen_supported::luau_codegen_supported,
    },
    records::{
      compilation_options::CompilationOptions, compilation_result::CompilationResult,
      compilation_stats::CompilationStats, shared_code_gen_context::SharedCodeGenContext,
    },
    type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  };
  // 本模块已有镜像 cpp codegen `compile` 的本地 `fn compile`，编译器入口改名引入。
  use ulua_compiler::{
    functions::compile::compile as compile_bytecode, records::compile_options::CompileOptions,
  };
  use ulua_vm::{
    functions::{lua_close::lua_close, lua_l_newstate::lua_l_newstate, luau_load::luau_load},
    records::lua_state::LuaState,
  };

  use super::{ModuleId, module_id};

  /// 对应 cpp `std::unique_ptr<LuaState, void (*)(LuaState*)>` 的 RAII 包装。
  struct StateRef(NonNull<LuaState>);

  impl StateRef {
    fn new(state: *mut LuaState) -> Option<Self> {
      NonNull::new(state).map(Self)
    }

    fn as_ptr(&self) -> *mut LuaState {
      self.0.as_ptr()
    }
  }

  impl Drop for StateRef {
    fn drop(&mut self) {
      // Safety: 构造处已判空；self.0 指向本包装独占保有的存活 LuaState，
      // Drop 是至多一次的唯一回收点。
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
      // Safety: 内部 Unique 句柄按 type_aliases 契约指向未销毁的存活上下文。
      unsafe {
        destroy_shared_code_gen_context(self.as_ptr());
      }
    }
  }

  /// `compile_internal` 的出参 → 返回值形态：(编译结果, 统计)。
  fn compile(
    l: *mut LuaState,
    module_id: ModuleId,
    options: &CompilationOptions,
  ) -> (CompilationResult, CompilationStats) {
    let mut stats = CompilationStats::default();
    // Safety: l 为 StateRef 保有的存活 VM；stats 为本帧局部落地槽，
    // 按 callee 出参契约在返回前写完；两者均本用例独占，无并发访问。
    let result = unsafe { compile_internal(&Some(module_id), l, -1, options, &mut stats) };
    (result, stats)
  }

  #[test]
  fn shared_allocation() {
    if luau_codegen_supported() == 0 {
      return;
    }

    let shared_code_gen_context = SharedContextRef(create_shared_code_gen_context());

    let state1 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let state2 = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l1 = state1.as_ptr();
    let l2 = state2.as_ptr();

    // Safety: 两个 l 为上方 StateRef 存活的 VM；context 指向 SharedContextRef 保有的
    // 存活上下文（callee C ABI 契约），单线程顺序接线。
    unsafe {
      create(l1, shared_code_gen_context.as_ptr());
      create(l2, shared_code_gen_context.as_ptr());
    }

    let source = r#"
        function add(x, y) return x + y end
        function sub(x, y) return x - y end
    "#;

    let bytecode = compile_bytecode(
      source,
      &CompileOptions::default(),
      &ParseOptions::default(),
      NoopEncoder,
    );

    // cpp `std::string bytecode` 同形态：本帧拥有的 Vec<u8>，两次 load 共用，
    // 出作用域自动回收，无裸缓冲与手动释放。
    let load_result1 = unsafe { luau_load(l1, "=Functions", &bytecode, 0) };
    let load_result2 = unsafe { luau_load(l2, "=Functions", &bytecode, 0) };

    assert_eq!(0, load_result1);
    assert_eq!(0, load_result2);

    let options = CompilationOptions {
      flags: CodeGenFlags::CodeGenColdFunctions as u32,
      ..Default::default()
    };
    let (code_gen_result1, native_stats1) = compile(l1, module_id(0x01), &options);
    let (code_gen_result2, native_stats2) = compile(l2, module_id(0x01), &options);

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
