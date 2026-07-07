use std::iter::Product;
use mach::port::{mach_port_t, MACH_PORT_NULL};
use mach::kern_return::{kern_return_t, KERN_SUCCESS};
use mach::mach_types::task_port_t;
use mach::traps::{mach_task_self, task_for_pid};
pub struct Process {
    pub pid: u32,
    pub port: mach_port_t,
}
#[derive(Debug)]
pub enum ProcessErr {
    PermissionDenied,        // KERN_FAILURE 5
    NoSuchProcess,           // KERN_INVALID_ARGUMENT 4
    Other(kern_return_t),    // something else
}
impl ProcessErr {
    fn from_kern(kr: kern_return_t) -> Self {
        match kr {
            5 => ProcessErr::PermissionDenied,
            4 => ProcessErr::NoSuchProcess,
            other => ProcessErr::Other(other),
        }
    }
}
impl Process {
    pub fn open(pid: u32) -> Result<Process, ProcessErr> {
        let mut port:mach_port_t = MACH_PORT_NULL; //0
        let kern_ret = unsafe {task_for_pid(mach_task_self(), pid as i32, &mut port)};

        return if kern_ret != KERN_SUCCESS {
            Err(ProcessErr::from_kern(kern_ret))
        } else {
            Ok(Process(pid, port))
        }
    }
}