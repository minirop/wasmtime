//! Power64 ISA definitions: registers.
//!

use crate::machinst::{Reg, Writable};

use alloc::vec;
use alloc::vec::Vec;

use regalloc2::{PReg, RegClass, VReg};

// first argument of function call
#[inline]
pub fn r0() -> Reg {
    r_reg(0)
}

#[inline]
pub fn r_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Int);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}

#[inline]
pub fn link_reg() -> Reg {
    unimplemented!()
}


#[inline]
pub fn stack_reg() -> Reg {
    unimplemented!()
}

#[inline]
pub fn fp_reg() -> Reg {
    unimplemented!()
}