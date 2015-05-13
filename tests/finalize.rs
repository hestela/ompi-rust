extern crate mpi;

use mpi::*;

/* Since we are testing init and finalize, can't do any other tests that
 * require a non-finalized proccess.
 */
#[test]
fn init_null_finalize() {
  let err = init_null();
  assert_eq!(err, mpi::error::MPI_SUCCESS);
  finalize();
}
