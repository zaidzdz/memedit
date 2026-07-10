#[cfg(target_os = "macos")]
use mach::port::{mach_port_t, MACH_PORT_NULL};
#[cfg(target_os = "macos")]
use mach::kern_return::{kern_return_t, KERN_SUCCESS};
#[cfg(target_os = "macos")]
use mach::traps::{mach_task_self, task_for_pid};
#[cfg(target_os = "macos")]
use libproc::{processes};
#[cfg(target_os = "macos")]
use libproc::processes::ProcFilter;

#[cfg(not(target_os = "macos"))]
#[allow(non_camel_case_types)]
type mach_port_t = u32;
/// A handle to an open process.
pub struct Process {
    pub pid: u32,
    pub port: mach_port_t,
}
#[derive(Debug)]
/// Errors that can occur when opening or interacting with a process.
pub enum ProcessErr {
    ListFailed(String),
    KernError(String),
}
impl std::fmt::Display for ProcessErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessErr::ListFailed(s) => write!(f, "failed to list processes: {}", s),
            ProcessErr::KernError(s) => write!(f, "kernel error: {}", s),
        }
    }
}

impl std::error::Error for ProcessErr {}

#[cfg(target_os = "macos")]
impl ProcessErr {
    pub(crate) fn from_kern(kr: kern_return_t) -> Self {
        unsafe extern "C" { fn mach_error_string(err: kern_return_t) -> *const std::ffi::c_char;} // Import c library for mach_error_string function

        let msg = unsafe { std::ffi::CStr::from_ptr(mach_error_string(kr)) }
            .to_string_lossy()
            .to_string();
        ProcessErr::KernError(msg)
    }
}
impl Process {
    /// Opens a process by PID.
    pub fn open(pid: u32) -> Result<Process, ProcessErr> {
        #[cfg(target_os = "macos")]
        {
            let mut port:mach_port_t = MACH_PORT_NULL; //0
            let kern_ret:kern_return_t = unsafe {task_for_pid(mach_task_self(), pid as i32, &mut port)};

            return if kern_ret != KERN_SUCCESS {
                Err(ProcessErr::from_kern(kern_ret))
            } else {
                Ok(Process{pid, port})
            }
        }
        #[cfg(not(target_os = "macos"))]
        panic!("osxmem only supports macOS")
    }
    /// Opens a process by name.
    pub fn open_by_name(name: &str) -> Result<Process, ProcessErr> {
        #[cfg(target_os = "macos")]
        {
            let pids: Vec<u32> = processes::pids_by_type(ProcFilter::All).map_err(|e| ProcessErr::ListFailed(e.to_string()))?;

            let pid = pids.iter()
                .find(|pid| libproc::proc_pid::name(**pid as i32).ok().as_deref() == Some(name))
                .ok_or(ProcessErr::KernError("no such process".to_string()))?;

            return Process::open(*pid);
        }
        #[cfg(not(target_os = "macos"))]
        panic!("osxmem only supports macOS")
    }
}
