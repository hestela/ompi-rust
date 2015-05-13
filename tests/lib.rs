extern crate mpi;

pub use mpi::*;

// Need to wrap call to init_null to prevent each thread from calling init.
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
  use super::*;

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
