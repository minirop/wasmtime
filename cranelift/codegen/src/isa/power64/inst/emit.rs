//! Power64 ISA: binary code emission.

use cranelift_control::ControlPlane;

use crate::ir::{self, types::*};
use crate::isa::power64::inst::*;
use crate::trace;

pub struct EmitInfo {
    shared_flag: settings::Flags,
    isa_flags: super::super::power64_settings::Flags,
}

impl EmitInfo {
    pub(crate) fn new(
        shared_flag: settings::Flags,
        isa_flags: super::super::power64_settings::Flags,
    ) -> Self {
        Self {
            shared_flag,
            isa_flags,
        }
    }
}

/// State carried between emissions of a sequence of instructions.
#[derive(Default, Clone, Debug)]
pub struct EmitState {
}

impl MachInstEmitState<Inst> for EmitState {
    fn new(abi: &Callee<Power64MachineDeps>, ctrl_plane: ControlPlane) -> Self {
        unimplemented!()
    }

    fn pre_safepoint(&mut self, user_stack_map: Option<ir::UserStackMap>) {
        unimplemented!()
    }
    
    fn ctrl_plane_mut(&mut self) -> &mut ControlPlane {
        unimplemented!()
    }
    
    fn take_ctrl_plane(self) -> ControlPlane {
        unimplemented!()
    }
    
    fn frame_layout(&self) -> &FrameLayout {
        unimplemented!()
    }
}

impl MachInstEmit for Inst {
    type State = EmitState;
    type Info = EmitInfo;

    fn emit(&self, sink: &mut MachBuffer<Inst>, emit_info: &Self::Info, state: &mut Self::State) {
    }

    fn pretty_print_inst(&self, state: &mut Self::State) -> String {
        self.print_with_state(state)
    }
}
