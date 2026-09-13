extern crate alloc;

mod runtime_limits_comparison_to_nil_when_normalization_fails_should_not_crash {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_comparison_to_nil_when_normalization_fails_should_not_crash() {
    use alloc::string::String;

    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _fuel = ScopedFastInt::new(&FInt::LuauNormalizerInitialFuel, 3);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        type T = { foo: number } | { bar: number } | { baz: number }
        type U = { oof: number } | { rab: number } | { zab: number }
        type TU = T & U
        local function check(t: TU): boolean
            return t == nil
        end
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod runtime_limits_fusion_normalization_spin {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_fusion_normalization_spin() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Task = unknown
type Constructors = unknown
export type Scope<Constructors = any> = {Task} & Constructors
export type DeriveScopeConstructor = (<S>(Scope<S>) -> Scope<S>)
    & (<S, A>(Scope<S>, A & {}) -> Scope<S & A>)
    & (<S, A, B>(Scope<S>, A & {}, B & {}) -> Scope<S & A & B>)
    & (<S, A, B, C>(Scope<S>, A & {}, B & {}, C & {}) -> Scope<S & A & B & C>)
    & (<S, A, B, C, D>(Scope<S>, A & {}, B & {}, C & {}, D & {}) -> Scope<S & A & B & C & D>)
    & (<S, A, B, C, D, E>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}) -> Scope<S & A & B & C & D & E>)
    & (<S, A, B, C, D, E, F>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}) -> Scope<S & A & B & C & D & E & F>)
    & (<S, A, B, C, D, E, F, G>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}) -> Scope<S & A & B & C & D & E & F & G>)
    & (<S, A, B, C, D, E, F, G, H>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}) -> Scope<S & A & B & C & D & E & F & G & H>)
    & (<S, A, B, C, D, E, F, G, H, I>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}) -> Scope<S & A & B & C & D & E & F & G & H & I>)
    & (<S, A, B, C, D, E, F, G, H, I, J>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J>)
    & (<S, A, B, C, D, E, F, G, H, I, J, K>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}, K & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J & K>)
    & (<S, A, B, C, D, E, F, G, H, I, J, K, l>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}, K & {}, l & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J & K & l>)

local deriveScopeImpl : DeriveScopeConstructor = (nil :: any)

local function innerScope<T>(
    existing: Types.Scope<T>,
    ...: {[unknown]: unknown}
): any
    local new = deriveScopeImpl(existing, ...)
end

    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod runtime_limits_fuzzer_oom_unions {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_fuzzer_oom_unions() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local _ = true,l0
        _ = if _ then _ else _._,if _[_] then nil elseif _ then `` else _._,...
        _ = if _ then _ elseif _ then `` else _.n0,true,...
        _G = if "" then _ else _.n0,_
        _ = if _[_] then _ elseif _ then _ + n0 else _._,32804,...
        _.readstring = _,_
        local l0 = require(module0)
        _ = _,l0,_
        do end
        _.readstring += _
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod runtime_limits_fuzzer_stepwise_normalization_works {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_fuzzer_stepwise_normalization_works() {
    use alloc::string::String;

    use ulua_unit_test::records::builtins_fixture::BuiltinsFixture;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        _ = if _ then {n0=# _,[_]=_,``,[function(l0,l0,l0)
        do end
        end]=_,setmetatable,[l0(_ + _)]=_,} else _(),_,_
        _[_](_,_(coroutine,_,_,nil),_(0,_()),function()
        end)
    "#,
      ),
      None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
  }
}

mod runtime_limits_limit_number_of_dynamically_created_constraints {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_limit_number_of_dynamically_created_constraints() {
    use alloc::string::String;

    use ulua_analysis::records::code_too_complex::CodeTooComplex;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      functions::has_error::has_error,
      records::fixture::Fixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let src = String::from(
      r#"
        type Array<T> = {T}

        type Hello = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>
    "#,
    );

