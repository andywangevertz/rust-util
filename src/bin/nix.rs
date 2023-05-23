extern crate nix; 
use nix::sys::signal::*; 
use nix::unistd::*; 
fn main () { 
  match unsafe {fork ()}.expect ("fork failed") { 
    ForkResult::Parent { child } => { 
       sleep (5); 
       kill (child, SIGKILL).expect ("kill failed"); 
       println!("Parent exit");
    } 
    ForkResult::Child => { 
	loop {} // until killed 
        println!("Child exit"); 
    }
  } 
}
