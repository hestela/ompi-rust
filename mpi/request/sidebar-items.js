initSidebarItems({"fn":[["get_status","Replacement for MPI_Request_get_status. Since success is always retured, an option containing the status is returned to avoid having to use int* flag to test success."],["new",""],["wait","Replacement for MPI_Wait. Rerturns Ok(Option<MPI_Status) on MPI_SUCCESS, else Err. MPI_STATUS_IGNORED is encapsulated in the option as None."]]});