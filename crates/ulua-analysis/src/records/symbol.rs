use core::{
  hash::{Hash, Hasher},
  ptr::NonNull,
};

use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};
#[derive(Debug, Clone)]
pub struct Symbol {
  /// §2：cpp `Symbol::AstLocal* local`（`Symbol(AstName)` 构造置 nullptr，即「此
  /// symbol 指全局而非局部」的互斥可选臂）。收口为 `Option<NonNull<AstLocal>>`：
  /// `None`＝全局/空臂，`Some`＝非空局部回指。`local` 仅作指针身份参与 `eq`/`hash`
  /// 与「是否为局部」判空，唯二的名字读取集中在 `local_ref` chokepoint（见其
  /// `// Safety:` 契约），`ast_name`/`snapshot_scope` 经它取 `&AstLocal`，解引用不外渗。
  pub(crate) local: Option<NonNull<AstLocal>>,
  pub(crate) global: AstName,
}

impl Default for Symbol {
  fn default() -> Self {
    Self {
      local: None,
      global: AstName::new(),
    }
  }
}

impl Symbol {
  /// `local` 保持 cpp `Symbol(AstLocal*)` 的裸指针形参（既有 API，调用方经
  /// arena 句柄 `.as_ptr()` 桥接）；裸→`Option<NonNull>` 的收口集中在构造器内，
  /// 故消费端不再散落可空裸字段语义。
  pub fn from_local(local: *mut AstLocal) -> Self {
    Self {
      local: NonNull::new(local),
      global: AstName::new(),
    }
  }

  /// §2：调用方持有借用（`&AstLocal`）时的构造形——`local` 只作身份，从不经此
  /// 句柄写入，故形参取共享引用而非 `*mut`（旧写法在调用点
  /// `from_ref(r).cast_mut()` 把只读借用洗成可变裸指针，语义上宣称了不存在的
  /// 可变性）。指针臂由 [`Self::from_local`] 承担（arena 句柄 `.as_ptr()` 桥接）。
  #[inline]
  pub fn from_local_ref(local: &AstLocal) -> Self {
    Self {
      local: Some(NonNull::from(local)),
      global: AstName::new(),
    }
  }

  pub fn from_global(global: AstName) -> Self {
    Self {
      local: None,
      global,
    }
  }

  /// `local` 臂的解引用 chokepoint：仅此处把身份句柄物化为 `&AstLocal`。
  pub(crate) fn local_ref(&self) -> Option<&AstLocal> {
    // Safety: `local`（若 Some）借用自与本 Symbol 同生命周期、由 arena 保活的
    // AstLocal（cpp 不变式），Symbol 从不转移/释放其所有权，全程单线程只读；
    // 返回借用不超过 `&self` 寿命。
    self.local.map(|p| unsafe { p.as_ref() })
  }

  #[inline]
  pub fn name(&self) -> &str {
    self.ast_name().as_str().unwrap_or("")
  }
}

impl PartialEq for Symbol {
  fn eq(&self, rhs: &Self) -> bool {
    if self.local.is_some() {
      self.local == rhs.local
    } else if !self.global.is_null() {
      !rhs.global.is_null() && self.global.as_bytes() == rhs.global.as_bytes()
    } else {
      rhs.local.is_none() && rhs.global.is_null()
    }
  }
}

impl Eq for Symbol {}

impl Hash for Symbol {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.local.hash(state);

    if !self.global.is_null() {
      self.global.as_bytes().hash(state);
    }
  }
}

// Safety: `local` 是借用自存活 AST/局部符号 arena 的身份句柄（现 `Option<NonNull>`），
// 仅作身份比较（`eq`/`hash` 只比指针值与 `global` 的字节），只在 `local_ref` chokepoint
// 内只读解引用，从不对它写或释放；移动 `Symbol` 只搬动这些身份值，不转移 pointee 所有权，
// 故只要被指向对象在该符号存活期内有效即可靠。
unsafe impl Send for Symbol {}
// Safety: `local` 只被只读访问作身份比较；`global: AstName` 的指针臂指向全局
// 字符串驻留表中的不可变字节，跨线程共享 `&Symbol` 仅读取这些只读身份/字节，不产生数据竞争。
unsafe impl Sync for Symbol {}
