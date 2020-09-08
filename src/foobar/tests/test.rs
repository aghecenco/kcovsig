// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

extern crate libc;
extern crate foobar;

use std::thread::sleep;
use std::time::Duration;

use libc::{fork, kill, waitpid, SIGHUP, WIFSIGNALED, WTERMSIG};

fn wait_child_process(pid: i32) {
    let mut status: i32 = -1;
    let pid_done = unsafe { waitpid(pid, &mut status, 0) };
    assert_eq!(pid_done, pid);
    assert!(unsafe { WIFSIGNALED(status) });
    assert_eq!(unsafe { WTERMSIG(status) }, SIGHUP);
}

#[test]
fn test() {
    let pid = unsafe { fork() };
    match pid {
        0 => {
            // Child process. Do stuff, never die.
            loop {}
        }
        pid => {
            // Parent process: give the child some time, then kill it.
            sleep(Duration::from_secs(2));
            unsafe { kill(pid, SIGHUP) };
            wait_child_process(pid);
        }
    }
}
