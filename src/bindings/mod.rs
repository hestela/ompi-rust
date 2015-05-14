/* The types and functions defined here are not meant to be used directly.
 * Instead, wrappers are provided in other mods.
 * Ex. prefer to use mpi::comm::rank() instead of unsafe{ MPI_Comm_Rank() }.
 */
// TODO: temp warning supression until I make these bindings more rusty.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod consts;

use libc::{ptrdiff_t, size_t, c_int, c_char};
use libc::{c_longlong, c_void, c_uint, c_double};

/* In C many MPI types are given to users as a pointer to an incomplete struct.
 * We can do the same thing by making a pointer to a unit struct.
 */
#[repr(C)]
pub struct ompi_communicator_t;
#[repr(C)]
pub struct ompi_group_t;
#[repr(C)]
pub struct mca_base_pvar_handle_t;
#[repr(C)]
pub struct mca_base_pvar_session_t;
#[repr(C)]
pub struct mca_base_var_enum_t;
#[repr(C)]
pub struct ompi_datatype_t;
#[repr(C)]
pub struct ompi_errhandler_t;
#[repr(C)]
pub struct ompi_file_t;
#[repr(C)]
pub struct ompi_info_t;
#[repr(C)]
pub struct ompi_message_t;
#[repr(C)]
pub struct ompi_mpit_cvar_handle_t;
#[repr(C)]
pub struct ompi_op_t;
#[repr(C)]
pub struct ompi_request_t;
#[repr(C)]
pub struct ompi_win_t;

// One of the few structs that we require access to its members.
#[repr(C)]
pub struct ompi_status_public_t {
    pub source: c_int,
    pub tag: c_int,
    pub err: c_int,
    pub cancelled: c_int,
    pub count: size_t,
}

// User level types
pub type MPI_Aint = ptrdiff_t;
pub type MPI_Comm = *mut ompi_communicator_t;
pub type MPI_Count = c_longlong;
pub type MPI_Datatype = *mut ompi_datatype_t;
pub type MPI_Errhandler = *mut ompi_errhandler_t;
pub type MPI_File = *mut ompi_file_t;
pub type MPI_Group = *mut ompi_group_t;
pub type MPI_Info = *mut ompi_info_t;
pub type MPI_Message = *mut ompi_message_t;
pub type MPI_Offset = c_longlong;
pub type MPI_Op = *mut ompi_op_t;
pub type MPI_Request = *mut ompi_request_t;
pub type MPI_Status = ompi_status_public_t;
pub type MPI_T_cvar_handle = *mut ompi_mpit_cvar_handle_t;
pub type MPI_T_enum = *mut mca_base_var_enum_t;
pub type MPI_T_pvar_handle = *mut mca_base_pvar_handle_t;
pub type MPI_T_pvar_session = *mut mca_base_pvar_session_t;
pub type MPI_Win = *mut ompi_win_t;

pub type MPI_Copy_function =
    extern "C" fn(arg1: MPI_Comm, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void,
                  arg5: *mut c_void, arg6: *mut c_int)
        -> c_int;
pub type MPI_Delete_function =
    extern "C" fn(arg1: MPI_Comm, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void)
        -> c_int;
pub type MPI_Datarep_extent_function =
    extern "C" fn(arg1: MPI_Datatype, arg2: *mut MPI_Aint,
                  arg3: *mut c_void) -> c_int;
pub type MPI_Datarep_conversion_function =
    extern "C" fn(arg1: *mut c_void, arg2: MPI_Datatype,
                  arg3: c_int, arg4: *mut c_void,
                  arg5: MPI_Offset, arg6: *mut c_void)
        -> c_int;
pub type MPI_Comm_errhandler_function =
    extern "C" fn(arg1: *mut MPI_Comm, arg2: *mut c_int, ...) -> ();
pub type MPI_Comm_errhandler_fn = MPI_Comm_errhandler_function;
pub type ompi_file_errhandler_fn =
    extern "C" fn(arg1: *mut MPI_File, arg2: *mut c_int, ...) -> ();
pub type MPI_File_errhandler_fn = ompi_file_errhandler_fn;
pub type MPI_File_errhandler_function = ompi_file_errhandler_fn;
pub type MPI_Win_errhandler_function =
    extern "C" fn(arg1: *mut MPI_Win, arg2: *mut c_int, ...) -> ();
pub type MPI_Win_errhandler_fn = MPI_Win_errhandler_function;
pub type MPI_Handler_function =
    extern "C" fn(arg1: *mut MPI_Comm, arg2: *mut c_int, ...) -> ();
pub type MPI_User_function =
    extern "C" fn(arg1: *mut c_void, arg2: *mut c_void,
                  arg3: *mut c_int, arg4: *mut MPI_Datatype) -> ();
pub type MPI_Comm_copy_attr_function =
    extern "C" fn(arg1: MPI_Comm, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void,
                  arg5: *mut c_void, arg6: *mut c_int)
        -> c_int;
pub type MPI_Comm_delete_attr_function =
    extern "C" fn(arg1: MPI_Comm, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void)
        -> c_int;
pub type MPI_Type_copy_attr_function =
    extern "C" fn(arg1: MPI_Datatype, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void,
                  arg5: *mut c_void, arg6: *mut c_int)
        -> c_int;
pub type MPI_Type_delete_attr_function =
    extern "C" fn(arg1: MPI_Datatype, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void)
        -> c_int;
pub type MPI_Win_copy_attr_function =
    extern "C" fn(arg1: MPI_Win, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void,
                  arg5: *mut c_void, arg6: *mut c_int)
        -> c_int;
pub type MPI_Win_delete_attr_function =
    extern "C" fn(arg1: MPI_Win, arg2: c_int,
                  arg3: *mut c_void, arg4: *mut c_void)
        -> c_int;
pub type MPI_Grequest_query_function =
    extern "C" fn(arg1: *mut c_void, arg2: *mut MPI_Status)
        -> c_int;
pub type MPI_Grequest_free_function =
    extern "C" fn(arg1: *mut c_void) -> c_int;
pub type MPI_Grequest_cancel_function =
    extern "C" fn(arg1: *mut c_void, arg2: c_int)
        -> c_int;

#[link(name = "mpi")]
extern "C" {
    /* I know these are supposed to be the predefined types but,
     * using those types requires uncomfortable typecasts in rust.
     * The predefined types just have padding with the normal type.
     * TODO: confirm this.
     * TODO: cleanup types that will never be used.
     */
    pub static mut ompi_mpi_comm_world: ompi_communicator_t;
    pub static mut ompi_mpi_comm_self: ompi_communicator_t;
    pub static mut ompi_mpi_comm_null: ompi_communicator_t;
    pub static mut ompi_mpi_group_empty: ompi_group_t;
    pub static mut ompi_mpi_group_null: ompi_group_t;
    pub static mut ompi_request_null: ompi_request_t;
    pub static mut ompi_message_null: ompi_message_t;
    pub static mut ompi_message_no_proc: ompi_message_t;
    pub static mut ompi_mpi_op_null: ompi_op_t;
    pub static mut ompi_mpi_op_min: ompi_op_t;
    pub static mut ompi_mpi_op_max: ompi_op_t;
    pub static mut ompi_mpi_op_sum: ompi_op_t;
    pub static mut ompi_mpi_op_prod: ompi_op_t;
    pub static mut ompi_mpi_op_land: ompi_op_t;
    pub static mut ompi_mpi_op_band: ompi_op_t;
    pub static mut ompi_mpi_op_lor: ompi_op_t;
    pub static mut ompi_mpi_op_bor: ompi_op_t;
    pub static mut ompi_mpi_op_lxor: ompi_op_t;
    pub static mut ompi_mpi_op_bxor: ompi_op_t;
    pub static mut ompi_mpi_op_maxloc: ompi_op_t;
    pub static mut ompi_mpi_op_minloc: ompi_op_t;
    pub static mut ompi_mpi_op_replace: ompi_op_t;
    pub static mut ompi_mpi_op_no_op: ompi_op_t;
    pub static mut ompi_mpi_datatype_null: ompi_datatype_t;
    pub static mut ompi_mpi_lb: ompi_datatype_t;
    pub static mut ompi_mpi_ub: ompi_datatype_t;
    pub static mut ompi_mpi_char: ompi_datatype_t;
    pub static mut ompi_mpi_signed_char: ompi_datatype_t;
    pub static mut ompi_mpi_unsigned_char: ompi_datatype_t;
    pub static mut ompi_mpi_byte: ompi_datatype_t;
    pub static mut ompi_mpi_short: ompi_datatype_t;
    pub static mut ompi_mpi_unsigned_short: ompi_datatype_t;
    pub static mut ompi_mpi_int: ompi_datatype_t;
    pub static mut ompi_mpi_unsigned: ompi_datatype_t;
    pub static mut ompi_mpi_long: ompi_datatype_t;
    pub static mut ompi_mpi_unsigned_long: ompi_datatype_t;
    pub static mut ompi_mpi_long_long_int: ompi_datatype_t;
    pub static mut ompi_mpi_unsigned_long_long:
               ompi_datatype_t;
    pub static mut ompi_mpi_float: ompi_datatype_t;
    pub static mut ompi_mpi_double: ompi_datatype_t;
    pub static mut ompi_mpi_long_double: ompi_datatype_t;
    pub static mut ompi_mpi_wchar: ompi_datatype_t;
    pub static mut ompi_mpi_packed: ompi_datatype_t;
    pub static mut ompi_mpi_cxx_bool: ompi_datatype_t;
    pub static mut ompi_mpi_cxx_cplex: ompi_datatype_t;
    pub static mut ompi_mpi_cxx_dblcplex: ompi_datatype_t;
    pub static mut ompi_mpi_cxx_ldblcplex: ompi_datatype_t;
    pub static mut ompi_mpi_logical: ompi_datatype_t;
    pub static mut ompi_mpi_character: ompi_datatype_t;
    pub static mut ompi_mpi_integer: ompi_datatype_t;
    pub static mut ompi_mpi_real: ompi_datatype_t;
    pub static mut ompi_mpi_dblprec: ompi_datatype_t;
    pub static mut ompi_mpi_cplex: ompi_datatype_t;
    pub static mut ompi_mpi_dblcplex: ompi_datatype_t;
    pub static mut ompi_mpi_ldblcplex: ompi_datatype_t;
    pub static mut ompi_mpi_2int: ompi_datatype_t;
    pub static mut ompi_mpi_2integer: ompi_datatype_t;
    pub static mut ompi_mpi_2real: ompi_datatype_t;
    pub static mut ompi_mpi_2dblprec: ompi_datatype_t;
    pub static mut ompi_mpi_2cplex: ompi_datatype_t;
    pub static mut ompi_mpi_2dblcplex: ompi_datatype_t;
    pub static mut ompi_mpi_float_int: ompi_datatype_t;
    pub static mut ompi_mpi_double_int: ompi_datatype_t;
    pub static mut ompi_mpi_longdbl_int: ompi_datatype_t;
    pub static mut ompi_mpi_short_int: ompi_datatype_t;
    pub static mut ompi_mpi_long_int: ompi_datatype_t;
    pub static mut ompi_mpi_logical1: ompi_datatype_t;
    pub static mut ompi_mpi_logical2: ompi_datatype_t;
    pub static mut ompi_mpi_logical4: ompi_datatype_t;
    pub static mut ompi_mpi_logical8: ompi_datatype_t;
    pub static mut ompi_mpi_integer1: ompi_datatype_t;
    pub static mut ompi_mpi_integer2: ompi_datatype_t;
    pub static mut ompi_mpi_integer4: ompi_datatype_t;
    pub static mut ompi_mpi_integer8: ompi_datatype_t;
    pub static mut ompi_mpi_integer16: ompi_datatype_t;
    pub static mut ompi_mpi_real2: ompi_datatype_t;
    pub static mut ompi_mpi_real4: ompi_datatype_t;
    pub static mut ompi_mpi_real8: ompi_datatype_t;
    pub static mut ompi_mpi_real16: ompi_datatype_t;
    pub static mut ompi_mpi_complex8: ompi_datatype_t;
    pub static mut ompi_mpi_complex16: ompi_datatype_t;
    pub static mut ompi_mpi_complex32: ompi_datatype_t;
    pub static mut ompi_mpi_int8_t: ompi_datatype_t;
    pub static mut ompi_mpi_uint8_t: ompi_datatype_t;
    pub static mut ompi_mpi_int16_t: ompi_datatype_t;
    pub static mut ompi_mpi_uint16_t: ompi_datatype_t;
    pub static mut ompi_mpi_int32_t: ompi_datatype_t;
    pub static mut ompi_mpi_uint32_t: ompi_datatype_t;
    pub static mut ompi_mpi_int64_t: ompi_datatype_t;
    pub static mut ompi_mpi_uint64_t: ompi_datatype_t;
    pub static mut ompi_mpi_aint: ompi_datatype_t;
    pub static mut ompi_mpi_offset: ompi_datatype_t;
    pub static mut ompi_mpi_count: ompi_datatype_t;
    pub static mut ompi_mpi_c_bool: ompi_datatype_t;
    pub static mut ompi_mpi_c_complex: ompi_datatype_t;
    pub static mut ompi_mpi_c_float_complex:
               ompi_datatype_t;
    pub static mut ompi_mpi_c_double_complex:
               ompi_datatype_t;
    pub static mut ompi_mpi_c_long_double_complex:
               ompi_datatype_t;
    pub static mut ompi_mpi_errhandler_null:
               ompi_errhandler_t;
    pub static mut ompi_mpi_errors_are_fatal:
               ompi_errhandler_t;
    pub static mut ompi_mpi_errors_return:
               ompi_errhandler_t;
    pub static mut ompi_mpi_win_null: ompi_win_t;
    pub static mut ompi_mpi_file_null: ompi_file_t;
    pub static mut ompi_mpi_info_null: ompi_info_t;
    pub static mut ompi_mpi_info_env: ompi_info_t;
    pub static mut MPI_F_STATUS_IGNORE: *mut c_int;
    pub static mut MPI_F_STATUSES_IGNORE: *mut c_int;
}

