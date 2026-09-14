use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{null_mut, write},
};

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_type::AstType,
    ast_type_function::AstTypeFunction, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_reference::AstTypeReference,
    location::Location,
  },
  type_aliases::ast_argument_name::AstArgumentName,
};

use crate::{
  functions::{
    allocate_string_type_attach::allocate_string_luau_allocator_string_view,
    flatten_type_pack::flatten_type_pack_id, get_type_alt_j::get as get_type,
    get_type_pack::get as get_type_pack,
  },
  records::{
    function_type::FunctionType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    recursion_counter::RecursionCounter, type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl TypeRehydrationVisitor {
  /// 将扁平化后的类型向量逐个 rehydrate，写入 AST 分配器的原始数组。
  ///
  /// # Safety
  /// 调用方须保证 `self.allocator` 指向有效分配器，且满足 C++ 原实现的调用契约。
  unsafe fn rehydrate_types(&mut self, tys: &[TypeId]) -> AstArray<*mut AstType> {
    unsafe {
      let data =
        (*self.allocator).allocate(size_of::<*mut AstType>() * tys.len()) as *mut *mut AstType;
      for (i, &ty) in tys.iter().enumerate() {
        let _counter = RecursionCounter::recursion_counter_i32(&mut self.count);
        *data.add(i) = self.visit_type(ty);
      }
      AstArray {
        data,
        size: tys.len(),
      }
    }
  }

  /// 尾随类型包 rehydrate；无尾随则空指针。
  fn rehydrate_tail(&mut self, tail: Option<TypePackId>) -> *mut AstTypePack {
    tail.map_or_else(null_mut, |tp| self.rehydrate(tp))
  }

  pub fn operator_call_6(&mut self, ftv: &FunctionType) -> *mut AstType {
    let _recursion_counter = unsafe { RecursionCounter::recursion_counter_i32(&mut self.count) };

    if self.has_seen(ftv as *const FunctionType as *const c_void) {
      let cycle_ref = AstTypeReference::new(
        Location::default(),
        None,
        AstName::ast_name_c_char(c"<Cycle>".as_ptr()),
        None,
        Location::default(),
        false,
        AstArray::default(),
      );
      let allocator = unsafe { &mut *self.allocator };
      return allocator.alloc(cycle_ref) as *mut AstType;
    }

    // generics
    let mut generics_array = AstArray::<*mut AstGenericType> {
      data: null_mut(),
      size: 0,
    };
    if !ftv.generics.is_empty() {
      let generics_ptr = unsafe {
        (*self.allocator).allocate(size_of::<*mut AstGenericType>() * ftv.generics.len())
          as *mut *mut AstGenericType
      };
      generics_array.data = generics_ptr;
      let mut num_generics: usize = 0;
      for &gen_id in &ftv.generics {
        if let Some(r#gen) = get_type::<GenericType>(gen_id) {
          // `r#gen.name` is a Rust String (not NUL-terminated); copy it into
          // the AST allocator with a trailing NUL so AstName's borrowed
          // C-string pointer is safe to read (was UB: read past the bytes).
          let name_ptr = allocate_string_luau_allocator_string_view(
            unsafe { &mut *self.allocator },
            &r#gen.name,
          );
          let ast_gen = AstGenericType::new(
            Location::default(),
            AstName::ast_name_c_char(name_ptr),
            null_mut(),
          );
          let allocator = unsafe { &mut *self.allocator };
          unsafe { *generics_ptr.add(num_generics) = allocator.alloc(ast_gen) };
          num_generics += 1;
        }
      }
      generics_array.size = num_generics;
    }

    // generic packs
    let mut generic_packs_array = AstArray::<*mut AstGenericTypePack> {
      data: null_mut(),
      size: 0,
    };
    if !ftv.generic_packs.is_empty() {
      let packs_ptr = unsafe {
        (*self.allocator).allocate(size_of::<*mut AstGenericTypePack>() * ftv.generic_packs.len())
          as *mut *mut AstGenericTypePack
      };
      generic_packs_array.data = packs_ptr;
      let mut num_packs: usize = 0;
      for &pack_id in &ftv.generic_packs {
        if let Some(pack) = get_type_pack::<GenericTypePack>(pack_id) {
          let name_ptr =
            allocate_string_luau_allocator_string_view(unsafe { &mut *self.allocator }, &pack.name);
          let ast_pack = AstGenericTypePack::new(
            Location::default(),
            AstName::ast_name_c_char(name_ptr),
            null_mut(),
          );
          let allocator = unsafe { &mut *self.allocator };
          unsafe { *packs_ptr.add(num_packs) = allocator.alloc(ast_pack) };
          num_packs += 1;
        }
      }
      generic_packs_array.size = num_packs;
    }

    // argument types
    let (arg_vector, arg_tail) = flatten_type_pack_id(ftv.arg_types);
    let arg_types_array = unsafe { self.rehydrate_types(&arg_vector) };
    let arg_tail_annotation = self.rehydrate_tail(arg_tail);

    // argument names
    let mut arg_names_array = AstArray::<Option<AstArgumentName>> {
      data: null_mut(),
      size: 0,
    };
    if !ftv.arg_names.is_empty() {
      let names_ptr = unsafe {
        (*self.allocator).allocate(size_of::<Option<AstArgumentName>>() * ftv.arg_names.len())
          as *mut Option<AstArgumentName>
      };
      for (i, arg_opt) in ftv.arg_names.iter().enumerate() {
        let slot: Option<AstArgumentName> = if let Some(ref arg) = *arg_opt {
          let name_ptr =
            allocate_string_luau_allocator_string_view(unsafe { &mut *self.allocator }, &arg.name);
          let name = AstName::ast_name_c_char(name_ptr);
          Some((name, Location::default()))
        } else {
          None
        };
        unsafe { write(names_ptr.add(i), slot) };
      }
      arg_names_array.data = names_ptr;
      arg_names_array.size = ftv.arg_names.len();
    }

    // return types
    let (ret_vector, ret_tail) = flatten_type_pack_id(ftv.ret_types);
    let return_types_array = unsafe { self.rehydrate_types(&ret_vector) };

    let ret_tail_annotation = self.rehydrate_tail(ret_tail);

    let allocator = unsafe { &mut *self.allocator };
    let return_annotation = allocator.alloc(AstTypePackExplicit::new(
      Location::default(),
      AstTypeList {
        types: return_types_array,
        tail_type: ret_tail_annotation,
      },
    )) as *mut AstTypePack;

    let func_type = AstTypeFunction::ast_type_function_location_ast_array_ast_generic_type_ast_array_ast_generic_type_pack_ast_type_list_ast_array_optional_ast_argument_name_ast_type_pack(
            Location::default(),
            generics_array,
            generic_packs_array,
            AstTypeList {
                types: arg_types_array,
                tail_type: arg_tail_annotation,
            },
            arg_names_array,
            return_annotation,
        );

    allocator.alloc(func_type) as *mut AstType
  }
}
