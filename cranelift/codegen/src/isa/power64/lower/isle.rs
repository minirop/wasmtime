//! ISLE integration glue code for power64 lowering.

// Pull in the ISLE generated code.
#[allow(unused)]
pub mod generated_code;
use generated_code::MInst;

// Types that the generated ISLE code uses via `use super::*`.
//use self::generated_code::{FpuOPWidth, VecAluOpRR, VecLmul};
use crate::isa;
use crate::isa::power64::abi::Power64ABICallSite;
use crate::isa::power64::Power64Backend;
use crate::machinst::Reg;
use crate::machinst::{isle::*, CallInfo, MachInst};
use crate::machinst::{VCodeConstant, VCodeConstantData};
use crate::{
    ir::{
        immediates::*, types::*, AtomicRmwOp, BlockCall, ExternalName, Inst, InstructionData,
        MemFlags, Opcode, TrapCode, Value, ValueList,
    },
    isa::power64::inst::*,
    machinst::{ArgPair, InstOutput, IsTailCall},
};
use regalloc2::PReg;
use std::boxed::Box;
use std::vec::Vec;

/// The main entry point for lowering with ISLE.
pub(crate) fn lower(
    lower_ctx: &mut Lower<MInst>,
    backend: &Power64Backend,
    inst: Inst,
) -> Option<InstOutput> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower(&mut isle_ctx, inst)
}

/// The main entry point for branch lowering with ISLE.
pub(crate) fn lower_branch(
    lower_ctx: &mut Lower<MInst>,
    backend: &Power64Backend,
    branch: Inst,
    targets: &[MachLabel],
) -> Option<()> {
    // TODO: reuse the ISLE context across lowerings so we can reuse its
    // internal heap allocations.
    let mut isle_ctx = IsleContext { lower_ctx, backend };
    generated_code::constructor_lower_branch(&mut isle_ctx, branch, targets)
}

impl generated_code::Context for IsleContext<'_, '_, MInst, Power64Backend> {
    isle_lower_prelude_methods!();
    isle_prelude_caller_methods!(Power64ABICallSite);
    
    #[inline]
    fn emit(&mut self, arg0: &MInst) -> Unit {
        self.lower_ctx.emit(arg0.clone());
    }
}
