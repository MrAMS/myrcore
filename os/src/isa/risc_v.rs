use super::*;

// RISC-V SBI
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

use riscv::register::time;

/// RISC-V ISA
pub struct ISA;

impl ISAMethod for ISA {
    type WordType = u32;
    fn set_trap_entry(entry: Self::WordType){
        unsafe {
            stvec::write(entry as usize, TrapMode::Direct);
        }
    }
    fn get_trap_cause() -> (TrapCause, Self::WordType) {
        let scause = scause::read(); // get trap cause
        // The stval register can optionally also be used to return the faulting instruction bits on an illegal instruction exception
        let stval = stval::read() as Self::WordType;
        match scause.cause() {
            Trap::Exception(Exception::UserEnvCall) => {
                return (TrapCause::UserEnvCall, stval);
            }
            Trap::Exception(Exception::StoreFault) => {
                return (TrapCause::StoreFault, stval);
            }
            Trap::Exception(Exception::StorePageFault) => {
                return (TrapCause::StorePageFault, stval);
            }
            Trap::Exception(Exception::IllegalInstruction) => {
                return (TrapCause::IllegalInstruction, stval);
            }
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                return (TrapCause::SupervisorTimer, stval);
            }
            _ => {
                panic!(
                    "Unsupported trap {:?}",
                    scause.cause(),
                );
            }
        }
    }

    fn abi_gpr_id_ret() -> Self::WordType {
        10
    }
    fn int_timer_enable() {
        unsafe {
            sie::set_stimer();
        }
    }
    fn get_timer_val() -> Self::WordType {
        // TODO type    
        time::read() as Self::WordType
    }
}


use riscv::register::sstatus::{self, Sstatus, SPP};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User); //previous privilege mode: user mode
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
    /// get function arguments
    pub fn get_func_args(&self) -> [usize; 3] {
        [self.x[10], self.x[11], self.x[12]]
    }
    /// get syscall id
    pub fn get_syscall_id(&self) -> usize {
        self.x[17]
    }
    /// set function return value
    pub fn set_func_ret(&mut self, ret: usize) {
        self.x[10] = ret;
    }
}


