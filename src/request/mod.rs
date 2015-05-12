use bindings::{ompi_request_null, MPI_Request};

pub fn new() -> MPI_Request {
  unsafe { &mut ompi_request_null }
}
