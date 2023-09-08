use std::thread;



pub const SIGHUP :      u8 = 1;	    /* Hangup.  */
pub const SIGINT :      u8 = 2;	    /* Interactive attention signal.  */
pub const SIGQUIT:      u8 = 3;	    /* Quit.  */
pub const SIGILL :      u8 = 4;	    /* Illegal instruction.  */
pub const SIGTRAP:      u8 = 5;	    /* Trace/breakpoint trap.  */
pub const SIGABRT:      u8 = 6;	    /* Abnormal termination.  */
pub const SIGBUS:       u8 = 7;	    /* Bus error.  */
pub const SIGFPE :      u8 = 8;	    /* Erroneous arithmetic operation.  */
pub const SIGKILL:      u8 = 9;	    /* Killed.  */
pub const SIGUSR1:      u8 = 10;	/* User-defined signal 1.  */
pub const SIGSEGV:      u8 = 11;	/* Invalid access to storage.  */
pub const SIGUSR2:      u8 = 12;	/* User-defined signal 2.  */
pub const SIGPIPE:      u8 = 13;	/* Broken pipe.  */
pub const SIGALRM:      u8 = 14;	/* Alarm clock.  */
pub const SIGTERM:      u8 = 15;	/* Termination request.  */
pub const SIGSTKFLT:    u8 = 16;	/* Stack fault (obsolete).  */
pub const SIGCHLD:      u8 = 17;	/* Child terminated or stopped.  */
pub const SIGCONT:      u8 = 18;	/* Continue.  */
pub const SIGSTOP:      u8 = 19;	/* Stop, unblockable.  */
pub const SIGTSTP:      u8 = 20;	/* Keyboard stop.  */
pub const SIGTTIN:      u8 = 21;	/* Background read from control terminal.  */
pub const SIGTTOU:      u8 = 22;	/* Background write to control terminal.  */
pub const SIGURG:       u8 = 23;	/* Urgent data is available at a socket.  */
pub const SIGXCPU:      u8 = 24;	/* CPU time limit exceeded.  */
pub const SIGXFSZ:      u8 = 25;	/* File size limit exceeded.  */
pub const SIGVTALRM:    u8 = 26;	/* Virtual timer expired.  */
pub const SIGPROF:      u8 = 27;	/* Profiling timer expired.  */
pub const SIGWINCH:     u8 = 28;    /* Window size change (4.3 BSD, Sun).  */
pub const SIGPOLL:      u8 = 29;	/* Pollable event occurred (System V).  */
pub const SIGPWR:       u8 = 30;	/* Power failure imminent.  */
pub const SIGSYS:       u8 = 31;	/* Bad system call.  */



extern "C" {
    #[allow(improper_ctypes)]
    fn signal_wrapper(interrupt: i32, function: fn(i32)) -> fn(i32);
}

pub unsafe fn signal(interrupt: u8, function: fn(i32)) -> fn(i32) {
    signal_wrapper(interrupt as i32, function)
}


// Make all of this good.


static mut LIST: Vec<fn()> = Vec::new();


fn ctrl_c_handler(_: i32) {
    unsafe {
        for function in &LIST {
            thread::spawn(function);
        }
    }
}


pub unsafe fn add_ctrl_c_function(function: fn()) {
    LIST.push(function);
    signal(SIGINT, ctrl_c_handler);
}