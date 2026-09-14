use crate::records::ir_function::IrFunction;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;

impl RemoveDeadStoreState {
    pub fn remove_dead_store_state_remove_dead_store_state(
        function: &mut IrFunction,
        remaining_uses: &mut alloc::vec::Vec<u32>,
    ) -> Self {
        let max_reg = if unsafe { (*function).proto }.is_null() {
            255
        } else {
            unsafe { (*(*function).proto).maxstacksize as i32 }
        };

        Self {
            function: function as *mut IrFunction,
            remaining_uses: remaining_uses as *mut alloc::vec::Vec<u32>,
            info: [crate::records::store_reg_info::StoreRegInfo::default(); 256],
            max_reg,
            has_gco_to_clear: false,
            has_allocations: false,
            non_propagating_store: ulua_common::records::dense_hash_set::DenseHashSet::new(0),
            recorded_vm_exit_syncs: alloc::vec::Vec::new(),
        }
    }
}
