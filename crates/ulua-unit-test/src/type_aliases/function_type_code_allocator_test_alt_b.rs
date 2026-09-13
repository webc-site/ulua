pub type FunctionType = extern "C-unwind" fn(i64, Option<extern "C-unwind" fn(i64)>) -> i64;
