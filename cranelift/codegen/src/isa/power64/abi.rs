//! Implementation of a standard Power64 ABI.

use crate::ir;
use crate::ir::types;
use crate::ir::types::*;
use crate::ir::MemFlags;
use crate::ir::{dynamic_to_fixed, ExternalName, LibCall, Signature};
use crate::isa;
use crate::isa::power64::{inst::*, settings as power64_settings, Power64Backend};
use crate::isa::unwind::UnwindInst;
use crate::isa::winch;
use crate::machinst::*;
use crate::settings;
use crate::CodegenResult;
use alloc::boxed::Box;
use alloc::vec::Vec;
use regalloc2::{MachineEnv, PReg, PRegSet};
use smallvec::{smallvec, SmallVec};
use std::borrow::ToOwned;
use std::sync::OnceLock;
use crate::isa::power64::settings::Flags as PowerFlags;

/// Support for the Power64 ABI from the callee side (within a function body).
pub(crate) type Power64Callee = Callee<Power64MachineDeps>;

/// Support for the Power64 ABI from the caller side (at a callsite).
pub(crate) type Power64ABICallSite = CallSite<Power64MachineDeps>;

/// Power64-specific ABI behavior. This struct just serves as an implementation
/// point for the trait; it is never actually instantiated.
pub struct Power64MachineDeps;

impl IsaFlags for PowerFlags {}
impl PowerFlags {
}

impl ABIMachineSpec for Power64MachineDeps {
    type I = Inst;
    type F = PowerFlags;

    const STACK_ARG_RET_SIZE_LIMIT: u32 = 128 * 1024 * 1024;

    fn word_bits() -> u32 {
        64
    }

    fn stack_align(_call_conv: isa::CallConv) -> u32 {
        16
    }

    fn compute_arg_locs(
        call_conv: isa::CallConv,
        flags: &settings::Flags,
        params: &[ir::AbiParam],
        args_or_rets: ArgsOrRets,
        add_ret_area_ptr: bool,
        mut args: ArgsAccumulator,
    ) -> CodegenResult<(u32, Option<usize>)> {
        assert_ne!(
            call_conv,
            isa::CallConv::Winch,
            "power64 does not support the 'winch' calling convention yet"
        );

        // All registers that can be used as parameters or rets.
        // both start and end are included.
        let (x_start, x_end, f_start, f_end) = match args_or_rets {
            ArgsOrRets::Args => (10, 17, 10, 17),
            ArgsOrRets::Rets => (10, 11, 10, 11),
        };
        let mut next_x_reg = x_start;
        let mut next_f_reg = f_start;
        // Stack space.
        let mut next_stack: u32 = 0;

        let ret_area_ptr = if add_ret_area_ptr {
            assert!(ArgsOrRets::Args == args_or_rets);
            next_x_reg += 1;
            Some(ABIArg::reg(
                x_reg(x_start).to_real_reg().unwrap(),
                I64,
                ir::ArgumentExtension::None,
                ir::ArgumentPurpose::Normal,
            ))
        } else {
            None
        };

        for param in params {
            if let ir::ArgumentPurpose::StructArgument(_) = param.purpose {
                panic!(
                    "StructArgument parameters are not supported on riscv64. \
                    Use regular pointer arguments instead."
                );
            }

            // Find regclass(es) of the register(s) used to store a value of this type.
            let (rcs, reg_tys) = Inst::rc_for_type(param.value_type)?;
            let mut slots = ABIArgSlotVec::new();
            for (rc, reg_ty) in rcs.iter().zip(reg_tys.iter()) {
                let next_reg = if (next_x_reg <= x_end) && *rc == RegClass::Int {
                    let x = Some(x_reg(next_x_reg));
                    next_x_reg += 1;
                    x
                } else if (next_f_reg <= f_end) && *rc == RegClass::Float {
                    let x = Some(f_reg(next_f_reg));
                    next_f_reg += 1;
                    x
                } else {
                    None
                };
                if let Some(reg) = next_reg {
                    slots.push(ABIArgSlot::Reg {
                        reg: reg.to_real_reg().unwrap(),
                        ty: *reg_ty,
                        extension: param.extension,
                    });
                } else {
                    if args_or_rets == ArgsOrRets::Rets && !flags.enable_multi_ret_implicit_sret() {
                        return Err(crate::CodegenError::Unsupported(
                            "Too many return values to fit in registers. \
                            Use a StructReturn argument instead. (#9510)"
                                .to_owned(),
                        ));
                    }

                    // Compute size and 16-byte stack alignment happens
                    // separately after all args.
                    let size = reg_ty.bits() / 8;
                    let size = std::cmp::max(size, 8);
                    // Align.
                    debug_assert!(size.is_power_of_two());
                    next_stack = align_to(next_stack, size);
                    slots.push(ABIArgSlot::Stack {
                        offset: next_stack as i64,
                        ty: *reg_ty,
                        extension: param.extension,
                    });
                    next_stack += size;
                }
            }
            args.push(ABIArg::Slots {
                slots,
                purpose: param.purpose,
            });
        }
        let pos = if let Some(ret_area_ptr) = ret_area_ptr {
            args.push_non_formal(ret_area_ptr);
            Some(args.args().len() - 1)
        } else {
            None
        };

        next_stack = align_to(next_stack, Self::stack_align(call_conv));

        Ok((next_stack, pos))
    }

