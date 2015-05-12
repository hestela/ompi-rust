use bindings::{Struct_ompi_status_public_t, MPI_Status};

pub fn new() -> MPI_Status {
  Struct_ompi_status_public_t::default()
}
