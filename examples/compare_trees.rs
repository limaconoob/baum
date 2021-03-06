extern crate baum;
extern crate libc;

use baum::baum::{Baum, BaumBenutz};

/// Compare deux arbres de pids, In sont les pids créés, Out sont ceux détruits
fn main() {
    unsafe {
        let alt_baum: Baum = Baum::new(libc::getppid());
        let neu_baum: Baum = Baum::new(libc::getpid());
        let (in_pids, out_pids) = neu_baum.vergleich(alt_baum);
        println!("In::{:?} | Out::{:?}", in_pids, out_pids);
        println!("Pid::{} | PPid::{}", libc::getpid(), libc::getppid());

        println!("");

        let alt_baum: Baum = Baum::new(libc::getppid());
        let neu_baum: Baum = Baum::new(libc::getpid());
        let (in_pids, out_pids) = alt_baum.vergleich(neu_baum);
        println!("In::{:?} | Out::{:?}", in_pids, out_pids);
        println!("Pid::{} | PPid::{}", libc::getpid(), libc::getppid());

        println!("");

        let a: Baum = Baum::new(libc::getppid());
        let b: Baum = Baum::new(libc::getppid());
        if a == b {
            println!("a and b are equivalents");
        }
    }
}
