//! This module defines power64-specific machine instruction types.

pub use crate::ir::condcodes::{ IntCC, FloatCC };
use crate::{settings, CodegenError, CodegenResult};
use crate::machinst::*;
use crate::isa::{CallConv, FunctionAlignment};
use crate::ir::types::{self, F128, F16, F32, F64, I128, I16, I32, I64, I8, I8X16};
use crate::binemit::{Addend, CodeOffset, Reloc};
use std::string::{String, ToString};
pub use crate::ir::{ExternalName, MemFlags, Type};

pub mod regs;
pub use self::regs::*;
pub mod emit;
pub(crate) use self::emit::*;

pub mod unwind;

use crate::isa::power64::abi::Power64MachineDeps;

pub use crate::isa::power64::lower::isle::generated_code::{
    /*AluOPRRI, AluOPRRR, AtomicOP, CsrImmOP, CsrRegOP, FClassResult, FFlagsException, FpuOPRR,
    FpuOPRRR, FpuOPRRRR, LoadOP, */MInst as Inst/*, StoreOP, CSR, FRM,*/
};

impl Inst {
    fn print_with_state(&self, state: &mut EmitState) -> String {
        unimplemented!()
    }
}

impl MachInst for Inst {
    type LabelUse = LabelUse;
    type ABIMachineSpec = Power64MachineDeps;

    fn get_operands(&mut self, collector: &mut impl OperandVisitor) {
        // riscv64_get_operands(self, collector);
        unimplemented!()
    }

    fn is_move(&self) -> Option<(Writable<Reg>, Reg)> {
        unimplemented!()
    }

    fn is_term(&self) -> MachTerminator {
        unimplemented!()
    }

    fn is_trap(&self) -> bool {
        unimplemented!()
    }

    fn is_args(&self) -> bool {
        unimplemented!()
    }

    fn is_included_in_clobbers(&self) -> bool {
        unimplemented!()
    }

    fn is_mem_access(&self) -> bool {
        unimplemented!()
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Inst {
        unimplemented!()
    }

    fn gen_dummy_use(reg: Reg) -> Self {
        unimplemented!()
        // Inst::DummyUse { reg }
    }

    fn rc_for_type(ty: Type) -> CodegenResult<(&'static [RegClass], &'static [Type])> {
        unimplemented!()
    }

    fn canonical_type_for_rc(rc: RegClass) -> Type {
        match rc {
            regalloc2::RegClass::Int => I64,
            regalloc2::RegClass::Float => F64,
            regalloc2::RegClass::Vector => I8X16,
        }
    }

    fn gen_jump(target: MachLabel) -> Inst {
        unimplemented!()
    }

    fn worst_case_size() -> CodeOffset {
        8
    }

    fn ref_type_regclass(_settings: &settings::Flags) -> RegClass {
        RegClass::Int
    }

    fn gen_nop(preferred_size: usize) -> Inst {
        if preferred_size == 0 {
            return Inst::Nop0;
        }
        // We can't give a NOP (or any insn) < 4 bytes.
        assert!(preferred_size >= 4);
        Inst::Nop4
    }

    fn is_safepoint(&self) -> bool {
        unimplemented!()
    }

    fn function_alignment() -> FunctionAlignment {
        FunctionAlignment {
            minimum: 4,
            preferred: 4,
        }
    }

    const TRAP_OPCODE: &'static [u8] = &[0; 4];
}

/// Different forms of label references for different instruction formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelUse {
}

impl MachInstLabelUse for LabelUse {
    const ALIGN: CodeOffset = 4;

    fn max_pos_range(self) -> CodeOffset {
        unimplemented!()
    }

    fn max_neg_range(self) -> CodeOffset {
        unimplemented!()
    }

    fn patch_size(self) -> CodeOffset {
        unimplemented!()
    }

    fn patch(self, buffer: &mut [u8], use_offset: CodeOffset, label_offset: CodeOffset) {
        unimplemented!()
    }

    fn supports_veneer(self) -> bool {
        unimplemented!()
    }

    fn veneer_size(self) -> CodeOffset {
        unimplemented!()
    }

    fn worst_case_veneer_size() -> CodeOffset {
        unimplemented!()
    }

    fn generate_veneer(
        self,
        buffer: &mut [u8],
        veneer_offset: CodeOffset,
    ) -> (CodeOffset, LabelUse) {
        unimplemented!()
    }

    fn from_reloc(reloc: Reloc, addend: Addend) -> Option<LabelUse> {
        unimplemented!()
    }
}
