# Extra FFI Tools

[![Build Status](https://jenkins.kaist.ac.kr/buildStatus/icon?job=ANLAB-KAIST%2Frust-ffi-extra%2Fmaster)](https://jenkins.kaist.ac.kr/job/ANLAB-KAIST/job/rust-ffi-extra/job/master/)

## `run_with_args`

Compose C-like argument vector from rusty string vectors.
Call a 'main-like' function with given arguments.

```{.rs}
use ffi_extra::run_with_args;

let args = vec!["bin_name", "first_arg"];
unsafe { run_with_args(ffi_main, &args) };

let args = vec![String::new("bin_name"), String::new("first_arg")];
unsafe { run_with_args(ffi_main, &args) };
```
