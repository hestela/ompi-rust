pub struct Error {
  pub code: i32,
  pub string: &'static str
}

impl Error {
  pub fn new(err: i32) -> Error {
    Error { code: err, string: Error::error_to_string(err) }
  }

  // TODO: Finish for all errors.
  pub fn error_to_string(err: i32) -> &'static str {
    match err {
      MPI_SUCCESS => "SUCCESS",
      MPI_ERR_BUFFER => "ERR BUFFER",
      MPI_ERR_COMM => "ERR COMM",
      MPI_ERR_RANK => "ERR RANK",
      _ => "ERR OTHER"
    }
  }
}

pub const MPI_SUCCESS: i32 = 0;
pub const MPI_ERR_BUFFER: i32 = 1;
pub const MPI_ERR_COUNT: i32 = 2;
pub const MPI_ERR_TYPE: i32 = 3;
pub const MPI_ERR_TAG: i32 = 4;
pub const MPI_ERR_COMM: i32 = 5;
pub const MPI_ERR_RANK: i32 = 6;
pub const MPI_ERR_REQUEST: i32 = 7;
pub const MPI_ERR_ROOT: i32 = 8;
pub const MPI_ERR_GROUP: i32 = 9;
pub const MPI_ERR_OP: i32 = 10;
pub const MPI_ERR_TOPOLOGY: i32 = 11;
pub const MPI_ERR_DIMS: i32 = 12;
pub const MPI_ERR_ARG: i32 = 13;
pub const MPI_ERR_UNKNOWN: i32 = 14;
pub const MPI_ERR_TRUNCATE: i32 = 15;
pub const MPI_ERR_OTHER: i32 = 16;
pub const MPI_ERR_INTERN: i32 = 17;
pub const MPI_ERR_IN_STATUS: i32 = 18;
pub const MPI_ERR_PENDING: i32 = 19;
pub const MPI_ERR_ACCESS: i32 = 20;
pub const MPI_ERR_AMODE: i32 = 21;
pub const MPI_ERR_ASSERT: i32 = 22;
pub const MPI_ERR_BAD_FILE: i32 = 23;
pub const MPI_ERR_BASE: i32 = 24;
pub const MPI_ERR_CONVERSION: i32 = 25;
pub const MPI_ERR_DISP: i32 = 26;
pub const MPI_ERR_DUP_DATAREP: i32 = 27;
pub const MPI_ERR_FILE_EXISTS: i32 = 28;
pub const MPI_ERR_FILE_IN_USE: i32 = 29;
pub const MPI_ERR_FILE: i32 = 30;
pub const MPI_ERR_INFO_KEY: i32 = 31;
pub const MPI_ERR_INFO_NOKEY: i32 = 32;
pub const MPI_ERR_INFO_VALUE: i32 = 33;
pub const MPI_ERR_INFO: i32 = 34;
pub const MPI_ERR_IO: i32 = 35;
pub const MPI_ERR_KEYVAL: i32 = 36;
pub const MPI_ERR_LOCKTYPE: i32 = 37;
pub const MPI_ERR_NAME: i32 = 38;
pub const MPI_ERR_NO_MEM: i32 = 39;
pub const MPI_ERR_NOT_SAME: i32 = 40;
pub const MPI_ERR_NO_SPACE: i32 = 41;
pub const MPI_ERR_NO_SUCH_FILE: i32 = 42;
pub const MPI_ERR_PORT: i32 = 43;
pub const MPI_ERR_QUOTA: i32 = 44;
pub const MPI_ERR_READ_ONLY: i32 = 45;
pub const MPI_ERR_RMA_CONFLICT: i32 = 46;
pub const MPI_ERR_RMA_SYNC: i32 = 47;
pub const MPI_ERR_SERVICE: i32 = 48;
pub const MPI_ERR_SIZE: i32 = 49;
pub const MPI_ERR_SPAWN: i32 = 50;
pub const MPI_ERR_UNSUPPORTED_DATAREP: i32 = 51;
pub const MPI_ERR_UNSUPPORTED_OPERATION: i32 = 52;
pub const MPI_ERR_WIN: i32 = 53;
pub const MPI_T_ERR_MEMORY: i32 = 54;
pub const MPI_T_ERR_NOT_INITIALIZED: i32 = 55;
pub const MPI_T_ERR_CANNOT_INIT: i32 = 56;
pub const MPI_T_ERR_INVALID_INDEX: i32 = 57;
pub const MPI_T_ERR_INVALID_ITEM: i32 = 58;
pub const MPI_T_ERR_INVALID_HANDLE: i32 = 59;
pub const MPI_T_ERR_OUT_OF_HANDLES: i32 = 60;
pub const MPI_T_ERR_OUT_OF_SESSIONS: i32 = 61;
pub const MPI_T_ERR_INVALID_SESSION: i32 = 62;
pub const MPI_T_ERR_CVAR_SET_NOT_NOW: i32 = 63;
pub const MPI_T_ERR_CVAR_SET_NEVER: i32 = 64;
pub const MPI_T_ERR_PVAR_NO_STARTSTOP: i32 = 65;
pub const MPI_T_ERR_PVAR_NO_WRITE: i32 = 66;
pub const MPI_T_ERR_PVAR_NO_ATOMIC: i32 = 67;
pub const MPI_ERR_RMA_RANGE: i32 = 68;
pub const MPI_ERR_RMA_ATTACH: i32 = 69;
pub const MPI_ERR_RMA_FLAVOR: i32 = 70;
pub const MPI_ERR_RMA_SHARED: i32 = 71;
pub const MPI_ERR_LASTCODE: i32 = MPI_ERR_RMA_SHARED;
pub const MPI_ERR_SYSRESOURCE: i32 = -2;
