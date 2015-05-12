// TODO: figure out how to repesent error conditions.
extern crate libc;

use std::env::Args;
use libc::{ptrdiff_t, size_t, c_int, c_char};
use libc::{c_longlong, c_void, c_uint, c_double};

pub mod bindings;
use bindings::*;

pub fn comm_new() -> MPI_Comm {
  unsafe { &mut ompi_communicator_t }
}

pub fn comm_world() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_world }
}

pub fn comm_self() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_self }
}

pub fn comm_null() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_null }
}

/*
pub fn init(argc: usize, argv: Vec<Args>) -> () {
  unsafe { ffi::MPI_Init(&argc as *mut c_int, argv) } //TODO: howdoi?
}
*/

/// Calls MPI_Init with null parameters.
/// As of MPI-2, MPI_Init will accept NULL as input parameters.
pub fn init_null() -> i32 {
  let argc = 0 as *mut c_int;
  let argv = 0 as *mut *mut *mut c_char;
  unsafe { MPI_Init(argc, argv) }
}

pub fn finalize() -> i32 {
  unsafe { MPI_Finalize() }
}

/// Returns the rank of the process in the communicator.
/// Doesnt "return" rank as a pointer link in C.
// TODO: Handle errors, dont just throw the error away
pub fn comm_rank(comm: MPI_Comm) -> i32 {
  let mut rank = -1;
  let _ = unsafe { MPI_Comm_rank(comm, &mut rank) };
  rank
}
