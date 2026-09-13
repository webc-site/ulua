//! Source: `Analysis/include/Luau/Type.h:1077-1087` (hand-ported)
use crate::type_aliases::type_id::TypeId;

/// C++ `template<typename... Ts, typename T> bool is(T&& tv)` — tests whether
/// `tv`'s variant holds any of `Ts...` (`get<Ts>(tv) || ...`). The Rust port
/// resolves variant membership per-type via `match`/get-if traits, so every
/// caller is monomorphized to a concrete predicate; this unbounded template
/// generic over `Type` has no call site.
pub fn is<T>(_tv: TypeId) -> bool {
  unreachable!(
    "C++ Type `is<Ts...>` variant-membership template; Rust callers use monomorphized match/get_if — no call site"
  )
}
