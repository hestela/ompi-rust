use bindings::{ompi_request_t, MPI_Request, MPI_Status, MPI_Request_get_status};
use bindings::{MPI_Wait};
use error::{Error, MPI_SUCCESS};
use status;

pub fn new() -> MPI_Request {
  &mut ompi_request_t
}

// TODO: Provide examples after Isend/Irecv implemented
/// Replacement for MPI_Request_get_status.  
/// Since success is always retured, an option containing the status is
/// returned to avoid having to use int* flag to test success.
pub fn get_status(req: MPI_Request) -> Option<MPI_Status> {
  let mut flag: i32 = 0;
  let mut status = status::new();
  let _ = unsafe { MPI_Request_get_status(req, &mut flag, &mut status) };
    match flag {
    0 => None,
    _ => Some(status)
  }
}

// TODO: Test when MPI_STATUS_IGNORE comes from MPI_Wait
/// Replacement for MPI_Wait.  
/// Rerturns Ok(Option<MPI_Status) on MPI_SUCCESS, else Err.
/// MPI_STATUS_IGNORED is encapsulated in the option as None.
pub fn wait(mut req: MPI_Request) -> Result<Option<MPI_Status>, Error> {
  let mut stat = status::new();
  let err = unsafe { MPI_Wait(&mut req, &mut stat) };
  match err {
    MPI_SUCCESS => Ok(Some(stat)),
    _ => Err(Error::new(err))
  }
}
