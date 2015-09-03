use std::{thread};
extern crate sysinfo;
extern crate libc;
extern crate time;

fn kill_procs() {
    // Nobody aint got time for that
    let blocklist = ["firefox", "termite", "gimp", "thunderbird", "process_viewer"];
    let blocked = blocklist.iter();
    // TODO: Make config not hard coded

    let sys = sysinfo::System::new();
    let procs = sys.get_process_list();

    for (_, i) in procs {
        if blocked.clone().any(|x| x == &i.name) {
            i.kill(sysinfo::Signal::Kill);
        }
    }
}

fn condition() -> bool {
    // Time where blocking is enforced
    time::now().tm_hour < 22
}


fn check() {
    if condition() {
        thread::spawn(move || {
            println!("Checking thread started");
            kill_procs();
            thread::sleep_ms(2000);
            check();
        }).join().unwrap();
    }
}

fn main() {
    check();
}
