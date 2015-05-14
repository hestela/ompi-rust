use bindings::{ompi_status_public_t, MPI_Status};
use bindings::consts::UNDEFINED;

pub const STATUS_IGNORE: Option<MPI_Status> = None;

pub fn new() -> MPI_Status {
  ompi_status_public_t {
    source: UNDEFINED,
    tag: UNDEFINED,
    err: UNDEFINED,
    cancelled: 0,
    count: 0
  }
}
