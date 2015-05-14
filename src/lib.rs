extern crate libc;

use std::env::Args;
use libc::{ptrdiff_t, size_t, c_int, c_char};
use libc::{c_longlong, c_void, c_uint, c_double};

pub mod request;
pub mod bindings;
pub mod comm;
pub mod error;
pub mod status;

use bindings::*;

/*
pub fn init(argc: usize, argv: Vec<Args>) -> () {
  unsafe { ffi::MPI_Init(&argc as *mut c_int, argv) } //TODO: howdoi?
}
*/

/// Calls MPI_Init with null parameters.
/// As of MPI-2, MPI_Init will accept NULL as input parameters.
/// Must be called before any other mpi function is called.
/// # Panics
/// When called multiple times.
/// When called without finalize at end of program.
/// # Examples
/// ```
/// // Valid
/// init_null();
/// finalize();
///
/// // Invalid
/// init_null();
/// init_null();
/// finalize();
///
/// // Invalid without finalize
/// init_null();
/// ```
pub fn init_null() -> i32 {
  let argc = 0 as *mut c_int;
  let argv = 0 as *mut *mut *mut c_char;
  unsafe { MPI_Init(argc, argv) }
}

/// Cleans up mpi stuff. Should always return success, no need for Error.
/// # Panics
/// When called multiple times.
/// When called without init.
/// # Examples
/// ```
/// // Valid
/// init_null();
/// finalize();
///
/// // Invalid
/// init_null();
/// finalize();
/// finalize();
///
/// // Invalid without init
/// finalize();
/// ```
pub fn finalize() -> () {
  let _ = unsafe { MPI_Finalize() };
}
