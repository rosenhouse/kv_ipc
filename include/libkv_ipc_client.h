#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

// Connection to the Key/Value Server
//
// Fields are published so the caller can allocate memory of the correct size
// but the caller should not use the fields themselves
typedef struct {
  int32_t fd;
} KVConn;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Create a connection to the named Key/Value server
// Keyed off a file descriptor
// So it is safe for use by concurrent threads and for
// use by child processes
//
// Returns 0 on success or an OS errno
int32_t open_kvconn(const char *ipc_server_name, KVConn *conn_out);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