#[link(name = "mpi")]
extern "C" {
    pub fn OMPI_C_MPI_TYPE_NULL_DELETE_FN(datatype: MPI_Datatype,
                                          type_keyval: c_int,
                                          attribute_val_out:
                                              *mut c_void,
                                          extra_state: *mut c_void)
     -> c_int;
    pub fn OMPI_C_MPI_TYPE_NULL_COPY_FN(datatype: MPI_Datatype,
                                        type_keyval: c_int,
                                        extra_state: *mut c_void,
                                        attribute_val_in: *mut c_void,
                                        attribute_val_out:
                                            *mut c_void,
                                        flag: *mut c_int)
     -> c_int;
    pub fn OMPI_C_MPI_TYPE_DUP_FN(datatype: MPI_Datatype,
                                  type_keyval: c_int,
                                  extra_state: *mut c_void,
                                  attribute_val_in: *mut c_void,
                                  attribute_val_out: *mut c_void,
                                  flag: *mut c_int) -> c_int;
    pub fn OMPI_C_MPI_COMM_NULL_DELETE_FN(comm: MPI_Comm,
                                          comm_keyval: c_int,
                                          attribute_val_out:
                                              *mut c_void,
                                          extra_state: *mut c_void)
     -> c_int;
    pub fn OMPI_C_MPI_COMM_NULL_COPY_FN(comm: MPI_Comm,
                                        comm_keyval: c_int,
                                        extra_state: *mut c_void,
                                        attribute_val_in: *mut c_void,
                                        attribute_val_out:
                                            *mut c_void,
                                        flag: *mut c_int)
     -> c_int;
    pub fn OMPI_C_MPI_COMM_DUP_FN(comm: MPI_Comm, comm_keyval: c_int,
                                  extra_state: *mut c_void,
                                  attribute_val_in: *mut c_void,
                                  attribute_val_out: *mut c_void,
                                  flag: *mut c_int) -> c_int;
    pub fn OMPI_C_MPI_NULL_DELETE_FN(comm: MPI_Comm,
                                     comm_keyval: c_int,
                                     attribute_val_out: *mut c_void,
                                     extra_state: *mut c_void)
     -> c_int;
    pub fn OMPI_C_MPI_NULL_COPY_FN(comm: MPI_Comm, comm_keyval: c_int,
                                   extra_state: *mut c_void,
                                   attribute_val_in: *mut c_void,
                                   attribute_val_out: *mut c_void,
                                   flag: *mut c_int) -> c_int;
    pub fn OMPI_C_MPI_DUP_FN(comm: MPI_Comm, comm_keyval: c_int,
                             extra_state: *mut c_void,
                             attribute_val_in: *mut c_void,
                             attribute_val_out: *mut c_void,
                             flag: *mut c_int) -> c_int;
    pub fn OMPI_C_MPI_WIN_NULL_DELETE_FN(window: MPI_Win,
                                         win_keyval: c_int,
                                         attribute_val_out:
                                             *mut c_void,
                                         extra_state: *mut c_void)
     -> c_int;
    pub fn OMPI_C_MPI_WIN_NULL_COPY_FN(window: MPI_Win,
                                       win_keyval: c_int,
                                       extra_state: *mut c_void,
                                       attribute_val_in: *mut c_void,
                                       attribute_val_out: *mut c_void,
                                       flag: *mut c_int)
     -> c_int;
    pub fn OMPI_C_MPI_WIN_DUP_FN(window: MPI_Win, win_keyval: c_int,
                                 extra_state: *mut c_void,
                                 attribute_val_in: *mut c_void,
                                 attribute_val_out: *mut c_void,
                                 flag: *mut c_int) -> c_int;
    pub fn MPI_Abort(comm: MPI_Comm, errorcode: c_int)
     -> c_int;
    pub fn MPI_Accumulate(origin_addr: *const c_void,
                          origin_count: c_int,
                          origin_datatype: MPI_Datatype,
                          target_rank: c_int, target_disp: MPI_Aint,
                          target_count: c_int,
                          target_datatype: MPI_Datatype, op: MPI_Op,
                          win: MPI_Win) -> c_int;
    pub fn MPI_Add_error_class(errorclass: *mut c_int)
     -> c_int;
    pub fn MPI_Add_error_code(errorclass: c_int,
                              errorcode: *mut c_int) -> c_int;
    pub fn MPI_Add_error_string(errorcode: c_int,
                                string: *const c_char)
     -> c_int;
    pub fn MPI_Address(location: *mut c_void, address: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Allgather(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         comm: MPI_Comm) -> c_int;
    pub fn MPI_Iallgather(sendbuf: *const c_void,
                          sendcount: c_int, sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcount: c_int, recvtype: MPI_Datatype,
                          comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Allgatherv(sendbuf: *const c_void,
                          sendcount: c_int, sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcounts: *const c_int,
                          displs: *const c_int,
                          recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Iallgatherv(sendbuf: *const c_void,
                           sendcount: c_int, sendtype: MPI_Datatype,
                           recvbuf: *mut c_void,
                           recvcounts: *const c_int,
                           displs: *const c_int,
                           recvtype: MPI_Datatype, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn MPI_Alloc_mem(size: MPI_Aint, info: MPI_Info,
                         baseptr: *mut c_void) -> c_int;
    pub fn MPI_Allreduce(sendbuf: *const c_void,
                         recvbuf: *mut c_void, count: c_int,
                         datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Iallreduce(sendbuf: *const c_void,
                          recvbuf: *mut c_void, count: c_int,
                          datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Alltoall(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        comm: MPI_Comm) -> c_int;
    pub fn MPI_Ialltoall(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Alltoallv(sendbuf: *const c_void,
                         sendcounts: *const c_int,
                         sdispls: *const c_int,
                         sendtype: MPI_Datatype, recvbuf: *mut c_void,
                         recvcounts: *const c_int,
                         rdispls: *const c_int,
                         recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ialltoallv(sendbuf: *const c_void,
                          sendcounts: *const c_int,
                          sdispls: *const c_int,
                          sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcounts: *const c_int,
                          rdispls: *const c_int,
                          recvtype: MPI_Datatype, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Alltoallw(sendbuf: *const c_void,
                         sendcounts: *const c_int,
                         sdispls: *const c_int,
                         sendtypes: *const MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcounts: *const c_int,
                         rdispls: *const c_int,
                         recvtypes: *const MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ialltoallw(sendbuf: *const c_void,
                          sendcounts: *const c_int,
                          sdispls: *const c_int,
                          sendtypes: *const MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcounts: *const c_int,
                          rdispls: *const c_int,
                          recvtypes: *const MPI_Datatype, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Attr_delete(comm: MPI_Comm, keyval: c_int)
     -> c_int;
    pub fn MPI_Attr_get(comm: MPI_Comm, keyval: c_int,
                        attribute_val: *mut c_void,
                        flag: *mut c_int) -> c_int;
    pub fn MPI_Attr_put(comm: MPI_Comm, keyval: c_int,
                        attribute_val: *mut c_void) -> c_int;
    pub fn MPI_Barrier(comm: MPI_Comm) -> c_int;
    pub fn MPI_Ibarrier(comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Bcast(buffer: *mut c_void, count: c_int,
                     datatype: MPI_Datatype, root: c_int,
                     comm: MPI_Comm) -> c_int;
    pub fn MPI_Bsend(buf: *const c_void, count: c_int,
                     datatype: MPI_Datatype, dest: c_int,
                     tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Ibcast(buffer: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, root: c_int,
                      comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Bsend_init(buf: *const c_void, count: c_int,
                          datatype: MPI_Datatype, dest: c_int,
                          tag: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Buffer_attach(buffer: *mut c_void, size: c_int)
     -> c_int;
    pub fn MPI_Buffer_detach(buffer: *mut c_void,
                             size: *mut c_int) -> c_int;
    pub fn MPI_Cancel(request: *mut MPI_Request) -> c_int;
    pub fn MPI_Cart_coords(comm: MPI_Comm, rank: c_int,
                           maxdims: c_int, coords: *mut c_int)
     -> c_int;
    pub fn MPI_Cart_create(old_comm: MPI_Comm, ndims: c_int,
                           dims: *const c_int,
                           periods: *const c_int,
                           reorder: c_int, comm_cart: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Cart_get(comm: MPI_Comm, maxdims: c_int,
                        dims: *mut c_int, periods: *mut c_int,
                        coords: *mut c_int) -> c_int;
    pub fn MPI_Cart_map(comm: MPI_Comm, ndims: c_int,
                        dims: *const c_int,
                        periods: *const c_int,
                        newrank: *mut c_int) -> c_int;
    pub fn MPI_Cart_rank(comm: MPI_Comm, coords: *const c_int,
                         rank: *mut c_int) -> c_int;
    pub fn MPI_Cart_shift(comm: MPI_Comm, direction: c_int,
                          disp: c_int,
                          rank_source: *mut c_int,
                          rank_dest: *mut c_int) -> c_int;
    pub fn MPI_Cart_sub(comm: MPI_Comm, remain_dims: *const c_int,
                        new_comm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Cartdim_get(comm: MPI_Comm, ndims: *mut c_int)
     -> c_int;
    pub fn MPI_Close_port(port_name: *const c_char) -> c_int;
    pub fn MPI_Comm_accept(port_name: *const c_char, info: MPI_Info,
                           root: c_int, comm: MPI_Comm,
                           newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_c2f(comm: MPI_Comm) -> c_int;
    pub fn MPI_Comm_call_errhandler(comm: MPI_Comm, errorcode: c_int)
     -> c_int;
    pub fn MPI_Comm_compare(comm1: MPI_Comm, comm2: MPI_Comm,
                            result: *mut c_int) -> c_int;
    pub fn MPI_Comm_connect(port_name: *const c_char, info: MPI_Info,
                            root: c_int, comm: MPI_Comm,
                            newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_create_errhandler(function:
                                          *mut MPI_Comm_errhandler_function,
                                      errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Comm_create_keyval(comm_copy_attr_fn:
                                      *mut MPI_Comm_copy_attr_function,
                                  comm_delete_attr_fn:
                                      *mut MPI_Comm_delete_attr_function,
                                  comm_keyval: *mut c_int,
                                  extra_state: *mut c_void)
     -> c_int;
    pub fn MPI_Comm_create_group(comm: MPI_Comm, group: MPI_Group,
                                 tag: c_int, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Comm_create(comm: MPI_Comm, group: MPI_Group,
                           newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_delete_attr(comm: MPI_Comm, comm_keyval: c_int)
     -> c_int;
    pub fn MPI_Comm_disconnect(comm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_dup(comm: MPI_Comm, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Comm_idup(comm: MPI_Comm, newcomm: *mut MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn MPI_Comm_dup_with_info(comm: MPI_Comm, info: MPI_Info,
                                  newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_f2c(comm: c_int) -> MPI_Comm;
    pub fn MPI_Comm_free_keyval(comm_keyval: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_free(comm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_get_attr(comm: MPI_Comm, comm_keyval: c_int,
                             attribute_val: *mut c_void,
                             flag: *mut c_int) -> c_int;
    pub fn MPI_Dist_graph_create(comm_old: MPI_Comm, n: c_int,
                                 nodes: *const c_int,
                                 degrees: *const c_int,
                                 targets: *const c_int,
                                 weights: *const c_int,
                                 info: MPI_Info, reorder: c_int,
                                 newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Dist_graph_create_adjacent(comm_old: MPI_Comm,
                                          indegree: c_int,
                                          sources: *const c_int,
                                          sourceweights: *const c_int,
                                          outdegree: c_int,
                                          destinations: *const c_int,
                                          destweights: *const c_int,
                                          info: MPI_Info,
                                          reorder: c_int,
                                          comm_dist_graph: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Dist_graph_neighbors(comm: MPI_Comm,
                                    maxindegree: c_int,
                                    sources: *mut c_int,
                                    sourceweights: *mut c_int,
                                    maxoutdegree: c_int,
                                    destinations: *mut c_int,
                                    destweights: *mut c_int)
     -> c_int;
    pub fn MPI_Dist_graph_neighbors_count(comm: MPI_Comm,
                                          inneighbors: *mut c_int,
                                          outneighbors: *mut c_int,
                                          weighted: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_get_errhandler(comm: MPI_Comm,
                                   erhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Comm_get_info(comm: MPI_Comm, info_used: *mut MPI_Info)
     -> c_int;
    pub fn MPI_Comm_get_name(comm: MPI_Comm, comm_name: *mut c_char,
                             resultlen: *mut c_int) -> c_int;
    pub fn MPI_Comm_get_parent(parent: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_group(comm: MPI_Comm, group: *mut MPI_Group)
     -> c_int;
    pub fn MPI_Comm_join(fd: c_int, intercomm: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Comm_rank(comm: MPI_Comm, rank: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_remote_group(comm: MPI_Comm, group: *mut MPI_Group)
     -> c_int;
    pub fn MPI_Comm_remote_size(comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_set_attr(comm: MPI_Comm, comm_keyval: c_int,
                             attribute_val: *mut c_void)
     -> c_int;
    pub fn MPI_Comm_set_errhandler(comm: MPI_Comm, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn MPI_Comm_set_info(comm: MPI_Comm, info: MPI_Info) -> c_int;
    pub fn MPI_Comm_set_name(comm: MPI_Comm, comm_name: *const c_char)
     -> c_int;
    pub fn MPI_Comm_size(comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_spawn(command: *const c_char,
                          argv: *mut *mut c_char,
                          maxprocs: c_int, info: MPI_Info,
                          root: c_int, comm: MPI_Comm,
                          intercomm: *mut MPI_Comm,
                          array_of_errcodes: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_spawn_multiple(count: c_int,
                                   array_of_commands:
                                       *mut *mut c_char,
                                   array_of_argv:
                                       *mut *mut *mut c_char,
                                   array_of_maxprocs: *const c_int,
                                   array_of_info: *const MPI_Info,
                                   root: c_int, comm: MPI_Comm,
                                   intercomm: *mut MPI_Comm,
                                   array_of_errcodes: *mut c_int)
     -> c_int;
    pub fn MPI_Comm_split(comm: MPI_Comm, color: c_int,
                          key: c_int, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Comm_split_type(comm: MPI_Comm, split_type: c_int,
                               key: c_int, info: MPI_Info,
                               newcomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Comm_test_inter(comm: MPI_Comm, flag: *mut c_int)
     -> c_int;
    pub fn MPI_Compare_and_swap(origin_addr: *mut c_void,
                                compare_addr: *mut c_void,
                                result_addr: *mut c_void,
                                datatype: MPI_Datatype,
                                target_rank: c_int,
                                target_disp: MPI_Aint, win: MPI_Win)
     -> c_int;
    pub fn MPI_Dims_create(nnodes: c_int, ndims: c_int,
                           dims: *mut c_int) -> c_int;
    pub fn MPI_Errhandler_c2f(errhandler: MPI_Errhandler) -> c_int;
    pub fn MPI_Errhandler_create(function: *mut MPI_Handler_function,
                                 errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Errhandler_f2c(errhandler: c_int) -> MPI_Errhandler;
    pub fn MPI_Errhandler_free(errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Errhandler_get(comm: MPI_Comm, errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Errhandler_set(comm: MPI_Comm, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn MPI_Error_class(errorcode: c_int,
                           errorclass: *mut c_int) -> c_int;
    pub fn MPI_Error_string(errorcode: c_int,
                            string: *mut c_char,
                            resultlen: *mut c_int) -> c_int;
    pub fn MPI_Exscan(sendbuf: *const c_void,
                      recvbuf: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Fetch_and_op(origin_addr: *mut c_void,
                            result_addr: *mut c_void,
                            datatype: MPI_Datatype,
                            target_rank: c_int, target_disp: MPI_Aint,
                            op: MPI_Op, win: MPI_Win) -> c_int;
    pub fn MPI_Iexscan(sendbuf: *const c_void,
                       recvbuf: *mut c_void, count: c_int,
                       datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                       request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_c2f(file: MPI_File) -> c_int;
    pub fn MPI_File_f2c(file: c_int) -> MPI_File;
    pub fn MPI_File_call_errhandler(fh: MPI_File, errorcode: c_int)
     -> c_int;
    pub fn MPI_File_create_errhandler(function:
                                          *mut MPI_File_errhandler_function,
                                      errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_File_set_errhandler(file: MPI_File, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn MPI_File_get_errhandler(file: MPI_File,
                                   errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_File_open(comm: MPI_Comm, filename: *const c_char,
                         amode: c_int, info: MPI_Info,
                         fh: *mut MPI_File) -> c_int;
    pub fn MPI_File_close(fh: *mut MPI_File) -> c_int;
    pub fn MPI_File_delete(filename: *const c_char, info: MPI_Info)
     -> c_int;
    pub fn MPI_File_set_size(fh: MPI_File, size: MPI_Offset) -> c_int;
    pub fn MPI_File_preallocate(fh: MPI_File, size: MPI_Offset)
     -> c_int;
    pub fn MPI_File_get_size(fh: MPI_File, size: *mut MPI_Offset)
     -> c_int;
    pub fn MPI_File_get_group(fh: MPI_File, group: *mut MPI_Group)
     -> c_int;
    pub fn MPI_File_get_amode(fh: MPI_File, amode: *mut c_int)
     -> c_int;
    pub fn MPI_File_set_info(fh: MPI_File, info: MPI_Info) -> c_int;
    pub fn MPI_File_get_info(fh: MPI_File, info_used: *mut MPI_Info)
     -> c_int;
    pub fn MPI_File_set_view(fh: MPI_File, disp: MPI_Offset,
                             etype: MPI_Datatype, filetype: MPI_Datatype,
                             datarep: *const c_char, info: MPI_Info)
     -> c_int;
    pub fn MPI_File_get_view(fh: MPI_File, disp: *mut MPI_Offset,
                             etype: *mut MPI_Datatype,
                             filetype: *mut MPI_Datatype,
                             datarep: *mut c_char) -> c_int;
    pub fn MPI_File_read_at(fh: MPI_File, offset: MPI_Offset,
                            buf: *mut c_void, count: c_int,
                            datatype: MPI_Datatype, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_File_read_at_all(fh: MPI_File, offset: MPI_Offset,
                                buf: *mut c_void,
                                count: c_int, datatype: MPI_Datatype,
                                status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_at(fh: MPI_File, offset: MPI_Offset,
                             buf: *const c_void, count: c_int,
                             datatype: MPI_Datatype, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_File_write_at_all(fh: MPI_File, offset: MPI_Offset,
                                 buf: *const c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_iread_at(fh: MPI_File, offset: MPI_Offset,
                             buf: *mut c_void, count: c_int,
                             datatype: MPI_Datatype,
                             request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_iwrite_at(fh: MPI_File, offset: MPI_Offset,
                              buf: *const c_void,
                              count: c_int, datatype: MPI_Datatype,
                              request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_read(fh: MPI_File, buf: *mut c_void,
                         count: c_int, datatype: MPI_Datatype,
                         status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_read_all(fh: MPI_File, buf: *mut c_void,
                             count: c_int, datatype: MPI_Datatype,
                             status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write(fh: MPI_File, buf: *const c_void,
                          count: c_int, datatype: MPI_Datatype,
                          status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_all(fh: MPI_File, buf: *const c_void,
                              count: c_int, datatype: MPI_Datatype,
                              status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_iread(fh: MPI_File, buf: *mut c_void,
                          count: c_int, datatype: MPI_Datatype,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_iwrite(fh: MPI_File, buf: *const c_void,
                           count: c_int, datatype: MPI_Datatype,
                           request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_seek(fh: MPI_File, offset: MPI_Offset,
                         whence: c_int) -> c_int;
    pub fn MPI_File_get_position(fh: MPI_File, offset: *mut MPI_Offset)
     -> c_int;
    pub fn MPI_File_get_byte_offset(fh: MPI_File, offset: MPI_Offset,
                                    disp: *mut MPI_Offset) -> c_int;
    pub fn MPI_File_read_shared(fh: MPI_File, buf: *mut c_void,
                                count: c_int, datatype: MPI_Datatype,
                                status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_shared(fh: MPI_File, buf: *const c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_iread_shared(fh: MPI_File, buf: *mut c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_iwrite_shared(fh: MPI_File, buf: *const c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  request: *mut MPI_Request) -> c_int;
    pub fn MPI_File_read_ordered(fh: MPI_File, buf: *mut c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_ordered(fh: MPI_File, buf: *const c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_seek_shared(fh: MPI_File, offset: MPI_Offset,
                                whence: c_int) -> c_int;
    pub fn MPI_File_get_position_shared(fh: MPI_File, offset: *mut MPI_Offset)
     -> c_int;
    pub fn MPI_File_read_at_all_begin(fh: MPI_File, offset: MPI_Offset,
                                      buf: *mut c_void,
                                      count: c_int,
                                      datatype: MPI_Datatype)
     -> c_int;
    pub fn MPI_File_read_at_all_end(fh: MPI_File, buf: *mut c_void,
                                    status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_at_all_begin(fh: MPI_File, offset: MPI_Offset,
                                       buf: *const c_void,
                                       count: c_int,
                                       datatype: MPI_Datatype)
     -> c_int;
    pub fn MPI_File_write_at_all_end(fh: MPI_File, buf: *const c_void,
                                     status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_File_read_all_begin(fh: MPI_File, buf: *mut c_void,
                                   count: c_int,
                                   datatype: MPI_Datatype) -> c_int;
    pub fn MPI_File_read_all_end(fh: MPI_File, buf: *mut c_void,
                                 status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_write_all_begin(fh: MPI_File, buf: *const c_void,
                                    count: c_int,
                                    datatype: MPI_Datatype) -> c_int;
    pub fn MPI_File_write_all_end(fh: MPI_File, buf: *const c_void,
                                  status: *mut MPI_Status) -> c_int;
    pub fn MPI_File_read_ordered_begin(fh: MPI_File, buf: *mut c_void,
                                       count: c_int,
                                       datatype: MPI_Datatype)
     -> c_int;
    pub fn MPI_File_read_ordered_end(fh: MPI_File, buf: *mut c_void,
                                     status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_File_write_ordered_begin(fh: MPI_File,
                                        buf: *const c_void,
                                        count: c_int,
                                        datatype: MPI_Datatype)
     -> c_int;
    pub fn MPI_File_write_ordered_end(fh: MPI_File,
                                      buf: *const c_void,
                                      status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_File_get_type_extent(fh: MPI_File, datatype: MPI_Datatype,
                                    extent: *mut MPI_Aint) -> c_int;
    pub fn MPI_File_set_atomicity(fh: MPI_File, flag: c_int)
     -> c_int;
    pub fn MPI_File_get_atomicity(fh: MPI_File, flag: *mut c_int)
     -> c_int;
    pub fn MPI_File_sync(fh: MPI_File) -> c_int;
    pub fn MPI_Finalize() -> c_int;
    pub fn MPI_Finalized(flag: *mut c_int) -> c_int;
    pub fn MPI_Free_mem(base: *mut c_void) -> c_int;
    pub fn MPI_Gather(sendbuf: *const c_void,
                      sendcount: c_int, sendtype: MPI_Datatype,
                      recvbuf: *mut c_void, recvcount: c_int,
                      recvtype: MPI_Datatype, root: c_int,
                      comm: MPI_Comm) -> c_int;
    pub fn MPI_Igather(sendbuf: *const c_void,
                       sendcount: c_int, sendtype: MPI_Datatype,
                       recvbuf: *mut c_void, recvcount: c_int,
                       recvtype: MPI_Datatype, root: c_int,
                       comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Gatherv(sendbuf: *const c_void,
                       sendcount: c_int, sendtype: MPI_Datatype,
                       recvbuf: *mut c_void,
                       recvcounts: *const c_int,
                       displs: *const c_int, recvtype: MPI_Datatype,
                       root: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Igatherv(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcounts: *const c_int,
                        displs: *const c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm,
                        request: *mut MPI_Request) -> c_int;
    pub fn MPI_Get_address(location: *const c_void,
                           address: *mut MPI_Aint) -> c_int;
    pub fn MPI_Get_count(status: *const MPI_Status, datatype: MPI_Datatype,
                         count: *mut c_int) -> c_int;
    pub fn MPI_Get_elements(status: *const MPI_Status, datatype: MPI_Datatype,
                            count: *mut c_int) -> c_int;
    pub fn MPI_Get_elements_x(status: *const MPI_Status,
                              datatype: MPI_Datatype, count: *mut MPI_Count)
     -> c_int;
    pub fn MPI_Get(origin_addr: *mut c_void,
                   origin_count: c_int, origin_datatype: MPI_Datatype,
                   target_rank: c_int, target_disp: MPI_Aint,
                   target_count: c_int, target_datatype: MPI_Datatype,
                   win: MPI_Win) -> c_int;
    pub fn MPI_Get_accumulate(origin_addr: *const c_void,
                              origin_count: c_int,
                              origin_datatype: MPI_Datatype,
                              result_addr: *mut c_void,
                              result_count: c_int,
                              result_datatype: MPI_Datatype,
                              target_rank: c_int,
                              target_disp: MPI_Aint,
                              target_count: c_int,
                              target_datatype: MPI_Datatype, op: MPI_Op,
                              win: MPI_Win) -> c_int;
    pub fn MPI_Get_library_version(version: *mut c_char,
                                   resultlen: *mut c_int)
     -> c_int;
    pub fn MPI_Get_processor_name(name: *mut c_char,
                                  resultlen: *mut c_int)
     -> c_int;
    pub fn MPI_Get_version(version: *mut c_int,
                           subversion: *mut c_int) -> c_int;
    pub fn MPI_Graph_create(comm_old: MPI_Comm, nnodes: c_int,
                            index: *const c_int,
                            edges: *const c_int,
                            reorder: c_int, comm_graph: *mut MPI_Comm)
     -> c_int;
    pub fn MPI_Graph_get(comm: MPI_Comm, maxindex: c_int,
                         maxedges: c_int, index: *mut c_int,
                         edges: *mut c_int) -> c_int;
    pub fn MPI_Graph_map(comm: MPI_Comm, nnodes: c_int,
                         index: *const c_int,
                         edges: *const c_int,
                         newrank: *mut c_int) -> c_int;
    pub fn MPI_Graph_neighbors_count(comm: MPI_Comm, rank: c_int,
                                     nneighbors: *mut c_int)
     -> c_int;
    pub fn MPI_Graph_neighbors(comm: MPI_Comm, rank: c_int,
                               maxneighbors: c_int,
                               neighbors: *mut c_int)
     -> c_int;
    pub fn MPI_Graphdims_get(comm: MPI_Comm, nnodes: *mut c_int,
                             nedges: *mut c_int) -> c_int;
    pub fn MPI_Grequest_complete(request: MPI_Request) -> c_int;
    pub fn MPI_Grequest_start(query_fn: *mut MPI_Grequest_query_function,
                              free_fn: *mut MPI_Grequest_free_function,
                              cancel_fn: *mut MPI_Grequest_cancel_function,
                              extra_state: *mut c_void,
                              request: *mut MPI_Request) -> c_int;
    pub fn MPI_Group_c2f(group: MPI_Group) -> c_int;
    pub fn MPI_Group_compare(group1: MPI_Group, group2: MPI_Group,
                             result: *mut c_int) -> c_int;
    pub fn MPI_Group_difference(group1: MPI_Group, group2: MPI_Group,
                                newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_excl(group: MPI_Group, n: c_int,
                          ranks: *const c_int,
                          newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_f2c(group: c_int) -> MPI_Group;
    pub fn MPI_Group_free(group: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_incl(group: MPI_Group, n: c_int,
                          ranks: *const c_int,
                          newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_intersection(group1: MPI_Group, group2: MPI_Group,
                                  newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_range_excl(group: MPI_Group, n: c_int,
                                ranges: *mut [c_int; 3usize],
                                newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_range_incl(group: MPI_Group, n: c_int,
                                ranges: *mut [c_int; 3usize],
                                newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Group_rank(group: MPI_Group, rank: *mut c_int)
     -> c_int;
    pub fn MPI_Group_size(group: MPI_Group, size: *mut c_int)
     -> c_int;
    pub fn MPI_Group_translate_ranks(group1: MPI_Group, n: c_int,
                                     ranks1: *const c_int,
                                     group2: MPI_Group,
                                     ranks2: *mut c_int)
     -> c_int;
    pub fn MPI_Group_union(group1: MPI_Group, group2: MPI_Group,
                           newgroup: *mut MPI_Group) -> c_int;
    pub fn MPI_Ibsend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn MPI_Improbe(source: c_int, tag: c_int,
                       comm: MPI_Comm, flag: *mut c_int,
                       message: *mut MPI_Message, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Imrecv(buf: *mut c_void, count: c_int,
                      _type: MPI_Datatype, message: *mut MPI_Message,
                      request: *mut MPI_Request) -> c_int;
    pub fn MPI_Info_c2f(info: MPI_Info) -> c_int;
    pub fn MPI_Info_create(info: *mut MPI_Info) -> c_int;
    pub fn MPI_Info_delete(info: MPI_Info, key: *const c_char)
     -> c_int;
    pub fn MPI_Info_dup(info: MPI_Info, newinfo: *mut MPI_Info)
     -> c_int;
    pub fn MPI_Info_f2c(info: c_int) -> MPI_Info;
    pub fn MPI_Info_free(info: *mut MPI_Info) -> c_int;
    pub fn MPI_Info_get(info: MPI_Info, key: *const c_char,
                        valuelen: c_int, value: *mut c_char,
                        flag: *mut c_int) -> c_int;
    pub fn MPI_Info_get_nkeys(info: MPI_Info, nkeys: *mut c_int)
     -> c_int;
    pub fn MPI_Info_get_nthkey(info: MPI_Info, n: c_int,
                               key: *mut c_char) -> c_int;
    pub fn MPI_Info_get_valuelen(info: MPI_Info, key: *const c_char,
                                 valuelen: *mut c_int,
                                 flag: *mut c_int) -> c_int;
    pub fn MPI_Info_set(info: MPI_Info, key: *const c_char,
                        value: *const c_char) -> c_int;
    pub fn MPI_Init(argc: *mut c_int,
                    argv: *mut *mut *mut c_char) -> c_int;
    pub fn MPI_Initialized(flag: *mut c_int) -> c_int;
    pub fn MPI_Init_thread(argc: *mut c_int,
                           argv: *mut *mut *mut c_char,
                           required: c_int,
                           provided: *mut c_int) -> c_int;
    pub fn MPI_Intercomm_create(local_comm: MPI_Comm,
                                local_leader: c_int,
                                bridge_comm: MPI_Comm,
                                remote_leader: c_int,
                                tag: c_int,
                                newintercomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Intercomm_merge(intercomm: MPI_Comm, high: c_int,
                               newintercomm: *mut MPI_Comm) -> c_int;
    pub fn MPI_Iprobe(source: c_int, tag: c_int,
                      comm: MPI_Comm, flag: *mut c_int,
                      status: *mut MPI_Status) -> c_int;
    pub fn MPI_Irecv(buf: *mut c_void, count: c_int,
                     datatype: MPI_Datatype, source: c_int,
                     tag: c_int, comm: MPI_Comm,
                     request: *mut MPI_Request) -> c_int;
    pub fn MPI_Irsend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn MPI_Isend(buf: *const c_void, count: c_int,
                     datatype: MPI_Datatype, dest: c_int,
                     tag: c_int, comm: MPI_Comm,
                     request: *mut MPI_Request) -> c_int;
    pub fn MPI_Issend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn MPI_Is_thread_main(flag: *mut c_int) -> c_int;
    pub fn MPI_Keyval_create(copy_fn: *mut MPI_Copy_function,
                             delete_fn: *mut MPI_Delete_function,
                             keyval: *mut c_int,
                             extra_state: *mut c_void)
     -> c_int;
    pub fn MPI_Keyval_free(keyval: *mut c_int) -> c_int;
    pub fn MPI_Lookup_name(service_name: *const c_char,
                           info: MPI_Info, port_name: *mut c_char)
     -> c_int;
    pub fn MPI_Message_c2f(message: MPI_Message) -> c_int;
    pub fn MPI_Message_f2c(message: c_int) -> MPI_Message;
    pub fn MPI_Mprobe(source: c_int, tag: c_int,
                      comm: MPI_Comm, message: *mut MPI_Message,
                      status: *mut MPI_Status) -> c_int;
    pub fn MPI_Mrecv(buf: *mut c_void, count: c_int,
                     _type: MPI_Datatype, message: *mut MPI_Message,
                     status: *mut MPI_Status) -> c_int;
    pub fn MPI_Neighbor_allgather(sendbuf: *const c_void,
                                  sendcount: c_int,
                                  sendtype: MPI_Datatype,
                                  recvbuf: *mut c_void,
                                  recvcount: c_int,
                                  recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ineighbor_allgather(sendbuf: *const c_void,
                                   sendcount: c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcount: c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm,
                                   request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Neighbor_allgatherv(sendbuf: *const c_void,
                                   sendcount: c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcounts: *const c_int,
                                   displs: *const c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ineighbor_allgatherv(sendbuf: *const c_void,
                                    sendcount: c_int,
                                    sendtype: MPI_Datatype,
                                    recvbuf: *mut c_void,
                                    recvcounts: *const c_int,
                                    displs: *const c_int,
                                    recvtype: MPI_Datatype, comm: MPI_Comm,
                                    request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Neighbor_alltoall(sendbuf: *const c_void,
                                 sendcount: c_int,
                                 sendtype: MPI_Datatype,
                                 recvbuf: *mut c_void,
                                 recvcount: c_int,
                                 recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ineighbor_alltoall(sendbuf: *const c_void,
                                  sendcount: c_int,
                                  sendtype: MPI_Datatype,
                                  recvbuf: *mut c_void,
                                  recvcount: c_int,
                                  recvtype: MPI_Datatype, comm: MPI_Comm,
                                  request: *mut MPI_Request) -> c_int;
    pub fn MPI_Neighbor_alltoallv(sendbuf: *const c_void,
                                  sendcounts: *const c_int,
                                  sdispls: *const c_int,
                                  sendtype: MPI_Datatype,
                                  recvbuf: *mut c_void,
                                  recvcounts: *const c_int,
                                  rdispls: *const c_int,
                                  recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Ineighbor_alltoallv(sendbuf: *const c_void,
                                   sendcounts: *const c_int,
                                   sdispls: *const c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcounts: *const c_int,
                                   rdispls: *const c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm,
                                   request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Neighbor_alltoallw(sendbuf: *const c_void,
                                  sendcounts: *const c_int,
                                  sdispls: *const MPI_Aint,
                                  sendtypes: *const MPI_Datatype,
                                  recvbuf: *mut c_void,
                                  recvcounts: *const c_int,
                                  rdispls: *const MPI_Aint,
                                  recvtypes: *const MPI_Datatype,
                                  comm: MPI_Comm) -> c_int;
    pub fn MPI_Ineighbor_alltoallw(sendbuf: *const c_void,
                                   sendcounts: *const c_int,
                                   sdispls: *const MPI_Aint,
                                   sendtypes: *const MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcounts: *const c_int,
                                   rdispls: *const MPI_Aint,
                                   recvtypes: *const MPI_Datatype,
                                   comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Op_c2f(op: MPI_Op) -> c_int;
    pub fn MPI_Op_commutative(op: MPI_Op, commute: *mut c_int)
     -> c_int;
    pub fn MPI_Op_create(function: *mut MPI_User_function,
                         commute: c_int, op: *mut MPI_Op)
     -> c_int;
    pub fn MPI_Open_port(info: MPI_Info, port_name: *mut c_char)
     -> c_int;
    pub fn MPI_Op_f2c(op: c_int) -> MPI_Op;
    pub fn MPI_Op_free(op: *mut MPI_Op) -> c_int;
    pub fn MPI_Pack_external(datarep: *const c_char,
                             inbuf: *const c_void,
                             incount: c_int, datatype: MPI_Datatype,
                             outbuf: *mut c_void, outsize: MPI_Aint,
                             position: *mut MPI_Aint) -> c_int;
    pub fn MPI_Pack_external_size(datarep: *const c_char,
                                  incount: c_int,
                                  datatype: MPI_Datatype, size: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Pack(inbuf: *const c_void, incount: c_int,
                    datatype: MPI_Datatype, outbuf: *mut c_void,
                    outsize: c_int, position: *mut c_int,
                    comm: MPI_Comm) -> c_int;
    pub fn MPI_Pack_size(incount: c_int, datatype: MPI_Datatype,
                         comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn MPI_Pcontrol(level: c_int, ...) -> c_int;
    pub fn MPI_Probe(source: c_int, tag: c_int,
                     comm: MPI_Comm, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Publish_name(service_name: *const c_char,
                            info: MPI_Info, port_name: *const c_char)
     -> c_int;
    pub fn MPI_Put(origin_addr: *const c_void,
                   origin_count: c_int, origin_datatype: MPI_Datatype,
                   target_rank: c_int, target_disp: MPI_Aint,
                   target_count: c_int, target_datatype: MPI_Datatype,
                   win: MPI_Win) -> c_int;
    pub fn MPI_Query_thread(provided: *mut c_int) -> c_int;
    pub fn MPI_Raccumulate(origin_addr: *mut c_void,
                           origin_count: c_int,
                           origin_datatype: MPI_Datatype,
                           target_rank: c_int, target_disp: MPI_Aint,
                           target_count: c_int,
                           target_datatype: MPI_Datatype, op: MPI_Op,
                           win: MPI_Win, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Recv_init(buf: *mut c_void, count: c_int,
                         datatype: MPI_Datatype, source: c_int,
                         tag: c_int, comm: MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn MPI_Recv(buf: *mut c_void, count: c_int,
                    datatype: MPI_Datatype, source: c_int,
                    tag: c_int, comm: MPI_Comm,
                    status: *mut MPI_Status) -> c_int;
    pub fn MPI_Reduce(sendbuf: *const c_void,
                      recvbuf: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, op: MPI_Op, root: c_int,
                      comm: MPI_Comm) -> c_int;
    pub fn MPI_Ireduce(sendbuf: *const c_void,
                       recvbuf: *mut c_void, count: c_int,
                       datatype: MPI_Datatype, op: MPI_Op,
                       root: c_int, comm: MPI_Comm,
                       request: *mut MPI_Request) -> c_int;
    pub fn MPI_Reduce_local(inbuf: *const c_void,
                            inoutbuf: *mut c_void,
                            count: c_int, datatype: MPI_Datatype,
                            op: MPI_Op) -> c_int;
    pub fn MPI_Reduce_scatter(sendbuf: *const c_void,
                              recvbuf: *mut c_void,
                              recvcounts: *const c_int,
                              datatype: MPI_Datatype, op: MPI_Op,
                              comm: MPI_Comm) -> c_int;
    pub fn MPI_Ireduce_scatter(sendbuf: *const c_void,
                               recvbuf: *mut c_void,
                               recvcounts: *const c_int,
                               datatype: MPI_Datatype, op: MPI_Op,
                               comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Reduce_scatter_block(sendbuf: *const c_void,
                                    recvbuf: *mut c_void,
                                    recvcount: c_int,
                                    datatype: MPI_Datatype, op: MPI_Op,
                                    comm: MPI_Comm) -> c_int;
    pub fn MPI_Ireduce_scatter_block(sendbuf: *const c_void,
                                     recvbuf: *mut c_void,
                                     recvcount: c_int,
                                     datatype: MPI_Datatype, op: MPI_Op,
                                     comm: MPI_Comm,
                                     request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Register_datarep(datarep: *const c_char,
                                read_conversion_fn:
                                    *mut MPI_Datarep_conversion_function,
                                write_conversion_fn:
                                    *mut MPI_Datarep_conversion_function,
                                dtype_file_extent_fn:
                                    *mut MPI_Datarep_extent_function,
                                extra_state: *mut c_void)
     -> c_int;
    pub fn MPI_Request_c2f(request: MPI_Request) -> c_int;
    pub fn MPI_Request_f2c(request: c_int) -> MPI_Request;
    pub fn MPI_Request_free(request: *mut MPI_Request) -> c_int;
    pub fn MPI_Request_get_status(request: MPI_Request,
                                  flag: *mut c_int,
                                  status: *mut MPI_Status) -> c_int;
    pub fn MPI_Rget(origin_addr: *mut c_void,
                    origin_count: c_int,
                    origin_datatype: MPI_Datatype, target_rank: c_int,
                    target_disp: MPI_Aint, target_count: c_int,
                    target_datatype: MPI_Datatype, win: MPI_Win,
                    request: *mut MPI_Request) -> c_int;
    pub fn MPI_Rget_accumulate(origin_addr: *const c_void,
                               origin_count: c_int,
                               origin_datatype: MPI_Datatype,
                               result_addr: *mut c_void,
                               result_count: c_int,
                               result_datatype: MPI_Datatype,
                               target_rank: c_int,
                               target_disp: MPI_Aint,
                               target_count: c_int,
                               target_datatype: MPI_Datatype, op: MPI_Op,
                               win: MPI_Win, request: *mut MPI_Request)
     -> c_int;
    pub fn MPI_Rput(origin_addr: *const c_void,
                    origin_count: c_int,
                    origin_datatype: MPI_Datatype, target_rank: c_int,
                    target_disp: MPI_Aint, target_cout: c_int,
                    target_datatype: MPI_Datatype, win: MPI_Win,
                    request: *mut MPI_Request) -> c_int;
    pub fn MPI_Rsend(ibuf: *const c_void, count: c_int,
                     datatype: MPI_Datatype, dest: c_int,
                     tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Rsend_init(buf: *const c_void, count: c_int,
                          datatype: MPI_Datatype, dest: c_int,
                          tag: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Scan(sendbuf: *const c_void,
                    recvbuf: *mut c_void, count: c_int,
                    datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Iscan(sendbuf: *const c_void,
                     recvbuf: *mut c_void, count: c_int,
                     datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                     request: *mut MPI_Request) -> c_int;
    pub fn MPI_Scatter(sendbuf: *const c_void,
                       sendcount: c_int, sendtype: MPI_Datatype,
                       recvbuf: *mut c_void, recvcount: c_int,
                       recvtype: MPI_Datatype, root: c_int,
                       comm: MPI_Comm) -> c_int;
    pub fn MPI_Iscatter(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm,
                        request: *mut MPI_Request) -> c_int;
    pub fn MPI_Scatterv(sendbuf: *const c_void,
                        sendcounts: *const c_int,
                        displs: *const c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Iscatterv(sendbuf: *const c_void,
                         sendcounts: *const c_int,
                         displs: *const c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         root: c_int, comm: MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn MPI_Send_init(buf: *const c_void, count: c_int,
                         datatype: MPI_Datatype, dest: c_int,
                         tag: c_int, comm: MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn MPI_Send(buf: *const c_void, count: c_int,
                    datatype: MPI_Datatype, dest: c_int,
                    tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Sendrecv(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        dest: c_int, sendtag: c_int,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        source: c_int, recvtag: c_int,
                        comm: MPI_Comm, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Sendrecv_replace(buf: *mut c_void,
                                count: c_int, datatype: MPI_Datatype,
                                dest: c_int, sendtag: c_int,
                                source: c_int, recvtag: c_int,
                                comm: MPI_Comm, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Ssend_init(buf: *const c_void, count: c_int,
                          datatype: MPI_Datatype, dest: c_int,
                          tag: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn MPI_Ssend(buf: *const c_void, count: c_int,
                     datatype: MPI_Datatype, dest: c_int,
                     tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn MPI_Start(request: *mut MPI_Request) -> c_int;
    pub fn MPI_Startall(count: c_int,
                        array_of_requests: *mut MPI_Request) -> c_int;
    pub fn MPI_Status_c2f(c_status: *const MPI_Status,
                          f_status: *mut c_int) -> c_int;
    pub fn MPI_Status_f2c(f_status: *const c_int,
                          c_status: *mut MPI_Status) -> c_int;
    pub fn MPI_Status_set_cancelled(status: *mut MPI_Status,
                                    flag: c_int) -> c_int;
    pub fn MPI_Status_set_elements(status: *mut MPI_Status,
                                   datatype: MPI_Datatype,
                                   count: c_int) -> c_int;
    pub fn MPI_Status_set_elements_x(status: *mut MPI_Status,
                                     datatype: MPI_Datatype, count: MPI_Count)
     -> c_int;
    pub fn MPI_Testall(count: c_int,
                       array_of_requests: *mut MPI_Request,
                       flag: *mut c_int,
                       array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn MPI_Testany(count: c_int,
                       array_of_requests: *mut MPI_Request,
                       index: *mut c_int, flag: *mut c_int,
                       status: *mut MPI_Status) -> c_int;
    pub fn MPI_Test(request: *mut MPI_Request, flag: *mut c_int,
                    status: *mut MPI_Status) -> c_int;
    pub fn MPI_Test_cancelled(status: *const MPI_Status,
                              flag: *mut c_int) -> c_int;
    pub fn MPI_Testsome(incount: c_int,
                        array_of_requests: *mut MPI_Request,
                        outcount: *mut c_int,
                        array_of_indices: *mut c_int,
                        array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn MPI_Topo_test(comm: MPI_Comm, status: *mut c_int)
     -> c_int;
    pub fn MPI_Type_c2f(datatype: MPI_Datatype) -> c_int;
    pub fn MPI_Type_commit(_type: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_contiguous(count: c_int, oldtype: MPI_Datatype,
                               newtype: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_create_darray(size: c_int, rank: c_int,
                                  ndims: c_int,
                                  gsize_array: *const c_int,
                                  distrib_array: *const c_int,
                                  darg_array: *const c_int,
                                  psize_array: *const c_int,
                                  order: c_int, oldtype: MPI_Datatype,
                                  newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_f90_complex(p: c_int, r: c_int,
                                       newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_f90_integer(r: c_int,
                                       newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_f90_real(p: c_int, r: c_int,
                                    newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_hindexed_block(count: c_int,
                                          blocklength: c_int,
                                          array_of_displacements:
                                              *const MPI_Aint,
                                          oldtype: MPI_Datatype,
                                          newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_hindexed(count: c_int,
                                    array_of_blocklengths:
                                        *const c_int,
                                    array_of_displacements: *const MPI_Aint,
                                    oldtype: MPI_Datatype,
                                    newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_hvector(count: c_int,
                                   blocklength: c_int,
                                   stride: MPI_Aint, oldtype: MPI_Datatype,
                                   newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_keyval(type_copy_attr_fn:
                                      *mut MPI_Type_copy_attr_function,
                                  type_delete_attr_fn:
                                      *mut MPI_Type_delete_attr_function,
                                  type_keyval: *mut c_int,
                                  extra_state: *mut c_void)
     -> c_int;
    pub fn MPI_Type_create_indexed_block(count: c_int,
                                         blocklength: c_int,
                                         array_of_displacements:
                                             *const c_int,
                                         oldtype: MPI_Datatype,
                                         newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_struct(count: c_int,
                                  array_of_block_lengths:
                                      *const c_int,
                                  array_of_displacements: *const MPI_Aint,
                                  array_of_types: *const MPI_Datatype,
                                  newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_subarray(ndims: c_int,
                                    size_array: *const c_int,
                                    subsize_array: *const c_int,
                                    start_array: *const c_int,
                                    order: c_int,
                                    oldtype: MPI_Datatype,
                                    newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_create_resized(oldtype: MPI_Datatype, lb: MPI_Aint,
                                   extent: MPI_Aint,
                                   newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_delete_attr(_type: MPI_Datatype,
                                type_keyval: c_int) -> c_int;
    pub fn MPI_Type_dup(_type: MPI_Datatype, newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_extent(_type: MPI_Datatype, extent: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Type_free(_type: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_free_keyval(type_keyval: *mut c_int)
     -> c_int;
    pub fn MPI_Type_f2c(datatype: c_int) -> MPI_Datatype;
    pub fn MPI_Type_get_attr(_type: MPI_Datatype, type_keyval: c_int,
                             attribute_val: *mut c_void,
                             flag: *mut c_int) -> c_int;
    pub fn MPI_Type_get_contents(mtype: MPI_Datatype,
                                 max_integers: c_int,
                                 max_addresses: c_int,
                                 max_datatypes: c_int,
                                 array_of_integers: *mut c_int,
                                 array_of_addresses: *mut MPI_Aint,
                                 array_of_datatypes: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_get_envelope(_type: MPI_Datatype,
                                 num_integers: *mut c_int,
                                 num_addresses: *mut c_int,
                                 num_datatypes: *mut c_int,
                                 combiner: *mut c_int)
     -> c_int;
    pub fn MPI_Type_get_extent(_type: MPI_Datatype, lb: *mut MPI_Aint,
                               extent: *mut MPI_Aint) -> c_int;
    pub fn MPI_Type_get_extent_x(_type: MPI_Datatype, lb: *mut MPI_Count,
                                 extent: *mut MPI_Count) -> c_int;
    pub fn MPI_Type_get_name(_type: MPI_Datatype,
                             type_name: *mut c_char,
                             resultlen: *mut c_int) -> c_int;
    pub fn MPI_Type_get_true_extent(datatype: MPI_Datatype,
                                    true_lb: *mut MPI_Aint,
                                    true_extent: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Type_get_true_extent_x(datatype: MPI_Datatype,
                                      true_lb: *mut MPI_Count,
                                      true_extent: *mut MPI_Count)
     -> c_int;
    pub fn MPI_Type_hindexed(count: c_int,
                             array_of_blocklengths: *mut c_int,
                             array_of_displacements: *mut MPI_Aint,
                             oldtype: MPI_Datatype,
                             newtype: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_hvector(count: c_int, blocklength: c_int,
                            stride: MPI_Aint, oldtype: MPI_Datatype,
                            newtype: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_indexed(count: c_int,
                            array_of_blocklengths: *const c_int,
                            array_of_displacements: *const c_int,
                            oldtype: MPI_Datatype, newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn MPI_Type_lb(_type: MPI_Datatype, lb: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Type_match_size(typeclass: c_int, size: c_int,
                               _type: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_set_attr(_type: MPI_Datatype, type_keyval: c_int,
                             attr_val: *mut c_void) -> c_int;
    pub fn MPI_Type_set_name(_type: MPI_Datatype,
                             type_name: *const c_char)
     -> c_int;
    pub fn MPI_Type_size(_type: MPI_Datatype, size: *mut c_int)
     -> c_int;
    pub fn MPI_Type_size_x(_type: MPI_Datatype, size: *mut MPI_Count)
     -> c_int;
    pub fn MPI_Type_struct(count: c_int,
                           array_of_blocklengths: *mut c_int,
                           array_of_displacements: *mut MPI_Aint,
                           array_of_types: *mut MPI_Datatype,
                           newtype: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Type_ub(mtype: MPI_Datatype, ub: *mut MPI_Aint)
     -> c_int;
    pub fn MPI_Type_vector(count: c_int, blocklength: c_int,
                           stride: c_int, oldtype: MPI_Datatype,
                           newtype: *mut MPI_Datatype) -> c_int;
    pub fn MPI_Unpack(inbuf: *const c_void, insize: c_int,
                      position: *mut c_int,
                      outbuf: *mut c_void, outcount: c_int,
                      datatype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn MPI_Unpublish_name(service_name: *const c_char,
                              info: MPI_Info,
                              port_name: *const c_char)
     -> c_int;
    pub fn MPI_Unpack_external(datarep: *const c_char,
                               inbuf: *const c_void, insize: MPI_Aint,
                               position: *mut MPI_Aint,
                               outbuf: *mut c_void,
                               outcount: c_int,
                               datatype: MPI_Datatype) -> c_int;
    pub fn MPI_Waitall(count: c_int,
                       array_of_requests: *mut MPI_Request,
                       array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn MPI_Waitany(count: c_int,
                       array_of_requests: *mut MPI_Request,
                       index: *mut c_int, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Wait(request: *mut MPI_Request, status: *mut MPI_Status)
     -> c_int;
    pub fn MPI_Waitsome(incount: c_int,
                        array_of_requests: *mut MPI_Request,
                        outcount: *mut c_int,
                        array_of_indices: *mut c_int,
                        array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn MPI_Win_allocate(size: MPI_Aint, disp_unit: c_int,
                            info: MPI_Info, comm: MPI_Comm,
                            baseptr: *mut c_void, win: *mut MPI_Win)
     -> c_int;
    pub fn MPI_Win_allocate_shared(size: MPI_Aint, disp_unit: c_int,
                                   info: MPI_Info, comm: MPI_Comm,
                                   baseptr: *mut c_void,
                                   win: *mut MPI_Win) -> c_int;
    pub fn MPI_Win_attach(win: MPI_Win, base: *mut c_void,
                          size: MPI_Aint) -> c_int;
    pub fn MPI_Win_c2f(win: MPI_Win) -> c_int;
    pub fn MPI_Win_call_errhandler(win: MPI_Win, errorcode: c_int)
     -> c_int;
    pub fn MPI_Win_complete(win: MPI_Win) -> c_int;
    pub fn MPI_Win_create(base: *mut c_void, size: MPI_Aint,
                          disp_unit: c_int, info: MPI_Info,
                          comm: MPI_Comm, win: *mut MPI_Win) -> c_int;
    pub fn MPI_Win_create_dynamic(info: MPI_Info, comm: MPI_Comm,
                                  win: *mut MPI_Win) -> c_int;
    pub fn MPI_Win_create_errhandler(function:
                                         *mut MPI_Win_errhandler_function,
                                     errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Win_create_keyval(win_copy_attr_fn:
                                     *mut MPI_Win_copy_attr_function,
                                 win_delete_attr_fn:
                                     *mut MPI_Win_delete_attr_function,
                                 win_keyval: *mut c_int,
                                 extra_state: *mut c_void)
     -> c_int;
    pub fn MPI_Win_delete_attr(win: MPI_Win, win_keyval: c_int)
     -> c_int;
    pub fn MPI_Win_detach(win: MPI_Win, base: *mut c_void)
     -> c_int;
    pub fn MPI_Win_f2c(win: c_int) -> MPI_Win;
    pub fn MPI_Win_fence(assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn MPI_Win_flush(rank: c_int, win: MPI_Win) -> c_int;
    pub fn MPI_Win_flush_all(win: MPI_Win) -> c_int;
    pub fn MPI_Win_flush_local(rank: c_int, win: MPI_Win)
     -> c_int;
    pub fn MPI_Win_flush_local_all(win: MPI_Win) -> c_int;
    pub fn MPI_Win_free(win: *mut MPI_Win) -> c_int;
    pub fn MPI_Win_free_keyval(win_keyval: *mut c_int)
     -> c_int;
    pub fn MPI_Win_get_attr(win: MPI_Win, win_keyval: c_int,
                            attribute_val: *mut c_void,
                            flag: *mut c_int) -> c_int;
    pub fn MPI_Win_get_errhandler(win: MPI_Win,
                                  errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn MPI_Win_get_group(win: MPI_Win, group: *mut MPI_Group)
     -> c_int;
    pub fn MPI_Win_get_info(win: MPI_Win, info_used: *mut MPI_Info)
     -> c_int;
    pub fn MPI_Win_get_name(win: MPI_Win, win_name: *mut c_char,
                            resultlen: *mut c_int) -> c_int;
    pub fn MPI_Win_lock(lock_type: c_int, rank: c_int,
                        assert: c_int, win: MPI_Win) -> c_int;
    pub fn MPI_Win_lock_all(assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn MPI_Win_post(group: MPI_Group, assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn MPI_Win_set_attr(win: MPI_Win, win_keyval: c_int,
                            attribute_val: *mut c_void)
     -> c_int;
    pub fn MPI_Win_set_errhandler(win: MPI_Win, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn MPI_Win_set_info(win: MPI_Win, info: MPI_Info) -> c_int;
    pub fn MPI_Win_set_name(win: MPI_Win, win_name: *const c_char)
     -> c_int;
    pub fn MPI_Win_shared_query(win: MPI_Win, rank: c_int,
                                size: *mut MPI_Aint,
                                disp_unit: *mut c_int,
                                baseptr: *mut c_void)
     -> c_int;
    pub fn MPI_Win_start(group: MPI_Group, assert: c_int,
                         win: MPI_Win) -> c_int;
    pub fn MPI_Win_sync(win: MPI_Win) -> c_int;
    pub fn MPI_Win_test(win: MPI_Win, flag: *mut c_int)
     -> c_int;
    pub fn MPI_Win_unlock(rank: c_int, win: MPI_Win) -> c_int;
    pub fn MPI_Win_unlock_all(win: MPI_Win) -> c_int;
    pub fn MPI_Win_wait(win: MPI_Win) -> c_int;
    pub fn MPI_Wtick() -> c_double;
    pub fn MPI_Wtime() -> c_double;
    pub fn PMPI_Abort(comm: MPI_Comm, errorcode: c_int)
     -> c_int;
    pub fn PMPI_Accumulate(origin_addr: *const c_void,
                           origin_count: c_int,
                           origin_datatype: MPI_Datatype,
                           target_rank: c_int, target_disp: MPI_Aint,
                           target_count: c_int,
                           target_datatype: MPI_Datatype, op: MPI_Op,
                           win: MPI_Win) -> c_int;
    pub fn PMPI_Add_error_class(errorclass: *mut c_int)
     -> c_int;
    pub fn PMPI_Add_error_code(errorclass: c_int,
                               errorcode: *mut c_int)
     -> c_int;
    pub fn PMPI_Add_error_string(errorcode: c_int,
                                 string: *const c_char)
     -> c_int;
    pub fn PMPI_Address(location: *mut c_void, address: *mut MPI_Aint)
     -> c_int;
    pub fn PMPI_Allgather(sendbuf: *const c_void,
                          sendcount: c_int, sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcount: c_int, recvtype: MPI_Datatype,
                          comm: MPI_Comm) -> c_int;
    pub fn PMPI_Iallgather(sendbuf: *const c_void,
                           sendcount: c_int, sendtype: MPI_Datatype,
                           recvbuf: *mut c_void,
                           recvcount: c_int, recvtype: MPI_Datatype,
                           comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Allgatherv(sendbuf: *const c_void,
                           sendcount: c_int, sendtype: MPI_Datatype,
                           recvbuf: *mut c_void,
                           recvcounts: *const c_int,
                           displs: *const c_int,
                           recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Iallgatherv(sendbuf: *const c_void,
                            sendcount: c_int, sendtype: MPI_Datatype,
                            recvbuf: *mut c_void,
                            recvcounts: *const c_int,
                            displs: *const c_int,
                            recvtype: MPI_Datatype, comm: MPI_Comm,
                            request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Alloc_mem(size: MPI_Aint, info: MPI_Info,
                          baseptr: *mut c_void) -> c_int;
    pub fn PMPI_Allreduce(sendbuf: *const c_void,
                          recvbuf: *mut c_void, count: c_int,
                          datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Iallreduce(sendbuf: *const c_void,
                           recvbuf: *mut c_void, count: c_int,
                           datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Alltoall(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ialltoall(sendbuf: *const c_void,
                          sendcount: c_int, sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcount: c_int, recvtype: MPI_Datatype,
                          comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Alltoallv(sendbuf: *const c_void,
                          sendcounts: *const c_int,
                          sdispls: *const c_int,
                          sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcounts: *const c_int,
                          rdispls: *const c_int,
                          recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ialltoallv(sendbuf: *const c_void,
                           sendcounts: *const c_int,
                           sdispls: *const c_int,
                           sendtype: MPI_Datatype,
                           recvbuf: *mut c_void,
                           recvcounts: *const c_int,
                           rdispls: *const c_int,
                           recvtype: MPI_Datatype, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Alltoallw(sendbuf: *const c_void,
                          sendcounts: *const c_int,
                          sdispls: *const c_int,
                          sendtypes: *const MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcounts: *const c_int,
                          rdispls: *const c_int,
                          recvtypes: *const MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ialltoallw(sendbuf: *const c_void,
                           sendcounts: *const c_int,
                           sdispls: *const c_int,
                           sendtypes: *const MPI_Datatype,
                           recvbuf: *mut c_void,
                           recvcounts: *const c_int,
                           rdispls: *const c_int,
                           recvtypes: *const MPI_Datatype, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Attr_delete(comm: MPI_Comm, keyval: c_int)
     -> c_int;
    pub fn PMPI_Attr_get(comm: MPI_Comm, keyval: c_int,
                         attribute_val: *mut c_void,
                         flag: *mut c_int) -> c_int;
    pub fn PMPI_Dist_graph_create(comm_old: MPI_Comm, n: c_int,
                                  nodes: *const c_int,
                                  degrees: *const c_int,
                                  targets: *const c_int,
                                  weights: *const c_int,
                                  info: MPI_Info, reorder: c_int,
                                  newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Dist_graph_create_adjacent(comm_old: MPI_Comm,
                                           indegree: c_int,
                                           sources: *const c_int,
                                           sourceweights:
                                               *const c_int,
                                           outdegree: c_int,
                                           destinations: *const c_int,
                                           destweights: *const c_int,
                                           info: MPI_Info,
                                           reorder: c_int,
                                           comm_dist_graph: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Dist_graph_neighbors(comm: MPI_Comm,
                                     maxindegree: c_int,
                                     sources: *mut c_int,
                                     sourceweights: *mut c_int,
                                     maxoutdegree: c_int,
                                     destinations: *mut c_int,
                                     destweights: *mut c_int)
     -> c_int;
    pub fn PMPI_Dist_graph_neighbors_count(comm: MPI_Comm,
                                           inneighbors: *mut c_int,
                                           outneighbors: *mut c_int,
                                           weighted: *mut c_int)
     -> c_int;
    pub fn PMPI_Attr_put(comm: MPI_Comm, keyval: c_int,
                         attribute_val: *mut c_void) -> c_int;
    pub fn PMPI_Barrier(comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ibarrier(comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Bcast(buffer: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, root: c_int,
                      comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ibcast(buffer: *mut c_void, count: c_int,
                       datatype: MPI_Datatype, root: c_int,
                       comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Bsend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Bsend_init(buf: *const c_void, count: c_int,
                           datatype: MPI_Datatype, dest: c_int,
                           tag: c_int, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Buffer_attach(buffer: *mut c_void,
                              size: c_int) -> c_int;
    pub fn PMPI_Buffer_detach(buffer: *mut c_void,
                              size: *mut c_int) -> c_int;
    pub fn PMPI_Cancel(request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Cart_coords(comm: MPI_Comm, rank: c_int,
                            maxdims: c_int,
                            coords: *mut c_int) -> c_int;
    pub fn PMPI_Cart_create(old_comm: MPI_Comm, ndims: c_int,
                            dims: *const c_int,
                            periods: *const c_int,
                            reorder: c_int, comm_cart: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Cart_get(comm: MPI_Comm, maxdims: c_int,
                         dims: *mut c_int,
                         periods: *mut c_int,
                         coords: *mut c_int) -> c_int;
    pub fn PMPI_Cart_map(comm: MPI_Comm, ndims: c_int,
                         dims: *const c_int,
                         periods: *const c_int,
                         newrank: *mut c_int) -> c_int;
    pub fn PMPI_Cart_rank(comm: MPI_Comm, coords: *const c_int,
                          rank: *mut c_int) -> c_int;
    pub fn PMPI_Cart_shift(comm: MPI_Comm, direction: c_int,
                           disp: c_int,
                           rank_source: *mut c_int,
                           rank_dest: *mut c_int) -> c_int;
    pub fn PMPI_Cart_sub(comm: MPI_Comm, remain_dims: *const c_int,
                         new_comm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Cartdim_get(comm: MPI_Comm, ndims: *mut c_int)
     -> c_int;
    pub fn PMPI_Close_port(port_name: *const c_char) -> c_int;
    pub fn PMPI_Comm_accept(port_name: *const c_char, info: MPI_Info,
                            root: c_int, comm: MPI_Comm,
                            newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_c2f(comm: MPI_Comm) -> c_int;
    pub fn PMPI_Comm_call_errhandler(comm: MPI_Comm, errorcode: c_int)
     -> c_int;
    pub fn PMPI_Comm_compare(comm1: MPI_Comm, comm2: MPI_Comm,
                             result: *mut c_int) -> c_int;
    pub fn PMPI_Comm_connect(port_name: *const c_char, info: MPI_Info,
                             root: c_int, comm: MPI_Comm,
                             newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_create_errhandler(function:
                                           *mut MPI_Comm_errhandler_function,
                                       errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Comm_create_keyval(comm_copy_attr_fn:
                                       *mut MPI_Comm_copy_attr_function,
                                   comm_delete_attr_fn:
                                       *mut MPI_Comm_delete_attr_function,
                                   comm_keyval: *mut c_int,
                                   extra_state: *mut c_void)
     -> c_int;
    pub fn PMPI_Comm_create_group(comm: MPI_Comm, group: MPI_Group,
                                  tag: c_int, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Comm_create(comm: MPI_Comm, group: MPI_Group,
                            newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_delete_attr(comm: MPI_Comm, comm_keyval: c_int)
     -> c_int;
    pub fn PMPI_Comm_disconnect(comm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_dup(comm: MPI_Comm, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Comm_idup(comm: MPI_Comm, newcomm: *mut MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Comm_dup_with_info(comm: MPI_Comm, info: MPI_Info,
                                   newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_f2c(comm: c_int) -> MPI_Comm;
    pub fn PMPI_Comm_free_keyval(comm_keyval: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_free(comm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_get_attr(comm: MPI_Comm, comm_keyval: c_int,
                              attribute_val: *mut c_void,
                              flag: *mut c_int) -> c_int;
    pub fn PMPI_Comm_get_errhandler(comm: MPI_Comm,
                                    erhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Comm_get_info(comm: MPI_Comm, info_used: *mut MPI_Info)
     -> c_int;
    pub fn PMPI_Comm_get_name(comm: MPI_Comm, comm_name: *mut c_char,
                              resultlen: *mut c_int) -> c_int;
    pub fn PMPI_Comm_get_parent(parent: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_group(comm: MPI_Comm, group: *mut MPI_Group)
     -> c_int;
    pub fn PMPI_Comm_join(fd: c_int, intercomm: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Comm_rank(comm: MPI_Comm, rank: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_remote_group(comm: MPI_Comm, group: *mut MPI_Group)
     -> c_int;
    pub fn PMPI_Comm_remote_size(comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_set_attr(comm: MPI_Comm, comm_keyval: c_int,
                              attribute_val: *mut c_void)
     -> c_int;
    pub fn PMPI_Comm_set_errhandler(comm: MPI_Comm,
                                    errhandler: MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Comm_set_info(comm: MPI_Comm, info: MPI_Info)
     -> c_int;
    pub fn PMPI_Comm_set_name(comm: MPI_Comm,
                              comm_name: *const c_char)
     -> c_int;
    pub fn PMPI_Comm_size(comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_spawn(command: *const c_char,
                           argv: *mut *mut c_char,
                           maxprocs: c_int, info: MPI_Info,
                           root: c_int, comm: MPI_Comm,
                           intercomm: *mut MPI_Comm,
                           array_of_errcodes: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_spawn_multiple(count: c_int,
                                    array_of_commands:
                                        *mut *mut c_char,
                                    array_of_argv:
                                        *mut *mut *mut c_char,
                                    array_of_maxprocs: *const c_int,
                                    array_of_info: *const MPI_Info,
                                    root: c_int, comm: MPI_Comm,
                                    intercomm: *mut MPI_Comm,
                                    array_of_errcodes: *mut c_int)
     -> c_int;
    pub fn PMPI_Comm_split(comm: MPI_Comm, color: c_int,
                           key: c_int, newcomm: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Comm_split_type(comm: MPI_Comm, split_type: c_int,
                                key: c_int, info: MPI_Info,
                                newcomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Comm_test_inter(comm: MPI_Comm, flag: *mut c_int)
     -> c_int;
    pub fn PMPI_Compare_and_swap(origin_addr: *mut c_void,
                                 compare_addr: *mut c_void,
                                 result_addr: *mut c_void,
                                 datatype: MPI_Datatype,
                                 target_rank: c_int,
                                 target_disp: MPI_Aint, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Dims_create(nnodes: c_int, ndims: c_int,
                            dims: *mut c_int) -> c_int;
    pub fn PMPI_Errhandler_c2f(errhandler: MPI_Errhandler) -> c_int;
    pub fn PMPI_Errhandler_create(function: *mut MPI_Handler_function,
                                  errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Errhandler_f2c(errhandler: c_int) -> MPI_Errhandler;
    pub fn PMPI_Errhandler_free(errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Errhandler_get(comm: MPI_Comm,
                               errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Errhandler_set(comm: MPI_Comm, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Error_class(errorcode: c_int,
                            errorclass: *mut c_int) -> c_int;
    pub fn PMPI_Error_string(errorcode: c_int,
                             string: *mut c_char,
                             resultlen: *mut c_int) -> c_int;
    pub fn PMPI_Exscan(sendbuf: *const c_void,
                       recvbuf: *mut c_void, count: c_int,
                       datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Fetch_and_op(origin_addr: *mut c_void,
                             result_addr: *mut c_void,
                             datatype: MPI_Datatype,
                             target_rank: c_int,
                             target_disp: MPI_Aint, op: MPI_Op, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Iexscan(sendbuf: *const c_void,
                        recvbuf: *mut c_void, count: c_int,
                        datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                        request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_c2f(file: MPI_File) -> c_int;
    pub fn PMPI_File_f2c(file: c_int) -> MPI_File;
    pub fn PMPI_File_call_errhandler(fh: MPI_File, errorcode: c_int)
     -> c_int;
    pub fn PMPI_File_create_errhandler(function:
                                           *mut MPI_File_errhandler_function,
                                       errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_File_set_errhandler(file: MPI_File,
                                    errhandler: MPI_Errhandler)
     -> c_int;
    pub fn PMPI_File_get_errhandler(file: MPI_File,
                                    errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_File_open(comm: MPI_Comm, filename: *const c_char,
                          amode: c_int, info: MPI_Info,
                          fh: *mut MPI_File) -> c_int;
    pub fn PMPI_File_close(fh: *mut MPI_File) -> c_int;
    pub fn PMPI_File_delete(filename: *const c_char, info: MPI_Info)
     -> c_int;
    pub fn PMPI_File_set_size(fh: MPI_File, size: MPI_Offset)
     -> c_int;
    pub fn PMPI_File_preallocate(fh: MPI_File, size: MPI_Offset)
     -> c_int;
    pub fn PMPI_File_get_size(fh: MPI_File, size: *mut MPI_Offset)
     -> c_int;
    pub fn PMPI_File_get_group(fh: MPI_File, group: *mut MPI_Group)
     -> c_int;
    pub fn PMPI_File_get_amode(fh: MPI_File, amode: *mut c_int)
     -> c_int;
    pub fn PMPI_File_set_info(fh: MPI_File, info: MPI_Info) -> c_int;
    pub fn PMPI_File_get_info(fh: MPI_File, info_used: *mut MPI_Info)
     -> c_int;
    pub fn PMPI_File_set_view(fh: MPI_File, disp: MPI_Offset,
                              etype: MPI_Datatype, filetype: MPI_Datatype,
                              datarep: *const c_char, info: MPI_Info)
     -> c_int;
    pub fn PMPI_File_get_view(fh: MPI_File, disp: *mut MPI_Offset,
                              etype: *mut MPI_Datatype,
                              filetype: *mut MPI_Datatype,
                              datarep: *mut c_char) -> c_int;
    pub fn PMPI_File_read_at(fh: MPI_File, offset: MPI_Offset,
                             buf: *mut c_void, count: c_int,
                             datatype: MPI_Datatype, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_File_read_at_all(fh: MPI_File, offset: MPI_Offset,
                                 buf: *mut c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_at(fh: MPI_File, offset: MPI_Offset,
                              buf: *const c_void,
                              count: c_int, datatype: MPI_Datatype,
                              status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_at_all(fh: MPI_File, offset: MPI_Offset,
                                  buf: *const c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_iread_at(fh: MPI_File, offset: MPI_Offset,
                              buf: *mut c_void, count: c_int,
                              datatype: MPI_Datatype,
                              request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_iwrite_at(fh: MPI_File, offset: MPI_Offset,
                               buf: *const c_void,
                               count: c_int, datatype: MPI_Datatype,
                               request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_read(fh: MPI_File, buf: *mut c_void,
                          count: c_int, datatype: MPI_Datatype,
                          status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_read_all(fh: MPI_File, buf: *mut c_void,
                              count: c_int, datatype: MPI_Datatype,
                              status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write(fh: MPI_File, buf: *const c_void,
                           count: c_int, datatype: MPI_Datatype,
                           status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_all(fh: MPI_File, buf: *const c_void,
                               count: c_int, datatype: MPI_Datatype,
                               status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_iread(fh: MPI_File, buf: *mut c_void,
                           count: c_int, datatype: MPI_Datatype,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_iwrite(fh: MPI_File, buf: *const c_void,
                            count: c_int, datatype: MPI_Datatype,
                            request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_seek(fh: MPI_File, offset: MPI_Offset,
                          whence: c_int) -> c_int;
    pub fn PMPI_File_get_position(fh: MPI_File, offset: *mut MPI_Offset)
     -> c_int;
    pub fn PMPI_File_get_byte_offset(fh: MPI_File, offset: MPI_Offset,
                                     disp: *mut MPI_Offset) -> c_int;
    pub fn PMPI_File_read_shared(fh: MPI_File, buf: *mut c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_shared(fh: MPI_File, buf: *const c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_iread_shared(fh: MPI_File, buf: *mut c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  request: *mut MPI_Request) -> c_int;
    pub fn PMPI_File_iwrite_shared(fh: MPI_File, buf: *const c_void,
                                   count: c_int,
                                   datatype: MPI_Datatype,
                                   request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_File_read_ordered(fh: MPI_File, buf: *mut c_void,
                                  count: c_int,
                                  datatype: MPI_Datatype,
                                  status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_ordered(fh: MPI_File, buf: *const c_void,
                                   count: c_int,
                                   datatype: MPI_Datatype,
                                   status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_seek_shared(fh: MPI_File, offset: MPI_Offset,
                                 whence: c_int) -> c_int;
    pub fn PMPI_File_get_position_shared(fh: MPI_File,
                                         offset: *mut MPI_Offset)
     -> c_int;
    pub fn PMPI_File_read_at_all_begin(fh: MPI_File, offset: MPI_Offset,
                                       buf: *mut c_void,
                                       count: c_int,
                                       datatype: MPI_Datatype)
     -> c_int;
    pub fn PMPI_File_read_at_all_end(fh: MPI_File, buf: *mut c_void,
                                     status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_File_write_at_all_begin(fh: MPI_File, offset: MPI_Offset,
                                        buf: *const c_void,
                                        count: c_int,
                                        datatype: MPI_Datatype)
     -> c_int;
    pub fn PMPI_File_write_at_all_end(fh: MPI_File,
                                      buf: *const c_void,
                                      status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_File_read_all_begin(fh: MPI_File, buf: *mut c_void,
                                    count: c_int,
                                    datatype: MPI_Datatype) -> c_int;
    pub fn PMPI_File_read_all_end(fh: MPI_File, buf: *mut c_void,
                                  status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_write_all_begin(fh: MPI_File, buf: *const c_void,
                                     count: c_int,
                                     datatype: MPI_Datatype) -> c_int;
    pub fn PMPI_File_write_all_end(fh: MPI_File, buf: *const c_void,
                                   status: *mut MPI_Status) -> c_int;
    pub fn PMPI_File_read_ordered_begin(fh: MPI_File,
                                        buf: *mut c_void,
                                        count: c_int,
                                        datatype: MPI_Datatype)
     -> c_int;
    pub fn PMPI_File_read_ordered_end(fh: MPI_File, buf: *mut c_void,
                                      status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_File_write_ordered_begin(fh: MPI_File,
                                         buf: *const c_void,
                                         count: c_int,
                                         datatype: MPI_Datatype)
     -> c_int;
    pub fn PMPI_File_write_ordered_end(fh: MPI_File,
                                       buf: *const c_void,
                                       status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_File_get_type_extent(fh: MPI_File, datatype: MPI_Datatype,
                                     extent: *mut MPI_Aint) -> c_int;
    pub fn PMPI_File_set_atomicity(fh: MPI_File, flag: c_int)
     -> c_int;
    pub fn PMPI_File_get_atomicity(fh: MPI_File, flag: *mut c_int)
     -> c_int;
    pub fn PMPI_File_sync(fh: MPI_File) -> c_int;
    pub fn PMPI_Finalize() -> c_int;
    pub fn PMPI_Finalized(flag: *mut c_int) -> c_int;
    pub fn PMPI_Free_mem(base: *mut c_void) -> c_int;
    pub fn PMPI_Gather(sendbuf: *const c_void,
                       sendcount: c_int, sendtype: MPI_Datatype,
                       recvbuf: *mut c_void, recvcount: c_int,
                       recvtype: MPI_Datatype, root: c_int,
                       comm: MPI_Comm) -> c_int;
    pub fn PMPI_Igather(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm,
                        request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Gatherv(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcounts: *const c_int,
                        displs: *const c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Igatherv(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcounts: *const c_int,
                         displs: *const c_int, recvtype: MPI_Datatype,
                         root: c_int, comm: MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Get_address(location: *const c_void,
                            address: *mut MPI_Aint) -> c_int;
    pub fn PMPI_Get_count(status: *const MPI_Status, datatype: MPI_Datatype,
                          count: *mut c_int) -> c_int;
    pub fn PMPI_Get_elements(status: *const MPI_Status,
                             datatype: MPI_Datatype,
                             count: *mut c_int) -> c_int;
    pub fn PMPI_Get_elements_x(status: *const MPI_Status,
                               datatype: MPI_Datatype, count: *mut MPI_Count)
     -> c_int;
    pub fn PMPI_Get(origin_addr: *mut c_void,
                    origin_count: c_int,
                    origin_datatype: MPI_Datatype, target_rank: c_int,
                    target_disp: MPI_Aint, target_count: c_int,
                    target_datatype: MPI_Datatype, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Get_accumulate(origin_addr: *const c_void,
                               origin_count: c_int,
                               origin_datatype: MPI_Datatype,
                               result_addr: *mut c_void,
                               result_count: c_int,
                               result_datatype: MPI_Datatype,
                               target_rank: c_int,
                               target_disp: MPI_Aint,
                               target_count: c_int,
                               target_datatype: MPI_Datatype, op: MPI_Op,
                               win: MPI_Win) -> c_int;
    pub fn PMPI_Get_library_version(version: *mut c_char,
                                    resultlen: *mut c_int)
     -> c_int;
    pub fn PMPI_Get_processor_name(name: *mut c_char,
                                   resultlen: *mut c_int)
     -> c_int;
    pub fn PMPI_Get_version(version: *mut c_int,
                            subversion: *mut c_int) -> c_int;
    pub fn PMPI_Graph_create(comm_old: MPI_Comm, nnodes: c_int,
                             index: *const c_int,
                             edges: *const c_int,
                             reorder: c_int,
                             comm_graph: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Graph_get(comm: MPI_Comm, maxindex: c_int,
                          maxedges: c_int, index: *mut c_int,
                          edges: *mut c_int) -> c_int;
    pub fn PMPI_Graph_map(comm: MPI_Comm, nnodes: c_int,
                          index: *const c_int,
                          edges: *const c_int,
                          newrank: *mut c_int) -> c_int;
    pub fn PMPI_Graph_neighbors_count(comm: MPI_Comm, rank: c_int,
                                      nneighbors: *mut c_int)
     -> c_int;
    pub fn PMPI_Graph_neighbors(comm: MPI_Comm, rank: c_int,
                                maxneighbors: c_int,
                                neighbors: *mut c_int)
     -> c_int;
    pub fn PMPI_Graphdims_get(comm: MPI_Comm, nnodes: *mut c_int,
                              nedges: *mut c_int) -> c_int;
    pub fn PMPI_Grequest_complete(request: MPI_Request) -> c_int;
    pub fn PMPI_Grequest_start(query_fn: *mut MPI_Grequest_query_function,
                               free_fn: *mut MPI_Grequest_free_function,
                               cancel_fn: *mut MPI_Grequest_cancel_function,
                               extra_state: *mut c_void,
                               request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Group_c2f(group: MPI_Group) -> c_int;
    pub fn PMPI_Group_compare(group1: MPI_Group, group2: MPI_Group,
                              result: *mut c_int) -> c_int;
    pub fn PMPI_Group_difference(group1: MPI_Group, group2: MPI_Group,
                                 newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_excl(group: MPI_Group, n: c_int,
                           ranks: *const c_int,
                           newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_f2c(group: c_int) -> MPI_Group;
    pub fn PMPI_Group_free(group: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_incl(group: MPI_Group, n: c_int,
                           ranks: *const c_int,
                           newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_intersection(group1: MPI_Group, group2: MPI_Group,
                                   newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_range_excl(group: MPI_Group, n: c_int,
                                 ranges: *mut [c_int; 3usize],
                                 newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_range_incl(group: MPI_Group, n: c_int,
                                 ranges: *mut [c_int; 3usize],
                                 newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Group_rank(group: MPI_Group, rank: *mut c_int)
     -> c_int;
    pub fn PMPI_Group_size(group: MPI_Group, size: *mut c_int)
     -> c_int;
    pub fn PMPI_Group_translate_ranks(group1: MPI_Group, n: c_int,
                                      ranks1: *const c_int,
                                      group2: MPI_Group,
                                      ranks2: *mut c_int)
     -> c_int;
    pub fn PMPI_Group_union(group1: MPI_Group, group2: MPI_Group,
                            newgroup: *mut MPI_Group) -> c_int;
    pub fn PMPI_Ibsend(buf: *const c_void, count: c_int,
                       datatype: MPI_Datatype, dest: c_int,
                       tag: c_int, comm: MPI_Comm,
                       request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Improbe(source: c_int, tag: c_int,
                        comm: MPI_Comm, flag: *mut c_int,
                        message: *mut MPI_Message, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_Imrecv(buf: *mut c_void, count: c_int,
                       _type: MPI_Datatype, message: *mut MPI_Message,
                       request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Info_c2f(info: MPI_Info) -> c_int;
    pub fn PMPI_Info_create(info: *mut MPI_Info) -> c_int;
    pub fn PMPI_Info_delete(info: MPI_Info, key: *const c_char)
     -> c_int;
    pub fn PMPI_Info_dup(info: MPI_Info, newinfo: *mut MPI_Info)
     -> c_int;
    pub fn PMPI_Info_f2c(info: c_int) -> MPI_Info;
    pub fn PMPI_Info_free(info: *mut MPI_Info) -> c_int;
    pub fn PMPI_Info_get(info: MPI_Info, key: *const c_char,
                         valuelen: c_int, value: *mut c_char,
                         flag: *mut c_int) -> c_int;
    pub fn PMPI_Info_get_nkeys(info: MPI_Info, nkeys: *mut c_int)
     -> c_int;
    pub fn PMPI_Info_get_nthkey(info: MPI_Info, n: c_int,
                                key: *mut c_char) -> c_int;
    pub fn PMPI_Info_get_valuelen(info: MPI_Info, key: *const c_char,
                                  valuelen: *mut c_int,
                                  flag: *mut c_int) -> c_int;
    pub fn PMPI_Info_set(info: MPI_Info, key: *const c_char,
                         value: *const c_char) -> c_int;
    pub fn PMPI_Init(argc: *mut c_int,
                     argv: *mut *mut *mut c_char) -> c_int;
    pub fn PMPI_Initialized(flag: *mut c_int) -> c_int;
    pub fn PMPI_Init_thread(argc: *mut c_int,
                            argv: *mut *mut *mut c_char,
                            required: c_int,
                            provided: *mut c_int) -> c_int;
    pub fn PMPI_Intercomm_create(local_comm: MPI_Comm,
                                 local_leader: c_int,
                                 bridge_comm: MPI_Comm,
                                 remote_leader: c_int,
                                 tag: c_int,
                                 newintercomm: *mut MPI_Comm)
     -> c_int;
    pub fn PMPI_Intercomm_merge(intercomm: MPI_Comm, high: c_int,
                                newintercomm: *mut MPI_Comm) -> c_int;
    pub fn PMPI_Iprobe(source: c_int, tag: c_int,
                       comm: MPI_Comm, flag: *mut c_int,
                       status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Irecv(buf: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, source: c_int,
                      tag: c_int, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Irsend(buf: *const c_void, count: c_int,
                       datatype: MPI_Datatype, dest: c_int,
                       tag: c_int, comm: MPI_Comm,
                       request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Isend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Issend(buf: *const c_void, count: c_int,
                       datatype: MPI_Datatype, dest: c_int,
                       tag: c_int, comm: MPI_Comm,
                       request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Is_thread_main(flag: *mut c_int) -> c_int;
    pub fn PMPI_Keyval_create(copy_fn: *mut MPI_Copy_function,
                              delete_fn: *mut MPI_Delete_function,
                              keyval: *mut c_int,
                              extra_state: *mut c_void)
     -> c_int;
    pub fn PMPI_Keyval_free(keyval: *mut c_int) -> c_int;
    pub fn PMPI_Lookup_name(service_name: *const c_char,
                            info: MPI_Info, port_name: *mut c_char)
     -> c_int;
    pub fn PMPI_Message_c2f(message: MPI_Message) -> c_int;
    pub fn PMPI_Message_f2c(message: c_int) -> MPI_Message;
    pub fn PMPI_Mprobe(source: c_int, tag: c_int,
                       comm: MPI_Comm, message: *mut MPI_Message,
                       status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Mrecv(buf: *mut c_void, count: c_int,
                      _type: MPI_Datatype, message: *mut MPI_Message,
                      status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Neighbor_allgather(sendbuf: *const c_void,
                                   sendcount: c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcount: c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ineighbor_allgather(sendbuf: *const c_void,
                                    sendcount: c_int,
                                    sendtype: MPI_Datatype,
                                    recvbuf: *mut c_void,
                                    recvcount: c_int,
                                    recvtype: MPI_Datatype, comm: MPI_Comm,
                                    request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Neighbor_allgatherv(sendbuf: *const c_void,
                                    sendcount: c_int,
                                    sendtype: MPI_Datatype,
                                    recvbuf: *mut c_void,
                                    recvcounts: *const c_int,
                                    displs: *const c_int,
                                    recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ineighbor_allgatherv(sendbuf: *const c_void,
                                     sendcount: c_int,
                                     sendtype: MPI_Datatype,
                                     recvbuf: *mut c_void,
                                     recvcounts: *const c_int,
                                     displs: *const c_int,
                                     recvtype: MPI_Datatype, comm: MPI_Comm,
                                     request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Neighbor_alltoall(sendbuf: *const c_void,
                                  sendcount: c_int,
                                  sendtype: MPI_Datatype,
                                  recvbuf: *mut c_void,
                                  recvcount: c_int,
                                  recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ineighbor_alltoall(sendbuf: *const c_void,
                                   sendcount: c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcount: c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm,
                                   request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Neighbor_alltoallv(sendbuf: *const c_void,
                                   sendcounts: *const c_int,
                                   sdispls: *const c_int,
                                   sendtype: MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcounts: *const c_int,
                                   rdispls: *const c_int,
                                   recvtype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Ineighbor_alltoallv(sendbuf: *const c_void,
                                    sendcounts: *const c_int,
                                    sdispls: *const c_int,
                                    sendtype: MPI_Datatype,
                                    recvbuf: *mut c_void,
                                    recvcounts: *const c_int,
                                    rdispls: *const c_int,
                                    recvtype: MPI_Datatype, comm: MPI_Comm,
                                    request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Neighbor_alltoallw(sendbuf: *const c_void,
                                   sendcounts: *const c_int,
                                   sdispls: *const MPI_Aint,
                                   sendtypes: *const MPI_Datatype,
                                   recvbuf: *mut c_void,
                                   recvcounts: *const c_int,
                                   rdispls: *const MPI_Aint,
                                   recvtypes: *const MPI_Datatype,
                                   comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ineighbor_alltoallw(sendbuf: *const c_void,
                                    sendcounts: *const c_int,
                                    sdispls: *const MPI_Aint,
                                    sendtypes: *const MPI_Datatype,
                                    recvbuf: *mut c_void,
                                    recvcounts: *const c_int,
                                    rdispls: *const MPI_Aint,
                                    recvtypes: *const MPI_Datatype,
                                    comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Op_c2f(op: MPI_Op) -> c_int;
    pub fn PMPI_Op_commutative(op: MPI_Op, commute: *mut c_int)
     -> c_int;
    pub fn PMPI_Op_create(function: *mut MPI_User_function,
                          commute: c_int, op: *mut MPI_Op)
     -> c_int;
    pub fn PMPI_Open_port(info: MPI_Info, port_name: *mut c_char)
     -> c_int;
    pub fn PMPI_Op_f2c(op: c_int) -> MPI_Op;
    pub fn PMPI_Op_free(op: *mut MPI_Op) -> c_int;
    pub fn PMPI_Pack_external(datarep: *const c_char,
                              inbuf: *const c_void,
                              incount: c_int, datatype: MPI_Datatype,
                              outbuf: *mut c_void, outsize: MPI_Aint,
                              position: *mut MPI_Aint) -> c_int;
    pub fn PMPI_Pack_external_size(datarep: *const c_char,
                                   incount: c_int,
                                   datatype: MPI_Datatype,
                                   size: *mut MPI_Aint) -> c_int;
    pub fn PMPI_Pack(inbuf: *const c_void, incount: c_int,
                     datatype: MPI_Datatype, outbuf: *mut c_void,
                     outsize: c_int, position: *mut c_int,
                     comm: MPI_Comm) -> c_int;
    pub fn PMPI_Pack_size(incount: c_int, datatype: MPI_Datatype,
                          comm: MPI_Comm, size: *mut c_int)
     -> c_int;
    pub fn PMPI_Pcontrol(level: c_int, ...) -> c_int;
    pub fn PMPI_Probe(source: c_int, tag: c_int,
                      comm: MPI_Comm, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_Publish_name(service_name: *const c_char,
                             info: MPI_Info, port_name: *const c_char)
     -> c_int;
    pub fn PMPI_Put(origin_addr: *const c_void,
                    origin_count: c_int,
                    origin_datatype: MPI_Datatype, target_rank: c_int,
                    target_disp: MPI_Aint, target_count: c_int,
                    target_datatype: MPI_Datatype, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Query_thread(provided: *mut c_int) -> c_int;
    pub fn PMPI_Raccumulate(origin_addr: *mut c_void,
                            origin_count: c_int,
                            origin_datatype: MPI_Datatype,
                            target_rank: c_int, target_disp: MPI_Aint,
                            target_count: c_int,
                            target_datatype: MPI_Datatype, op: MPI_Op,
                            win: MPI_Win, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Recv_init(buf: *mut c_void, count: c_int,
                          datatype: MPI_Datatype, source: c_int,
                          tag: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Recv(buf: *mut c_void, count: c_int,
                     datatype: MPI_Datatype, source: c_int,
                     tag: c_int, comm: MPI_Comm,
                     status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Reduce(sendbuf: *const c_void,
                       recvbuf: *mut c_void, count: c_int,
                       datatype: MPI_Datatype, op: MPI_Op,
                       root: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ireduce(sendbuf: *const c_void,
                        recvbuf: *mut c_void, count: c_int,
                        datatype: MPI_Datatype, op: MPI_Op,
                        root: c_int, comm: MPI_Comm,
                        request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Reduce_local(inbuf: *const c_void,
                             inoutbuf: *mut c_void,
                             count: c_int, datatype: MPI_Datatype,
                             arg1: MPI_Op) -> c_int;
    pub fn PMPI_Reduce_scatter(sendbuf: *const c_void,
                               recvbuf: *mut c_void,
                               recvcounts: *const c_int,
                               datatype: MPI_Datatype, op: MPI_Op,
                               comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ireduce_scatter(sendbuf: *const c_void,
                                recvbuf: *mut c_void,
                                recvcounts: *const c_int,
                                datatype: MPI_Datatype, op: MPI_Op,
                                comm: MPI_Comm, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Reduce_scatter_block(sendbuf: *const c_void,
                                     recvbuf: *mut c_void,
                                     recvcount: c_int,
                                     datatype: MPI_Datatype, op: MPI_Op,
                                     comm: MPI_Comm) -> c_int;
    pub fn PMPI_Ireduce_scatter_block(sendbuf: *const c_void,
                                      recvbuf: *mut c_void,
                                      recvcount: c_int,
                                      datatype: MPI_Datatype, op: MPI_Op,
                                      comm: MPI_Comm,
                                      request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Register_datarep(datarep: *const c_char,
                                 read_conversion_fn:
                                     *mut MPI_Datarep_conversion_function,
                                 write_conversion_fn:
                                     *mut MPI_Datarep_conversion_function,
                                 dtype_file_extent_fn:
                                     *mut MPI_Datarep_extent_function,
                                 extra_state: *mut c_void)
     -> c_int;
    pub fn PMPI_Request_c2f(request: MPI_Request) -> c_int;
    pub fn PMPI_Request_f2c(request: c_int) -> MPI_Request;
    pub fn PMPI_Request_free(request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Request_get_status(request: MPI_Request,
                                   flag: *mut c_int,
                                   status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Rget(origin_addr: *mut c_void,
                     origin_count: c_int,
                     origin_datatype: MPI_Datatype,
                     target_rank: c_int, target_disp: MPI_Aint,
                     target_count: c_int,
                     target_datatype: MPI_Datatype, win: MPI_Win,
                     request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Rget_accumulate(origin_addr: *const c_void,
                                origin_count: c_int,
                                origin_datatype: MPI_Datatype,
                                result_addr: *mut c_void,
                                result_count: c_int,
                                result_datatype: MPI_Datatype,
                                target_rank: c_int,
                                target_disp: MPI_Aint,
                                target_count: c_int,
                                target_datatype: MPI_Datatype, op: MPI_Op,
                                win: MPI_Win, request: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Rput(origin_addr: *const c_void,
                     origin_count: c_int,
                     origin_datatype: MPI_Datatype,
                     target_rank: c_int, target_disp: MPI_Aint,
                     target_cout: c_int,
                     target_datatype: MPI_Datatype, win: MPI_Win,
                     request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Rsend(ibuf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Rsend_init(buf: *const c_void, count: c_int,
                           datatype: MPI_Datatype, dest: c_int,
                           tag: c_int, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Scan(sendbuf: *const c_void,
                     recvbuf: *mut c_void, count: c_int,
                     datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Iscan(sendbuf: *const c_void,
                      recvbuf: *mut c_void, count: c_int,
                      datatype: MPI_Datatype, op: MPI_Op, comm: MPI_Comm,
                      request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Scatter(sendbuf: *const c_void,
                        sendcount: c_int, sendtype: MPI_Datatype,
                        recvbuf: *mut c_void,
                        recvcount: c_int, recvtype: MPI_Datatype,
                        root: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Iscatter(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         root: c_int, comm: MPI_Comm,
                         request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Scatterv(sendbuf: *const c_void,
                         sendcounts: *const c_int,
                         displs: *const c_int, sendtype: MPI_Datatype,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         root: c_int, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Iscatterv(sendbuf: *const c_void,
                          sendcounts: *const c_int,
                          displs: *const c_int,
                          sendtype: MPI_Datatype,
                          recvbuf: *mut c_void,
                          recvcount: c_int, recvtype: MPI_Datatype,
                          root: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Send_init(buf: *const c_void, count: c_int,
                          datatype: MPI_Datatype, dest: c_int,
                          tag: c_int, comm: MPI_Comm,
                          request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Send(buf: *const c_void, count: c_int,
                     datatype: MPI_Datatype, dest: c_int,
                     tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Sendrecv(sendbuf: *const c_void,
                         sendcount: c_int, sendtype: MPI_Datatype,
                         dest: c_int, sendtag: c_int,
                         recvbuf: *mut c_void,
                         recvcount: c_int, recvtype: MPI_Datatype,
                         source: c_int, recvtag: c_int,
                         comm: MPI_Comm, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_Sendrecv_replace(buf: *mut c_void,
                                 count: c_int, datatype: MPI_Datatype,
                                 dest: c_int, sendtag: c_int,
                                 source: c_int,
                                 recvtag: c_int, comm: MPI_Comm,
                                 status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Ssend_init(buf: *const c_void, count: c_int,
                           datatype: MPI_Datatype, dest: c_int,
                           tag: c_int, comm: MPI_Comm,
                           request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Ssend(buf: *const c_void, count: c_int,
                      datatype: MPI_Datatype, dest: c_int,
                      tag: c_int, comm: MPI_Comm) -> c_int;
    pub fn PMPI_Start(request: *mut MPI_Request) -> c_int;
    pub fn PMPI_Startall(count: c_int,
                         array_of_requests: *mut MPI_Request)
     -> c_int;
    pub fn PMPI_Status_c2f(c_status: *const MPI_Status,
                           f_status: *mut c_int) -> c_int;
    pub fn PMPI_Status_f2c(f_status: *const c_int,
                           c_status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Status_set_cancelled(status: *mut MPI_Status,
                                     flag: c_int) -> c_int;
    pub fn PMPI_Status_set_elements(status: *mut MPI_Status,
                                    datatype: MPI_Datatype,
                                    count: c_int) -> c_int;
    pub fn PMPI_Status_set_elements_x(status: *mut MPI_Status,
                                      datatype: MPI_Datatype,
                                      count: MPI_Count) -> c_int;
    pub fn PMPI_Testall(count: c_int,
                        array_of_requests: *mut MPI_Request,
                        flag: *mut c_int,
                        array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn PMPI_Testany(count: c_int,
                        array_of_requests: *mut MPI_Request,
                        index: *mut c_int, flag: *mut c_int,
                        status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Test(request: *mut MPI_Request, flag: *mut c_int,
                     status: *mut MPI_Status) -> c_int;
    pub fn PMPI_Test_cancelled(status: *const MPI_Status,
                               flag: *mut c_int) -> c_int;
    pub fn PMPI_Testsome(incount: c_int,
                         array_of_requests: *mut MPI_Request,
                         outcount: *mut c_int,
                         array_of_indices: *mut c_int,
                         array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn PMPI_Topo_test(comm: MPI_Comm, status: *mut c_int)
     -> c_int;
    pub fn PMPI_Type_c2f(datatype: MPI_Datatype) -> c_int;
    pub fn PMPI_Type_commit(_type: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_contiguous(count: c_int, oldtype: MPI_Datatype,
                                newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_create_darray(size: c_int, rank: c_int,
                                   ndims: c_int,
                                   gsize_array: *const c_int,
                                   distrib_array: *const c_int,
                                   darg_array: *const c_int,
                                   psize_array: *const c_int,
                                   order: c_int,
                                   oldtype: MPI_Datatype,
                                   newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_f90_complex(p: c_int, r: c_int,
                                        newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_f90_integer(r: c_int,
                                        newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_f90_real(p: c_int, r: c_int,
                                     newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_hindexed(count: c_int,
                                     array_of_blocklengths:
                                         *const c_int,
                                     array_of_displacements: *const MPI_Aint,
                                     oldtype: MPI_Datatype,
                                     newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_hvector(count: c_int,
                                    blocklength: c_int,
                                    stride: MPI_Aint, oldtype: MPI_Datatype,
                                    newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_keyval(type_copy_attr_fn:
                                       *mut MPI_Type_copy_attr_function,
                                   type_delete_attr_fn:
                                       *mut MPI_Type_delete_attr_function,
                                   type_keyval: *mut c_int,
                                   extra_state: *mut c_void)
     -> c_int;
    pub fn PMPI_Type_create_hindexed_block(count: c_int,
                                           blocklength: c_int,
                                           array_of_displacements:
                                               *const MPI_Aint,
                                           oldtype: MPI_Datatype,
                                           newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_indexed_block(count: c_int,
                                          blocklength: c_int,
                                          array_of_displacements:
                                              *const c_int,
                                          oldtype: MPI_Datatype,
                                          newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_struct(count: c_int,
                                   array_of_block_lengths:
                                       *const c_int,
                                   array_of_displacements: *const MPI_Aint,
                                   array_of_types: *const MPI_Datatype,
                                   newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_subarray(ndims: c_int,
                                     size_array: *const c_int,
                                     subsize_array: *const c_int,
                                     start_array: *const c_int,
                                     order: c_int,
                                     oldtype: MPI_Datatype,
                                     newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_create_resized(oldtype: MPI_Datatype, lb: MPI_Aint,
                                    extent: MPI_Aint,
                                    newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_delete_attr(_type: MPI_Datatype,
                                 type_keyval: c_int) -> c_int;
    pub fn PMPI_Type_dup(_type: MPI_Datatype, newtype: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_extent(_type: MPI_Datatype, extent: *mut MPI_Aint)
     -> c_int;
    pub fn PMPI_Type_free(_type: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_free_keyval(type_keyval: *mut c_int)
     -> c_int;
    pub fn PMPI_Type_f2c(datatype: c_int) -> MPI_Datatype;
    pub fn PMPI_Type_get_attr(_type: MPI_Datatype, type_keyval: c_int,
                              attribute_val: *mut c_void,
                              flag: *mut c_int) -> c_int;
    pub fn PMPI_Type_get_contents(mtype: MPI_Datatype,
                                  max_integers: c_int,
                                  max_addresses: c_int,
                                  max_datatypes: c_int,
                                  array_of_integers: *mut c_int,
                                  array_of_addresses: *mut MPI_Aint,
                                  array_of_datatypes: *mut MPI_Datatype)
     -> c_int;
    pub fn PMPI_Type_get_envelope(_type: MPI_Datatype,
                                  num_integers: *mut c_int,
                                  num_addresses: *mut c_int,
                                  num_datatypes: *mut c_int,
                                  combiner: *mut c_int)
     -> c_int;
    pub fn PMPI_Type_get_extent(_type: MPI_Datatype, lb: *mut MPI_Aint,
                                extent: *mut MPI_Aint) -> c_int;
    pub fn PMPI_Type_get_extent_x(_type: MPI_Datatype, lb: *mut MPI_Count,
                                  extent: *mut MPI_Count) -> c_int;
    pub fn PMPI_Type_get_name(_type: MPI_Datatype,
                              type_name: *mut c_char,
                              resultlen: *mut c_int) -> c_int;
    pub fn PMPI_Type_get_true_extent(datatype: MPI_Datatype,
                                     true_lb: *mut MPI_Aint,
                                     true_extent: *mut MPI_Aint)
     -> c_int;
    pub fn PMPI_Type_get_true_extent_x(datatype: MPI_Datatype,
                                       true_lb: *mut MPI_Count,
                                       true_extent: *mut MPI_Count)
     -> c_int;
    pub fn PMPI_Type_hindexed(count: c_int,
                              array_of_blocklengths: *mut c_int,
                              array_of_displacements: *mut MPI_Aint,
                              oldtype: MPI_Datatype,
                              newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_hvector(count: c_int, blocklength: c_int,
                             stride: MPI_Aint, oldtype: MPI_Datatype,
                             newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_indexed(count: c_int,
                             array_of_blocklengths: *const c_int,
                             array_of_displacements: *const c_int,
                             oldtype: MPI_Datatype,
                             newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_lb(_type: MPI_Datatype, lb: *mut MPI_Aint)
     -> c_int;
    pub fn PMPI_Type_match_size(typeclass: c_int, size: c_int,
                                _type: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_set_attr(_type: MPI_Datatype, type_keyval: c_int,
                              attr_val: *mut c_void) -> c_int;
    pub fn PMPI_Type_set_name(_type: MPI_Datatype,
                              type_name: *const c_char)
     -> c_int;
    pub fn PMPI_Type_size(_type: MPI_Datatype, size: *mut c_int)
     -> c_int;
    pub fn PMPI_Type_size_x(_type: MPI_Datatype, size: *mut MPI_Count)
     -> c_int;
    pub fn PMPI_Type_struct(count: c_int,
                            array_of_blocklengths: *mut c_int,
                            array_of_displacements: *mut MPI_Aint,
                            array_of_types: *mut MPI_Datatype,
                            newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Type_ub(mtype: MPI_Datatype, ub: *mut MPI_Aint)
     -> c_int;
    pub fn PMPI_Type_vector(count: c_int, blocklength: c_int,
                            stride: c_int, oldtype: MPI_Datatype,
                            newtype: *mut MPI_Datatype) -> c_int;
    pub fn PMPI_Unpack(inbuf: *const c_void, insize: c_int,
                       position: *mut c_int,
                       outbuf: *mut c_void, outcount: c_int,
                       datatype: MPI_Datatype, comm: MPI_Comm)
     -> c_int;
    pub fn PMPI_Unpublish_name(service_name: *const c_char,
                               info: MPI_Info,
                               port_name: *const c_char)
     -> c_int;
    pub fn PMPI_Unpack_external(datarep: *const c_char,
                                inbuf: *const c_void,
                                insize: MPI_Aint, position: *mut MPI_Aint,
                                outbuf: *mut c_void,
                                outcount: c_int,
                                datatype: MPI_Datatype) -> c_int;
    pub fn PMPI_Waitall(count: c_int,
                        array_of_requests: *mut MPI_Request,
                        array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn PMPI_Waitany(count: c_int,
                        array_of_requests: *mut MPI_Request,
                        index: *mut c_int, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_Wait(request: *mut MPI_Request, status: *mut MPI_Status)
     -> c_int;
    pub fn PMPI_Waitsome(incount: c_int,
                         array_of_requests: *mut MPI_Request,
                         outcount: *mut c_int,
                         array_of_indices: *mut c_int,
                         array_of_statuses: *mut MPI_Status) -> c_int;
    pub fn PMPI_Win_allocate(size: MPI_Aint, disp_unit: c_int,
                             info: MPI_Info, comm: MPI_Comm,
                             baseptr: *mut c_void, win: *mut MPI_Win)
     -> c_int;
    pub fn PMPI_Win_allocate_shared(size: MPI_Aint, disp_unit: c_int,
                                    info: MPI_Info, comm: MPI_Comm,
                                    baseptr: *mut c_void,
                                    win: *mut MPI_Win) -> c_int;
    pub fn PMPI_Win_attach(win: MPI_Win, base: *mut c_void,
                           size: MPI_Aint) -> c_int;
    pub fn PMPI_Win_c2f(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_call_errhandler(win: MPI_Win, errorcode: c_int)
     -> c_int;
    pub fn PMPI_Win_complete(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_create(base: *mut c_void, size: MPI_Aint,
                           disp_unit: c_int, info: MPI_Info,
                           comm: MPI_Comm, win: *mut MPI_Win)
     -> c_int;
    pub fn PMPI_Win_create_dynamic(info: MPI_Info, comm: MPI_Comm,
                                   win: *mut MPI_Win) -> c_int;
    pub fn PMPI_Win_create_errhandler(function:
                                          *mut MPI_Win_errhandler_function,
                                      errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Win_create_keyval(win_copy_attr_fn:
                                      *mut MPI_Win_copy_attr_function,
                                  win_delete_attr_fn:
                                      *mut MPI_Win_delete_attr_function,
                                  win_keyval: *mut c_int,
                                  extra_state: *mut c_void)
     -> c_int;
    pub fn PMPI_Win_delete_attr(win: MPI_Win, win_keyval: c_int)
     -> c_int;
    pub fn PMPI_Win_detach(win: MPI_Win, base: *mut c_void)
     -> c_int;
    pub fn PMPI_Win_f2c(win: c_int) -> MPI_Win;
    pub fn PMPI_Win_fence(assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Win_flush(rank: c_int, win: MPI_Win) -> c_int;
    pub fn PMPI_Win_flush_all(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_flush_local(rank: c_int, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Win_flush_local_all(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_free(win: *mut MPI_Win) -> c_int;
    pub fn PMPI_Win_free_keyval(win_keyval: *mut c_int)
     -> c_int;
    pub fn PMPI_Win_get_attr(win: MPI_Win, win_keyval: c_int,
                             attribute_val: *mut c_void,
                             flag: *mut c_int) -> c_int;
    pub fn PMPI_Win_get_errhandler(win: MPI_Win,
                                   errhandler: *mut MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Win_get_group(win: MPI_Win, group: *mut MPI_Group)
     -> c_int;
    pub fn PMPI_Win_get_info(win: MPI_Win, info_used: *mut MPI_Info)
     -> c_int;
    pub fn PMPI_Win_get_name(win: MPI_Win, win_name: *mut c_char,
                             resultlen: *mut c_int) -> c_int;
    pub fn PMPI_Win_lock(lock_type: c_int, rank: c_int,
                         assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Win_lock_all(assert: c_int, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Win_post(group: MPI_Group, assert: c_int,
                         win: MPI_Win) -> c_int;
    pub fn PMPI_Win_set_attr(win: MPI_Win, win_keyval: c_int,
                             attribute_val: *mut c_void)
     -> c_int;
    pub fn PMPI_Win_set_errhandler(win: MPI_Win, errhandler: MPI_Errhandler)
     -> c_int;
    pub fn PMPI_Win_set_info(win: MPI_Win, info: MPI_Info) -> c_int;
    pub fn PMPI_Win_set_name(win: MPI_Win, win_name: *const c_char)
     -> c_int;
    pub fn PMPI_Win_shared_query(win: MPI_Win, rank: c_int,
                                 size: *mut MPI_Aint,
                                 disp_unit: *mut c_int,
                                 baseptr: *mut c_void)
     -> c_int;
    pub fn PMPI_Win_start(group: MPI_Group, assert: c_int,
                          win: MPI_Win) -> c_int;
    pub fn PMPI_Win_sync(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_test(win: MPI_Win, flag: *mut c_int)
     -> c_int;
    pub fn PMPI_Win_unlock(rank: c_int, win: MPI_Win)
     -> c_int;
    pub fn PMPI_Win_unlock_all(win: MPI_Win) -> c_int;
    pub fn PMPI_Win_wait(win: MPI_Win) -> c_int;
    pub fn PMPI_Wtick() -> c_double;
    pub fn PMPI_Wtime() -> c_double;
    pub fn PMPI_T_init_thread(required: c_int,
                              provided: *mut c_int) -> c_int;
    pub fn PMPI_T_finalize() -> c_int;
    pub fn PMPI_T_cvar_get_num(num_cvar: *mut c_int) -> c_int;
    pub fn PMPI_T_cvar_get_info(cvar_index: c_int,
                                name: *mut c_char,
                                name_len: *mut c_int,
                                verbosity: *mut c_int,
                                datatype: *mut MPI_Datatype,
                                enumtype: *mut MPI_T_enum,
                                desc: *mut c_char,
                                desc_len: *mut c_int,
                                bind: *mut c_int,
                                scope: *mut c_int) -> c_int;
    pub fn PMPI_T_cvar_get_index(name: *const c_char,
                                 cvar_index: *mut c_int)
     -> c_int;
    pub fn PMPI_T_cvar_handle_alloc(cvar_index: c_int,
                                    obj_handle: *mut c_void,
                                    handle: *mut MPI_T_cvar_handle,
                                    count: *mut c_int)
     -> c_int;
    pub fn PMPI_T_cvar_handle_free(handle: *mut MPI_T_cvar_handle)
     -> c_int;
    pub fn PMPI_T_cvar_read(handle: MPI_T_cvar_handle,
                            buf: *mut c_void) -> c_int;
    pub fn PMPI_T_cvar_write(handle: MPI_T_cvar_handle,
                             buf: *const c_void) -> c_int;
    pub fn PMPI_T_category_get_num(num_cat: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_get_info(cat_index: c_int,
                                    name: *mut c_char,
                                    name_len: *mut c_int,
                                    desc: *mut c_char,
                                    desc_len: *mut c_int,
                                    num_cvars: *mut c_int,
                                    num_pvars: *mut c_int,
                                    num_categories: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_get_index(name: *const c_char,
                                     category_index: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_get_cvars(cat_index: c_int,
                                     len: c_int,
                                     indices: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_get_pvars(cat_index: c_int,
                                     len: c_int,
                                     indices: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_get_categories(cat_index: c_int,
                                          len: c_int,
                                          indices: *mut c_int)
     -> c_int;
    pub fn PMPI_T_category_changed(stamp: *mut c_int)
     -> c_int;
    pub fn PMPI_T_pvar_get_num(num_pvar: *mut c_int) -> c_int;
    pub fn PMPI_T_pvar_get_info(pvar_index: c_int,
                                name: *mut c_char,
                                name_len: *mut c_int,
                                verbosity: *mut c_int,
                                var_class: *mut c_int,
                                datatype: *mut MPI_Datatype,
                                enumtype: *mut MPI_T_enum,
                                desc: *mut c_char,
                                desc_len: *mut c_int,
                                bind: *mut c_int,
                                readonly: *mut c_int,
                                continuous: *mut c_int,
                                atomic: *mut c_int) -> c_int;
    pub fn PMPI_T_pvar_get_index(name: *const c_char,
                                 pvar_index: *mut c_int)
     -> c_int;
    pub fn PMPI_T_pvar_session_create(session: *mut MPI_T_pvar_session)
     -> c_int;
    pub fn PMPI_T_pvar_session_free(session: *mut MPI_T_pvar_session)
     -> c_int;
    pub fn PMPI_T_pvar_handle_alloc(session: MPI_T_pvar_session,
                                    pvar_index: c_int,
                                    obj_handle: *mut c_void,
                                    handle: *mut MPI_T_pvar_handle,
                                    count: *mut c_int)
     -> c_int;
    pub fn PMPI_T_pvar_handle_free(session: MPI_T_pvar_session,
                                   handle: *mut MPI_T_pvar_handle)
     -> c_int;
    pub fn PMPI_T_pvar_start(session: MPI_T_pvar_session,
                             handle: MPI_T_pvar_handle) -> c_int;
    pub fn PMPI_T_pvar_stop(session: MPI_T_pvar_session,
                            handle: MPI_T_pvar_handle) -> c_int;
    pub fn PMPI_T_pvar_read(session: MPI_T_pvar_session,
                            handle: MPI_T_pvar_handle,
                            buf: *mut c_void) -> c_int;
    pub fn PMPI_T_pvar_write(session: MPI_T_pvar_session,
                             handle: MPI_T_pvar_handle,
                             buf: *const c_void) -> c_int;
    pub fn PMPI_T_pvar_reset(session: MPI_T_pvar_session,
                             handle: MPI_T_pvar_handle) -> c_int;
    pub fn PMPI_T_pvar_readreset(session: MPI_T_pvar_session,
                                 handle: MPI_T_pvar_handle,
                                 buf: *mut c_void) -> c_int;
    pub fn PMPI_T_enum_get_info(enumtype: MPI_T_enum, num: *mut c_int,
                                name: *mut c_char,
                                name_len: *mut c_int)
     -> c_int;
    pub fn PMPI_T_enum_get_item(enumtype: MPI_T_enum, index: c_int,
                                value: *mut c_int,
                                name: *mut c_char,
                                name_len: *mut c_int)
     -> c_int;
    pub fn MPI_T_init_thread(required: c_int,
                             provided: *mut c_int) -> c_int;
    pub fn MPI_T_finalize() -> c_int;
    pub fn MPI_T_cvar_get_num(num_cvar: *mut c_int) -> c_int;
    pub fn MPI_T_cvar_get_info(cvar_index: c_int,
                               name: *mut c_char,
                               name_len: *mut c_int,
                               verbosity: *mut c_int,
                               datatype: *mut MPI_Datatype,
                               enumtype: *mut MPI_T_enum,
                               desc: *mut c_char,
                               desc_len: *mut c_int,
                               bind: *mut c_int,
                               scope: *mut c_int) -> c_int;
    pub fn MPI_T_cvar_get_index(name: *const c_char,
                                cvar_index: *mut c_int)
     -> c_int;
    pub fn MPI_T_cvar_handle_alloc(cvar_index: c_int,
                                   obj_handle: *mut c_void,
                                   handle: *mut MPI_T_cvar_handle,
                                   count: *mut c_int)
     -> c_int;
    pub fn MPI_T_cvar_handle_free(handle: *mut MPI_T_cvar_handle)
     -> c_int;
    pub fn MPI_T_cvar_read(handle: MPI_T_cvar_handle,
                           buf: *mut c_void) -> c_int;
    pub fn MPI_T_cvar_write(handle: MPI_T_cvar_handle,
                            buf: *const c_void) -> c_int;
    pub fn MPI_T_category_get_num(num_cat: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_get_info(cat_index: c_int,
                                   name: *mut c_char,
                                   name_len: *mut c_int,
                                   desc: *mut c_char,
                                   desc_len: *mut c_int,
                                   num_cvars: *mut c_int,
                                   num_pvars: *mut c_int,
                                   num_categories: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_get_index(name: *const c_char,
                                    category_index: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_get_cvars(cat_index: c_int,
                                    len: c_int,
                                    indices: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_get_pvars(cat_index: c_int,
                                    len: c_int,
                                    indices: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_get_categories(cat_index: c_int,
                                         len: c_int,
                                         indices: *mut c_int)
     -> c_int;
    pub fn MPI_T_category_changed(stamp: *mut c_int) -> c_int;
    pub fn MPI_T_pvar_get_num(num_pvar: *mut c_int) -> c_int;
    pub fn MPI_T_pvar_get_info(pvar_index: c_int,
                               name: *mut c_char,
                               name_len: *mut c_int,
                               verbosity: *mut c_int,
                               var_class: *mut c_int,
                               datatype: *mut MPI_Datatype,
                               enumtype: *mut MPI_T_enum,
                               desc: *mut c_char,
                               desc_len: *mut c_int,
                               bind: *mut c_int,
                               readonly: *mut c_int,
                               continuous: *mut c_int,
                               atomic: *mut c_int) -> c_int;
    pub fn MPI_T_pvar_get_index(name: *const c_char,
                                pvar_index: *mut c_int)
     -> c_int;
    pub fn MPI_T_pvar_session_create(session: *mut MPI_T_pvar_session)
     -> c_int;
    pub fn MPI_T_pvar_session_free(session: *mut MPI_T_pvar_session)
     -> c_int;
    pub fn MPI_T_pvar_handle_alloc(session: MPI_T_pvar_session,
                                   pvar_index: c_int,
                                   obj_handle: *mut c_void,
                                   handle: *mut MPI_T_pvar_handle,
                                   count: *mut c_int)
     -> c_int;
    pub fn MPI_T_pvar_handle_free(session: MPI_T_pvar_session,
                                  handle: *mut MPI_T_pvar_handle)
     -> c_int;
    pub fn MPI_T_pvar_start(session: MPI_T_pvar_session,
                            handle: MPI_T_pvar_handle) -> c_int;
    pub fn MPI_T_pvar_stop(session: MPI_T_pvar_session,
                           handle: MPI_T_pvar_handle) -> c_int;
    pub fn MPI_T_pvar_read(session: MPI_T_pvar_session,
                           handle: MPI_T_pvar_handle,
                           buf: *mut c_void) -> c_int;
    pub fn MPI_T_pvar_write(session: MPI_T_pvar_session,
                            handle: MPI_T_pvar_handle,
                            buf: *const c_void) -> c_int;
    pub fn MPI_T_pvar_reset(session: MPI_T_pvar_session,
                            handle: MPI_T_pvar_handle) -> c_int;
    pub fn MPI_T_pvar_readreset(session: MPI_T_pvar_session,
                                handle: MPI_T_pvar_handle,
                                buf: *mut c_void) -> c_int;
    pub fn MPI_T_enum_get_info(enumtype: MPI_T_enum, num: *mut c_int,
                               name: *mut c_char,
                               name_len: *mut c_int) -> c_int;
    pub fn MPI_T_enum_get_item(enumtype: MPI_T_enum, index: c_int,
                               value: *mut c_int,
                               name: *mut c_char,
                               name_len: *mut c_int) -> c_int;
}