    {
      let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 5);
      let mut fixture = Fixture::fixture_bool(false);
      let result = fixture.check_string_optional_frontend_options(&src, None);
      let dynamic_constraints_created = fixture
        .frontend
        .as_ref()
        .expect("frontend should be initialized")
        .stats
        .dynamic_constraints_created;
      assert!(
        dynamic_constraints_created > 3,
        "dynamic constraints created: {dynamic_constraints_created}, errors: {:?}",
        result.errors
      );
      assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
    }

    {
      let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 1000);
      let mut fixture = Fixture::fixture_bool(false);
      let result = fixture.check_string_optional_frontend_options(&src, None);
      assert!(result.errors.is_empty(), "{:?}", result.errors);
    }

    {
      let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 0);
      let mut fixture = Fixture::fixture_bool(false);
      let result = fixture.check_string_optional_frontend_options(&src, None);
      assert!(result.errors.is_empty(), "{:?}", result.errors);
    }
  }
}

mod runtime_limits_native_stack_guard_prevents_stack_overflows {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_native_stack_guard_prevents_stack_overflows() {
    use alloc::string::String;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    use ulua_analysis::records::internal_compiler_error::InternalCompilerError;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _sff_debug_luau_force_old_solver =
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _sff_luau_use_native_stack_guard =
      ScopedFastFlag::new(&FFlag::LuauUseNativeStackGuard, true);

    let _sff_luau_type_infer_iteration_limit =
      ScopedFastInt::new(&FInt::LuauTypeInferIterationLimit, 0);
    let _sff_luau_stack_guard_threshold =
      ScopedFastInt::new(&FInt::LuauStackGuardThreshold, i32::MAX);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = String::from(
      r#"
            local function l0<A...>()
                for l0=_,_ do
                end
            end

            _ = if _._ then function(l0)
            end elseif _._G then if `` then {n0=_,} else "luauExprConstantSt" elseif _[_][l0] then function()
            end elseif _.n0 then if _[_] then if _ then _ else "aeld" elseif false then 0 else "lead"
            return _.n0
        "#,
    );

    let result = catch_unwind(AssertUnwindSafe(|| {
      let _ = fixture
        .base
        .check_string_optional_frontend_options(&source, None);
    }));

    let payload = result.expect_err("An expected InternalCompilerError was not thrown");
    let ice = payload
      .downcast_ref::<InternalCompilerError>()
      .expect("expected InternalCompilerError panic payload");
    assert!(
      ice.message.starts_with("Stack overflow in "),
      "{}",
      ice.message
    );
  }
}

mod runtime_limits_signal_exerpt {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_signal_exerpt() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();

    let result = fixture.check_string_optional_frontend_options(
      &String::from(
        r#"
        local Signal = {}
        Signal.ClassName = "Signal"
        export type Signal<T...> = typeof(setmetatable(
            {} :: {},
            {} :: typeof({ __index = Signal })
        ))
        function Signal.new<T...>(): Signal<T...>
            return nil :: any
        end

        function Signal.Connect<T...>(self: Signal<T...>)
        end

        function Signal.DisconnectAll<T...>(self: Signal<T...>): ()
            self._handlerListHead = false
        end

        function Signal.Fire<T...>(self: Signal<T...>): ()
            local connection
            rawget(connection, "_signal")
        end

        function Signal.Wait<T...>(self: Signal<T...>)
            connection = self:Connect(function()
                connection:Disconnect()
            end)
        end

        function Signal.Once<T...>(self: Signal<T...>, fn: SignalHandler<T...>): Connection<T...>
            connection = self:Connect(function() end)
        end
    "#,
      ),
      None,
    );

    let _ = result;
  }
}

mod runtime_limits_subtyping_should_cache_pairs_in_seen_set {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_subtyping_should_cache_pairs_in_seen_set() {
    use alloc::string::String;

    use ulua_common::FFlag;
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture, type_aliases::scoped_fast_flag::ScopedFastFlag,
    };

