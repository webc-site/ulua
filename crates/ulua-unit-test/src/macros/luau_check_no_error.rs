#[macro_export]
macro_rules! LUAU_CHECK_NO_ERROR {
    ($result:expr, $type:ty) => {{
        using T = $type;
        const auto& res = ($result);
        if (findError<T>(res))
        {
            dumpErrors(res);
            CHECK_MESSAGE(false, "Expected to find no " #Type " error");
        }
    }};
}

pub use LUAU_CHECK_NO_ERROR;
