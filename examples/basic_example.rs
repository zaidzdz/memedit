use memedit::process::Process;
use memedit::memory::{read_mem,write_mem};
fn main() {
    let health_address: usize =  0x7060E5800;
    // Attach to process with name "Game"
    let process: Process = Process::open_by_name("Game").unwrap();

    let mut value: f32 = read_mem::<f32>(&process, health_address).unwrap(); // Read a f32 value (float) at the "health_address"
    println!("{}", value);

    write_mem::<f32>(&process, health_address,200.0).unwrap(); // Write 200 to the "health_address"

    value = read_mem::<f32>(&process, health_address).unwrap(); // Read the same address again
    println!("{}", value);

}