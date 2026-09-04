// Copyright (c) 2026 vivo Mobile Communication Co., Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `run` — launch a dynamic application through the `ApplicationLaunch`
//! syscall (C29, §18.2). The kernel resolves the path against the seeded
//! system image and links the app against the Ready `libc.so.1` instance.

use std::ffi::{c_char, CString};

use librs::spawn::spawn;

pub fn command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: run <path> [arg...]".to_string());
    }

    let path = CString::new(args[0]).map_err(|_| "path contains NUL".to_string())?;
    // POSIX shape: argv[0] is the application path, followed by the given
    // arguments, NUL-terminated. `envp` stays empty for Phase 1.
    let arg_strings: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(*arg).map_err(|_| "argument contains NUL".to_string()))
        .collect::<Result<_, _>>()?;
    let mut arg_ptrs: Vec<*const c_char> = arg_strings.iter().map(|c| c.as_ptr()).collect();
    arg_ptrs.push(std::ptr::null());
    let envp = [std::ptr::null::<c_char>()];

    let result = spawn(path.as_ptr(), arg_ptrs.as_ptr(), envp.as_ptr());
    if result < 0 {
        return Err(format!("spawn failed: errno {}", -result));
    }
    println!("launched handle {}", result);
    Ok(())
}
