use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, ForkResult, Pid};

fn main() {
    println!("[chibi] - init completed, process id: {}", std::process::id());

    let mut ipc_pid = fork_exec("ipc-bus");
    let mut sm_pid = fork_exec("/bin/service-manager");
    
    loop {
        match waitpid(None, None) {
            Ok(status) => {
                let pid = match status {
                    WaitStatus::Exited(pid, _) => pid,
                    WaitStatus::Signaled(pid, _, _) => pid,
                    _ => continue,
                };

                if pid == sm_pid {
                    eprintln!("ServiceManager crashed. Restarting...");
                    sm_pid = fork_exec("service-manager");
                }
                // Add similar handling for ipc_pid 
            }
            Err(e) => {
                eprintln!("Error waiting for child: {}", e);
                break;
            }
        }
    }
}

fn fork_exec(program: &str) -> Pid {
    match unsafe { fork() }.expect("Fork failed") {
        ForkResult::Parent { child } => child,
        ForkResult::Child => {
            let program_cstr = CString::new(program).expect("CString conversion failed");
            execvp(&program_cstr, &[program_cstr.clone()]).expect("Execvp failed");
            unreachable!();
        }
    }
}

fn log(message: &str) {
    match OpenOptions::new().write(true).open("/dev/console") {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "[chibi] {}", message) {
                println!("Failed to write to console: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to open console: {}", e);
        }
    }
}