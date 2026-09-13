use alloc::collections::BTreeMap;

use ulua_common::FFlag;

use crate::{
  enums::normalization_result::NormalizationResult,
  functions::{get_type_alt_j::get_type_id, tyvar_index::tyvar_index},
  records::{
    never_type::NeverType, normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType, normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType, normalizer::Normalizer, type_ids::TypeIds,
    unknown_type::UnknownType,
  },
  type_aliases::error_type::ErrorType,
};

impl Normalizer {
  pub fn union_normals(
    &mut self,
    here: &mut NormalizedType,
    there: &NormalizedType,
    ignore_smaller_tyvars: i32,
  ) -> NormalizationResult {
    self.consume_fuel();

    here.is_cacheable &= there.is_cacheable;

    let mut tops = self.union_of_tops(here.tops, there.tops);
    if !get_type_id::<UnknownType>(tops).is_none()
      && (!get_type_id::<ErrorType>(here.errors).is_none()
        || !get_type_id::<ErrorType>(there.errors).is_none())
    {
      tops = unsafe { (*here.builtin_types).any_type };
    }

    if get_type_id::<NeverType>(tops).is_none() {
      self.clear_normal(here);
      here.tops = tops;
      return NormalizationResult::True;
    }

    for (tyvar, inter_box) in &there.tyvars {
      let index = tyvar_index(*tyvar);
      if index <= ignore_smaller_tyvars {
        continue;
      }

      if !here.tyvars.contains_key(tyvar) {
        let never_type = unsafe { (*here.builtin_types).never_type };
        let mut fresh = NormalizedType {
          builtin_types: here.builtin_types,
          tops: never_type,
          booleans: never_type,
          extern_types: NormalizedExternType {
            extern_types: BTreeMap::new(),
            shape_extensions: TypeIds::new(),
            ordering: Vec::new(),
          },
          errors: never_type,
          nils: never_type,
          numbers: never_type,
          integers: never_type,
          strings: NormalizedStringType::NEVER,
          threads: never_type,
          buffers: never_type,
          tables: TypeIds::new(),
          functions: NormalizedFunctionType {
            is_top: false,
            parts: TypeIds::new(),
          },
          tyvars: BTreeMap::new(),
          is_cacheable: true,
        };
        let res = self.union_normals(&mut fresh, here, index);
        if res != NormalizationResult::True {
          return res;
        }
        here.tyvars.insert(*tyvar, Box::new(fresh));
      }

      if let Some(mut current) = here.tyvars.remove(tyvar) {
        let res = self.union_normals(&mut current, inter_box, index);
        if res != NormalizationResult::True {
          return res;
        }
        here.tyvars.insert(*tyvar, current);
      }
    }

    here.booleans = self.union_of_bools(here.booleans, there.booleans);
    self.union_extern_types_normalized_extern_type_normalized_extern_type(
      &mut here.extern_types,
      &there.extern_types,
    );

    here.errors = if !get_type_id::<NeverType>(there.errors).is_none() {
      here.errors
    } else {
      there.errors
    };
    here.nils = if !get_type_id::<NeverType>(there.nils).is_none() {
      here.nils
    } else {
      there.nils
    };
    here.numbers = if !get_type_id::<NeverType>(there.numbers).is_none() {
      here.numbers
    } else {
      there.numbers
    };
    if FFlag::LuauIntegerType2.get() {
      here.integers = if !get_type_id::<NeverType>(there.integers).is_none() {
        here.integers
      } else {
        there.integers
      };
    }
    self.union_strings(&mut here.strings, &there.strings);
    here.threads = if !get_type_id::<NeverType>(there.threads).is_none() {
      here.threads
    } else {
      there.threads
    };
    here.buffers = if !get_type_id::<NeverType>(there.buffers).is_none() {
      here.buffers
    } else {
      there.buffers
    };
    self.union_functions(&mut here.functions, &there.functions);
    self.union_tables(&mut here.tables, &there.tables);

    NormalizationResult::True
  }
}
