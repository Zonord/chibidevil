use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    println!("[chibi] - init completed , process id : ", std::process::id());

    let ipc_pid = fork_exec("ipc-bus");
    let sm_pid = fork_exec("service-manager");
    
    loop{
        let (pid, status) = waitpid(-1, None);
        if pid == sm_pid {
            eprintln!("ServiceManager crashed. Restarting...");
            sm_pid = fork_exec("service-manager");
            }   
        }
}



fn log(message: &str) {
    match OpenOptions::new().write(true).open("/dev/console") {
        Ok(mut file) => {
   
            if let Err(e) = writeln!(file, "[chibi] {}", message) {
                println!("bleh : {}", e);
            }
        }
        Err(e) => {
 
            println!("bleh : {}", e);
        }
    }
}

    