    fn gen_load_stack(mem: StackAMode, into_reg: Writable<Reg>, ty: Type) -> Inst {
        // Inst::gen_load(into_reg, mem.into(), ty, MemFlags::trusted())
        unimplemented!()
    }

    fn gen_store_stack(mem: StackAMode, from_reg: Reg, ty: Type) -> Inst {
        // Inst::gen_store(mem.into(), from_reg, ty, MemFlags::trusted())
        unimplemented!()
    }

    fn gen_move(to_reg: Writable<Reg>, from_reg: Reg, ty: Type) -> Inst {
        // Inst::gen_move(to_reg, from_reg, ty)
        unimplemented!()
    }

    fn gen_extend(
        to_reg: Writable<Reg>,
        from_reg: Reg,
        signed: bool,
        from_bits: u8,
        to_bits: u8,
    ) -> Inst {
        unimplemented!()
    }

    fn gen_args(args: Vec<ArgPair>) -> Inst {
        unimplemented!()
    }

    fn gen_rets(rets: Vec<RetPair>) -> Inst {
        unimplemented!()
    }

    fn gen_add_imm(
        _call_conv: isa::CallConv,
        into_reg: Writable<Reg>,
        from_reg: Reg,
        imm: u32,
    ) -> SmallInstVec<Inst> {
        unimplemented!()
    }

    fn gen_stack_lower_bound_trap(limit_reg: Reg) -> SmallInstVec<Inst> {
        unimplemented!()
    }

    fn gen_get_stack_addr(mem: StackAMode, into_reg: Writable<Reg>) -> Inst {
        unimplemented!()
    }

    fn gen_load_base_offset(into_reg: Writable<Reg>, base: Reg, offset: i32, ty: Type) -> Inst {
        unimplemented!()
    }

    fn gen_store_base_offset(base: Reg, offset: i32, from_reg: Reg, ty: Type) -> Inst {
        unimplemented!()
    }

    fn gen_sp_reg_adjust(amount: i32) -> SmallInstVec<Inst> {
        unimplemented!()
    }


    fn compute_frame_layout(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        _sig: &Signature,
        regs: &[Writable<RealReg>],
        is_leaf: bool,
        incoming_args_size: u32,
        tail_args_size: u32,
        fixed_frame_storage_size: u32,
        outgoing_args_size: u32,
    ) -> FrameLayout {
        unimplemented!()
    }

    fn gen_prologue_frame_setup(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        _isa_flags: &PowerFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        unimplemented!()
    }

    fn gen_epilogue_frame_restore(
        call_conv: isa::CallConv,
        _flags: &settings::Flags,
        _isa_flags: &PowerFlags,
        frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        unimplemented!()
    }

    fn gen_return(
        _call_conv: isa::CallConv,
        _isa_flags: &PowerFlags,
        _frame_layout: &FrameLayout,
    ) -> SmallInstVec<Inst> {
        unimplemented!()
    }

    fn gen_probestack(insts: &mut SmallInstVec<Self::I>, frame_size: u32) {
        unimplemented!()
    }

    fn gen_inline_probestack(
        insts: &mut SmallInstVec<Self::I>,
        _call_conv: isa::CallConv,
        frame_size: u32,
        guard_size: u32,
    ) {
        unimplemented!()
    }

    fn gen_clobber_save(
        _call_conv: isa::CallConv,
        flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        unimplemented!()
    }

    fn gen_clobber_restore(
        _call_conv: isa::CallConv,
        _flags: &settings::Flags,
        frame_layout: &FrameLayout,
    ) -> SmallVec<[Inst; 16]> {
        unimplemented!()
    }

    fn gen_call(dest: &CallDest, tmp: Writable<Reg>, info: CallInfo<()>) -> SmallVec<[Self::I; 2]> {
        unimplemented!()
    }

    fn gen_memcpy<F: FnMut(Type) -> Writable<Reg>>(
        call_conv: isa::CallConv,
        dst: Reg,
        src: Reg,
        size: usize,
        mut alloc_tmp: F,
    ) -> SmallVec<[Self::I; 8]> {
        unimplemented!()
    }

    fn get_number_of_spillslots_for_value(
        rc: RegClass,
        _target_vector_bytes: u32,
        isa_flags: &PowerFlags,
    ) -> u32 {
        unimplemented!()
    }

    fn get_machine_env(_flags: &settings::Flags, _call_conv: isa::CallConv) -> &MachineEnv {
        unimplemented!()
    }

    fn get_regs_clobbered_by_call(_call_conv_of_callee: isa::CallConv) -> PRegSet {
        unimplemented!()
    }

    fn get_ext_mode(
        _call_conv: isa::CallConv,
        specified: ir::ArgumentExtension,
    ) -> ir::ArgumentExtension {
        unimplemented!()
    }

    fn get_stacklimit_reg(_call_conv: isa::CallConv) -> Reg {
        spilltmp_reg()
    }
}

impl Power64ABICallSite {
    pub fn emit_return_call(
        mut self,
        ctx: &mut Lower<Inst>,
        args: isle::ValueSlice,
        _backend: &Power64Backend,
    ) {
        unimplemented!()
    }
}
