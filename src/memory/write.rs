use mach::vm::mach_vm_write;
use mach::kern_return::{kern_return_t, KERN_SUCCESS};
use crate::process::{Process, ProcessErr};

pub fn write_mem<T: Copy>(process: &Process, address: usize, value:T) -> Result<(), ProcessErr> {
    let kern_ret:kern_return_t = unsafe {
        mach_vm_write(
            process.port,
            address as u64,
            &value as *const T as usize,
            std::mem::size_of::<T>() as u32,
        )
    };
    if kern_ret != KERN_SUCCESS {
        return Err(ProcessErr::from_kern(kern_ret));
    }
    return Ok(());
}
