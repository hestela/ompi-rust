extern crate libc;

use std::env::Args;
use libc::{ptrdiff_t, size_t, c_int, c_char};
use libc::{c_longlong, c_void, c_uint, c_double};

pub mod bindings;
use bindings::*;

// Aliases for MPI_* types
pub type CommWorld = ompi_mpi_comm_world;

// TODO: figure out how to repesent error conditions.
/*
pub fn init(argc: usize, argv: Vec<Args>) -> () {
  unsafe { ffi::MPI_Init(&argc as *mut c_int, argv) } //TODO: howdoi?
}
*/

/// Calls MPI_Init with null parameters, temporary function
pub fn init_null() -> i32 {
  let argc = 0 as *mut c_int;
  let argv = 0 as *mut *mut *mut c_char;
  unsafe { MPI_Init(argc, argv) }
}

pub fn finalize() -> i32 {
  unsafe { MPI_Finalize() }
}

// TODO: Handle errors, dont just throw the error away
pub fn comm_rank(comm: MPI_Comm) -> i32 {
  let mut rank = -1;
  let _ = unsafe { MPI_Comm_rank(comm, &mut rank) };
  rank
}