    let _sff_debug_luau_force_old_solver =
      ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let source = String::from(
      r#"
    type DataProxy = any

    type _Transaction = (c: _ApolloCache) -> ()
    type _ApolloCache = {
	    read: <T, TVariables>(self: _ApolloCache, query: Cache_ReadOptions<TVariables, T>) -> T | nil,
	    write: <TResult, TVariables>(self: _ApolloCache, write: Cache_WriteOptions<TResult, TVariables>) -> Reference | nil,
	    diff: <T>(self: _ApolloCache, query: Cache_DiffOptions) -> Cache_DiffResult<T>,
	    watch: (self: _ApolloCache, watch: Cache_WatchOptions<Record<string, any>>) -> (),
	    reset: (self: _ApolloCache) -> Promise<nil>,
	    evict: (self: _ApolloCache, options: Cache_EvictOptions) -> boolean,
	    restore: (self: _ApolloCache, serializedState: TSerialized_) -> _ApolloCache,
	    extract: (self: _ApolloCache, optimistic: boolean?) -> any,
	    removeOptimistic: (self: _ApolloCache, id: string) -> (),
	    batch: (self: _ApolloCache, options: Cache_BatchOptions<_ApolloCache>) -> (),
	    performTransaction: (self: _ApolloCache, transaction: _Transaction, optimisticId: string) -> (),
	    recordOptimisticTransaction: (self: _ApolloCache, transaction: _Transaction, optimisticId: string) -> (),
	    transformDocument: (self: _ApolloCache, document: DocumentNode) -> DocumentNode,
	    identify: (self: _ApolloCache, object: StoreObject | Reference) -> string | nil,
	    gc: (self: _ApolloCache) -> Array<string>,
	    modify: (self: _ApolloCache, options: Cache_ModifyOptions) -> boolean,
	    transformForLink: (self: _ApolloCache, document: DocumentNode) -> DocumentNode,
	    readQuery: <QueryType, TVariables>(
		    self: _ApolloCache,
		    options: Cache_ReadQueryOptions<QueryType, TVariables>,
		    optimistic: boolean?
	    ) -> QueryType | nil,
	    readFragment: <FragmentType, TVariables>(
		    self: _ApolloCache,
		    options: Cache_ReadFragmentOptions<FragmentType, TVariables>,
		    optimistic: boolean?
	    ) -> FragmentType | nil,
	    writeQuery: <TData, TVariables>(self: _ApolloCache, Cache_WriteQueryOptions<TData, TVariables>) -> Reference | nil,
	    writeFragment: <TData, TVariables>(
		    self: _ApolloCache,
		    Cache_WriteFragmentOptions<TData, TVariables>
	    ) -> Reference | nil,
    }

    export type ApolloCache<TSerialized> = {
	    -- something here needed
	    read: <T, TVariables>(self: ApolloCache<TSerialized>, query: Cache_ReadOptions<TVariables, T>) -> T | nil,
	    write: <TResult, TVariables>(
		    self: ApolloCache<TSerialized>,
		    write: Cache_WriteOptions<TResult, TVariables>
	    ) -> Reference | nil,
	    diff: <T>(self: ApolloCache<TSerialized>, query: Cache_DiffOptions) -> Cache_DiffResult<T>,
	    watch: (self: ApolloCache<TSerialized>, watch: Cache_WatchOptions<Record<string, any>>) -> (() -> ()),
	    reset: (self: ApolloCache<TSerialized>) -> Promise<nil>,
	    evict: (self: ApolloCache<TSerialized>, options: Cache_EvictOptions) -> boolean,
	    restore: (self: ApolloCache<TSerialized>, serializedState: TSerialized_) -> _ApolloCache,
	    extract: (self: ApolloCache<TSerialized>, optimistic: boolean?) -> TSerialized,
	    removeOptimistic: (self: ApolloCache<TSerialized>, id: string) -> (),
	    batch: (self: ApolloCache<TSerialized>, options: Cache_BatchOptions<_ApolloCache>) -> (),
	    performTransaction: (self: ApolloCache<TSerialized>, transaction: _Transaction, optimisticId: string) -> (),
	    -- bottom text
	    -- TOP
	    recordOptimisticTransaction: (
		    self: ApolloCache<TSerialized>,
		    transaction: _Transaction,
		    optimisticId: string
	    ) -> (),
	    transformDocument: (self: ApolloCache<TSerialized>, document: DocumentNode) -> DocumentNode,
	    identify: (self: ApolloCache<TSerialized>, object: StoreObject | Reference) -> string | nil,
	    gc: (self: ApolloCache<TSerialized>) -> Array<string>,
	    modify: (self: ApolloCache<TSerialized>, options: Cache_ModifyOptions) -> boolean,
	    -- BOTTOM

	    transformForLink: (self: ApolloCache<TSerialized>, document: DocumentNode) -> DocumentNode,
	    readQuery: <QueryType, TVariables>(
		    self: ApolloCache<TSerialized>,
		    options: Cache_ReadQueryOptions<QueryType, TVariables>,
		    optimistic: boolean?
	    ) -> QueryType | nil,
	    readFragment: <FragmentType, TVariables>(
		    self: ApolloCache<TSerialized>,
		    options: Cache_ReadFragmentOptions<FragmentType, TVariables>,
		    optimistic: boolean?
	    ) -> FragmentType | nil,
	    writeQuery: <TData, TVariables>(
		    self: ApolloCache<TSerialized>,
		    Cache_WriteQueryOptions<TData, TVariables>
	    ) -> Reference | nil,
	    writeFragment: <TData, TVariables>(
		    self: ApolloCache<TSerialized>,
		    Cache_WriteFragmentOptions<TData, TVariables>
	    ) -> Reference | nil,
    }


    export type InMemoryCache = ApolloCache<NormalizedCacheObject> & {
	    performTransaction: (
		    self: InMemoryCache,
		    update: (cache: InMemoryCache) -> ()
	    ) -> ()
    }

    type InMemoryCachePrivate = InMemoryCache & {
	    broadcastWatches: (self: InMemoryCachePrivate) -> (), -- ROBLOX NOTE: protected method
    }

    local InMemoryCache = {}
    InMemoryCache.__index = InMemoryCache

    -- InMemoryCache.batch = nil :: any
    function InMemoryCache:batch()
	    self = self :: InMemoryCachePrivate

	    if self.txCount == 0 then
		    self:broadcastWatches() --  problematic call?
	    end
    end
    "#,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _ = fixture
      .base
      .check_string_optional_frontend_options(&source, None);
  }
}

