// C++ `lua_g_runerror(l, fmt, ...)` — per the project varargs convention the
// format string is a Rust format literal; the cpp C fmt pointer was a dead
// argument and has been dropped from `lua_g_runerror_l`'s signature.
#[macro_export]
macro_rules! lua_g_runerror {
    ($l:expr, $fmt:expr $(, $($arg:expr),+ )? $(,)? ) => {{
        $crate::functions::lua_g_runerror_l::lua_g_runerror_l(
            $l,
            format_args!($fmt $(, $($arg),* )?),
        )
    }};
}

pub use lua_g_runerror;
