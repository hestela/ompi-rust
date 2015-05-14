extern crate mpi;

pub use mpi::*;

// Need to wrap call to init_null to prevent each thread from calling init.
// TODO: consider thread multiple or make another set with thread multiple.
fn sync_init() {
  use std::sync::{Once, ONCE_INIT};
  static INIT: Once = ONCE_INIT;
  INIT.call_once(|| {
    init_null();
  });
}

/* Finalize can only be called once/process including threads.
 * Can't test finalize due to race conditions, it would have to be called after
 * all tests complete.
 */
#[test]
fn init() {
  sync_init();
}

#[cfg(test)]
mod comm_tests {
  use super::mpi::*;

  #[test]
  fn rank_in_world() {
    super::sync_init();
    let rank = comm::rank(comm::world());
    match rank {
      Ok(r) => assert_eq!(r, 0),
      Err(e) => panic!("got error: {}", e.string)
    }
  }

  #[test]
  fn rank_in_self() {
    super::sync_init();
    let rank = comm::rank(comm::slf());
    match rank {
      Ok(r) => assert_eq!(r, 0),
      Err(e) => panic!("got error: {}", e.string)
    }
  }

  #[test]
  fn rank_in_bad_comm() {
    super::sync_init();
    let comm = comm::new();
    let rank = comm::rank(comm);
    match rank {
      Ok(r) => assert_eq!(r, 0),
      Err(e) => panic!("got error: {}", e.string)
    }
  }
}

// Can't make tests for request unil MPI_Isend/Irecv are implemented
#[cfg(test)]
mod request_tests {
  use super::mpi::*;
  // TODO: tests like this require 2 ranks. Try spawning another thread?
  #[test]
  fn wait_on_message() {
    super::sync_init();
    let req = request::new();
    // MPI_Isend/Irecv will make the req non-null.
  }

  #[test]
  fn req_to_stat() {
    super::sync_init();
    let req = request::new();
    // Make a request from Isend.
    /*
     * let stat = get_status(req).unwrap();
     * let err = wait(req)
     * match err {
     *   Ok() => ,
     *   Err(e) => panic!("Error from wait: {}", e.string)
     * }
     */
  }
}

#[cfg(test)]
mod error_tests {
  use super::mpi::*;

  #[test]
  fn check_err_string() {
    let err = error::Error::new(error::MPI_SUCCESS);
    assert_eq!(err.string, "SUCCESS");
  }
}
