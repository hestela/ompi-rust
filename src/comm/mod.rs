use bindings::{ompi_communicator_t, ompi_mpi_comm_world, ompi_mpi_comm_self};
use bindings::{ompi_mpi_comm_null, MPI_Comm};

pub fn new() -> MPI_Comm {
  unsafe { &mut ompi_communicator_t }
}

pub fn world() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_world }
}

// self and Self reserved by rust, comm::comm_self seems redundant.
pub fn slf() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_self }
}

pub fn null() -> MPI_Comm {
  unsafe { &mut ompi_mpi_comm_null }
}
