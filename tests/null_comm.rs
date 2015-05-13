extern crate mpi;

use mpi::*;

// The way ompi fails will kill the current process, other tests cannot go in here.
#[test]
#[should_panic]
fn comm_null_rank() {
  init_null();
  let _ = comm::rank(comm::null());
  finalize();
}
