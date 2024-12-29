//! Power64 ISA definitions: registers.
//!

use crate::machinst::{Reg, Writable};

use alloc::vec;
use alloc::vec::Vec;

use regalloc2::{PReg, RegClass, VReg};

// first argument of function call
#[inline]
pub fn x0() -> Reg {
    x_reg(0)
}

#[inline]
pub fn x_reg(enc: usize) -> Reg {
    let p_reg = PReg::new(enc, RegClass::Int);
    let v_reg = VReg::new(p_reg.index(), p_reg.class());
    Reg::from(v_reg)
}

#[inline]
pub fn f_reg(enc: usize) -> Reg {
    let p = PReg::new(enc, RegClass::Float);
    let v = VReg::new(p.index(), p.class());
    Reg::from(v)
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

pub fn spilltmp_reg() -> Reg {
    x_reg(16)
}
