use bindings::*;
use error::{Error, MPI_SUCCESS};

/// Creates an unitialized (on the C side) comm.
/// # Examples
/// ```
/// let comm = comm::new();
/// // In C
/// MPI_Comm comm;
/// ```
pub fn new() -> MPI_Comm {
  &mut ompi_communicator_t
}

// TODO: figure out how to make a const.
/// Gives MPI_COMM_WORLD.
/// # Examples
/// ```
/// let comm_world = comm::world();
/// let rank_in_world = comm::rank(world);
/// ```
pub fn world() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_world }
}

// self and Self reserved by rust, comm::comm_self seems redundant.
/// Gives MPI_COMM_SELF.
/// # Examples
/// ```
/// let rank = comm::rank(comm::slf());
/// ```
pub fn slf() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_self }
}

// TODO: find a use case for null comm for docs.
/// Gives MPI_COMM_NULL.
/// Causes mpi fns to panic when null comm is given as an argument.
pub fn null() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_null }
}

/// Gets the rank of the process in the communicator.
/// Doesn't "return" rank as a pointer like in C.
/// Replacement for MPI_Comm_rank.
/// # Examples
/// ```
/// let rank_in_world = comm::rank(comm::world());
/// // In C
/// int rank;
/// MPI_Comm_rank(MPI_COMM_WORLD, &rank);
/// ```
pub fn rank(comm: MPI_Comm) -> Result<i32, Error> {
  let mut rank = -1;
  let err = unsafe { MPI_Comm_rank(comm, &mut rank) };
  match err {
    MPI_SUCCESS => Ok(rank),
    _ => Err(Error::new(err))
  }
}
