/// 展开为 `!` 类型表达式（`lua_l_error_l` 抛错不返回）：语句位置照常使用，
/// 亦可直接置于 match 臂/块尾等需要值收敛的表达式位置。
#[macro_export]
macro_rules! luaL_error {
    ($l:expr, $fmt:expr $(, $($arg:expr),+ )? $(,)? ) => {
        $crate::functions::lua_l_error_l::lua_l_error_l(
            $l,
            core::ptr::null(),
            core::format_args!($fmt $(, $($arg),* )?),
        )
    };
}

pub use luaL_error;
