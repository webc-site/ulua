# ulua-analysis rustify 施工规范（子代理必读）

仓库根：/Users/z/git/db/luaur。本规范把 c2rust 风格代码改写为 idiomatic Rust。
先读 ./.agents/skills/rust_review/SKILL.md 的代码规范。禁用 #[allow]。

## 背景

- 本 crate 是 Luau C++ Analysis 的 Rust 翻译。上游 C++ 在 ./cpp/ 下，可对照同名文件核对语义。
- clippy 报警主体：`not_unsafe_ptr_arg_deref`（公共函数收裸指针并解引用）。
- 访问器层已改造完成（勿改回）：
  - `get_type_id::<T>(ty) -> Option<&'static T>`（原 unsafe 返回 *const T）
  - `get_mutable_type_id::<T>(ty) -> Option<&'static mut T>`
  - `get_type_pack_id::<T>(tp) -> Option<&'static T>`、`get_mutable_type_pack_id::<T>(tp) -> Option<&'static mut T>`
  - `get_singleton_type::<T>(&SingletonType) -> Option<&T>`（原收 *const）
  - `follow_type_id(ty) -> TypeId`（已 safe）
  - `ast_node_try_as::<T>(&AstNode) -> Option<&T>`（下转，替代 ast_node_as+is_null）
  - `as_node::<T>(&Derived) -> &AstNode`（上转，替代 `&(*p).base` 链 / cast 写法）

## 改造模式（按序应用）

### 1. 访问器调用点

```
// 旧
let p = get_type_id::<TableType>(ty);
if !p.is_null() { let ttv = &*p; ... }
// 新
if let Some(ttv) = get_type_id::<TableType>(ty) { ... }

// 旧
if get_type_id::<TableType>(ty).is_null() { return; }
// 新
if get_type_id::<TableType>(ty).is_none() { return; }

// 旧（unsafe 包裹）
let p = unsafe { get_type_id::<T>(ty) };
// 新：去掉 unsafe 块（函数已 safe）
```

`get_mutable`/`get_singleton_type` 同理。注意 `get_singleton_type` 现在收引用：
调用处 `get_singleton_type::<StringSingleton>(ss)` 若 ss 是 *const，先 `unsafe { &*ss }`
或更好：ss 本身来自 `get_type_id::<SingletonType>(ty)` 的 Some(ss) —— 直接传 ss。

### 2. 对象参数引用化

函数签名的 `ctx: *mut XxxContext`、`node: *mut AstNode`、`scope: *mut Scope`、
`block: *mut Block`、`module: *const Module` 等**对象指针**改为 `&mut Xxx` / `&Xxx`：

- 函数体：`(*ctx).field` → `ctx.field`；`&*node` → `node`；`unsafe {}` 块因此缩小的要拆掉
- 调用方（同簇文件）：`f(ptr)` → `f(unsafe { &mut *ptr })` 或若调用方持有引用直接传
- 调用方若也是待改造文件，优先把调用方签名一并引用化（连锁收敛），只到句柄边界为止

### 3. 句柄类型不动

`TypeId`、`TypePackId`、`ConstraintPtr`、`DefId` 等 type_aliases 是 C++ 指针句柄
（`*const Type` 等），**保持原样**。它们存进容器、需要 Copy。解引用必须走访问器：
`(*ty).ty` → `get_type_id::<T>(ty)`；裸解引用集中的地方保持 unsafe 块但写
`// SAFETY:` 注释。

### 4. 泛型下转

```
// 旧
let s = ast_node_as::<AstExprConstantString>(node);   // *mut
if !s.is_null() { let sv = &*s; ... }
// 新（node 已是 &AstNode）
if let Some(sv) = ast_node_try_as::<AstExprConstantString>(node) { ... }
```

### 5. 消除后续噪音

- 改完后跑 `cargo +nightly fmt -p ulua-analysis`
- 产生的 `unused import`、`unnecessary unsafe` 等 warning 一并清理
- 外层已有 unsafe 块时，内层调用 safe 化后的访问器不要再包 `unsafe { }`（unnecessary-unsafe）
- 注释保持中文；新增注释说明 C++ 对照点（文件:行号）

### 6. 基类指针值一致性（map key）

`DenseHashMap<*const AstExpr>` 之类以节点指针为 key：引用化后取 key 用
`&expr.base`（AstExpr 层）或 `as_node(&derived) as *const _`。repr(C) 偏移 0
保证与 C++ 基类指针值相等，勿用整块 cast 引入偏移差。

## 验收（每个子代理自验）

```bash
cargo +nightly check -q -p ulua-analysis --all-features 2>&1 | grep -E '^error' | head
```

本簇文件的错误清零（其他簇文件的错误不算你的责任，但不要引入新错误）。
若调用方文件不在本簇，允许最小适配（传参处 `unsafe { &mut *p }` + SAFETY 注释）。

## 禁止

- #[allow]、#[expect]（除非绝对必要并说明）
- 改变行为逻辑（与 C++ 对照有疑问时保守处理并汇报）
- 动 ulua-ast / ulua-vm 等其他 crate（发现需要时汇报，由主控统一改）
- 使用 git commit