mod runtime_limits_test_generic_pruning_recursion_limit {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/RuntimeLimits.test.cpp:489:runtime_limits_test_generic_pruning_recursion_limit`
  //! Source: `tests/RuntimeLimits.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/RuntimeLimits.test.cpp
  //! - source_includes:
  //!   - includes -> source_file tests/ClassFixture.h
  //!   - includes -> source_file Analysis/include/Luau/Error.h
  //! - incoming:
  //!   - declares <- source_file tests/RuntimeLimits.test.cpp
  //! - outgoing:
  //!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
  //!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
  //!   - calls -> function get (tests/Fixture.h)
  //!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
  //!   - translates_to -> rust_item runtime_limits_test_generic_pruning_recursion_limit

  #[cfg(test)]
  #[test]
  fn runtime_limits_test_generic_pruning_recursion_limit() {
    use alloc::string::String;

    use ulua_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _steps = ScopedFastInt::new(&FInt::LuauGenericCounterMaxSteps, 1);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
      &String::from(
        r#"
        local function get(scale)
            print(scale.Do.Re.Mi)
        end
    "#,
      ),
      None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
      "<a>({ read Do: { read Re: { read Mi: a } } }) -> ()",
      to_string_type_id(fixture.base.require_type_string(&String::from("get")))
    );
  }
}

mod runtime_limits_typescript_port_of_result_type {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_typescript_port_of_result_type() {
    use alloc::string::String;

