use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    println!("Custom init started with PID {}", std::process::id());
    loop{
        log("hi");
        let st = Duration::from_secs(1);
        std::thread::sleep(st);
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

    
