//! 「仅携带一条 message 的错误/信息记录」单点：`GenericError`/`SyntaxError`/
//! `InternalError`/`RuntimeError`/`UserDefinedTypeFunctionError`/
//! `ExtraInformation` 六个记录同形（`String` 字段 + `new` + `message` 访问器，
//! 仅类型名不同），由本宏逐一展开出同一定义。

/// 生成「只带一条 message 的记录」：`pub(crate) message` 字段、`const fn new`
/// 构造器与 `message()` 访问器。展开点须已导入 `alloc::string::String`。
macro_rules! message_error {
  ($name:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $name {
      pub(crate) message: String,
    }

    impl $name {
      pub const fn new(message: String) -> Self {
        Self { message }
      }

      pub fn message(&self) -> &str {
        &self.message
      }
    }
  };
}

pub(crate) use message_error;