    use ulua_analysis::records::code_too_complex::CodeTooComplex;
    use ulua_common::FFlag;
    use ulua_unit_test::{
      functions::has_error::has_error, records::builtins_fixture::BuiltinsFixture,
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = String::from(
      r#"
        --!strict

        -- Big thanks to Dionysusnu by letting us use this code as part of our test suite!
        -- https://github.com/Dionysusnu/rbxts-rust-classes
        -- Licensed under the MPL 2.0: https://raw.githubusercontent.com/Dionysusnu/rbxts-rust-classes/master/LICENSE

        local TS = _G[script]
        local lazyGet = TS.import(script, script.Parent.Parent, "util", "lazyLoad").lazyGet
        local unit = TS.import(script, script.Parent.Parent, "util", "Unit").unit
        local Iterator
        lazyGet("Iterator", function(c)
            Iterator = c
        end)
        local Option
        lazyGet("Option", function(c)
            Option = c
        end)
        local Vec
        lazyGet("Vec", function(c)
            Vec = c
        end)
        local Result
        do
            Result = setmetatable({}, {
                __tostring = function()
                    return "Result"
                end,
            })
            Result.__index = Result
            function Result.new(...)
                local self = setmetatable({}, Result)
                self:constructor(...)
                return self
            end
            function Result:constructor(okValue, errValue)
                self.okValue = okValue
                self.errValue = errValue
            end
            function Result:ok(val)
                return Result.new(val, nil)
            end
            function Result:err(val)
                return Result.new(nil, val)
            end
            function Result:fromCallback(c)
                local _0 = c
                local _1, _2 = pcall(_0)
                local result = _1 and {
                    success = true,
                    value = _2,
                } or {
                    success = false,
                    error = _2,
                }
                return result.success and Result:ok(result.value) or Result:err(Option:wrap(result.error))
            end
            function Result:fromVoidCallback(c)
                local _0 = c
                local _1, _2 = pcall(_0)
                local result = _1 and {
                    success = true,
                    value = _2,
                } or {
                    success = false,
                    error = _2,
                }
                return result.success and Result:ok(unit()) or Result:err(Option:wrap(result.error))
            end
            Result.fromPromise = TS.async(function(self, p)
                local _0, _1 = TS.try(function()
                    return TS.TRY_RETURN, { Result:ok(TS.await(p)) }
                end, function(e)
                    return TS.TRY_RETURN, { Result:err(Option:wrap(e)) }
                end)
                if _0 then
                    return unpack(_1)
                end
            end)
            Result.fromVoidPromise = TS.async(function(self, p)
                local _0, _1 = TS.try(function()
                    TS.await(p)
                    return TS.TRY_RETURN, { Result:ok(unit()) }
                end, function(e)
                    return TS.TRY_RETURN, { Result:err(Option:wrap(e)) }
                end)
                if _0 then
                    return unpack(_1)
                end
            end)
            function Result:isOk()
                return self.okValue ~= nil
            end
            function Result:isErr()
                return self.errValue ~= nil
            end
            function Result:contains(x)
                return self.okValue == x
            end
            function Result:containsErr(x)
                return self.errValue == x
            end
            function Result:okOption()
                return Option:wrap(self.okValue)
            end
            function Result:errOption()
                return Option:wrap(self.errValue)
            end
            function Result:map(func)
                return self:isOk() and Result:ok(func(self.okValue)) or Result:err(self.errValue)
            end
            function Result:mapOr(def, func)
                local _0
                if self:isOk() then
                    _0 = func(self.okValue)
                else
                    _0 = def
                end
                return _0
            end
            function Result:mapOrElse(def, func)
                local _0
                if self:isOk() then
                    _0 = func(self.okValue)
                else
                    _0 = def(self.errValue)
                end
                return _0
            end
            function Result:mapErr(func)
                return self:isErr() and Result:err(func(self.errValue)) or Result:ok(self.okValue)
            end
            Result["and"] = function(self, other)
                return self:isErr() and Result:err(self.errValue) or other
            end
            function Result:andThen(func)
                return self:isErr() and Result:err(self.errValue) or func(self.okValue)
            end
            Result["or"] = function(self, other)
                return self:isOk() and Result:ok(self.okValue) or other
            end
            function Result:orElse(other)
                return self:isOk() and Result:ok(self.okValue) or other(self.errValue)
            end
            function Result:expect(msg)
                if self:isOk() then
                    return self.okValue
                else
                    error(msg)
                end
            end
            function Result:unwrap()
                return self:expect("called `Result.unwrap()` on an `Err` value: " .. tostring(self.errValue))
            end
            function Result:unwrapOr(def)
                local _0
                if self:isOk() then
                    _0 = self.okValue
                else
                    _0 = def
                end
                return _0
            end
            function Result:unwrapOrElse(gen)
                local _0
                if self:isOk() then
                    _0 = self.okValue
                else
                    _0 = gen(self.errValue)
                end
                return _0
            end
            function Result:expectErr(msg)
                if self:isErr() then
                    return self.errValue
                else
                    error(msg)
                end
            end
            function Result:unwrapErr()
                return self:expectErr("called `Result.unwrapErr()` on an `Ok` value: " .. tostring(self.okValue))
            end
            function Result:transpose()
                return self:isOk() and self.okValue:map(function(some)
                    return Result:ok(some)
                end) or Option:some(Result:err(self.errValue))
            end
            function Result:flatten()
                return self:isOk() and Result.new(self.okValue.okValue, self.okValue.errValue) or Result:err(self.errValue)
            end
            function Result:match(ifOk, ifErr)
                local _0
                if self:isOk() then
                    _0 = ifOk(self.okValue)
                else
                    _0 = ifErr(self.errValue)
                end
                return _0
            end
            function Result:asPtr()
                local _0 = (self.okValue)
                if _0 == nil then
                    _0 = (self.errValue)
                end
                return _0
            end
        end
        local resultMeta = Result
        resultMeta.__eq = function(a, b)
            return b:match(function(ok)
                return a:contains(ok)
            end, function(err)
                return a:containsErr(err)
            end)
        end
        resultMeta.__tostring = function(result)
            return result:match(function(ok)
                return "Result.ok(" .. tostring(ok) .. ")"
            end, function(err)
                return "Result.err(" .. tostring(err) .. ")"
            end)
        end
        return {
            Result = Result,
        }
    "#,
    );

    let result = fixture
      .base
      .check_string_optional_frontend_options(&source, None);

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    if FFlag::DebugLuauForceOldSolver.get() {
      assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
    }
  }
}

mod runtime_limits_unification_runs_a_limited_number_of_iterations_before_stopping_subtyping {
  //! Ported from `tests/RuntimeLimits.test.cpp`.

  #[cfg(test)]
  #[test]
  fn runtime_limits_unification_runs_a_limited_number_of_iterations_before_stopping_subtyping() {
    use alloc::string::String;

    use ulua_analysis::records::normalization_too_complex::NormalizationTooComplex;
    use ulua_common::{FFlag, FInt};
    use ulua_unit_test::{
      functions::has_error::has_error,
      records::builtins_fixture::BuiltinsFixture,
      type_aliases::{scoped_fast_flag::ScopedFastFlag, scoped_fast_int::ScopedFastInt},
    };

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _limit = ScopedFastInt::new(&FInt::LuauSubtypingIterationLimit, 100);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function l0<A...>()
            for l0=_,_ do
            end
        end

        _ = if _._ then function(l0)
        end elseif _._G then if `` then {n0=_,} else "luauExprConstantSt" elseif _[_][l0] then function()
        end elseif _.n0 then if _[_] then if _ then _ else "aeld" elseif false then 0 else "lead"
        return _.n0
    "#,
        ),
        None,
    );

    assert!(
      has_error::<NormalizationTooComplex>(&result),
      "{:?}",
      result.errors
    );
  }
}
