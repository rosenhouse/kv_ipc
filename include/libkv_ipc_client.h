#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

// Connection to the Key/Value server
//
// Fields are published so the caller can allocate memory of the correct size
// but the caller should not use the fields themselves
typedef struct {
  int fd;
} KVClient;

// Record that may be inserted into the server
typedef struct {
  uint32_t table_id;
  uintptr_t key_size;
  const uint8_t *key_data;
  uintptr_t value_size;
  const uint8_t *value_data;
} KVClientRecord;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// Create a connection to the named Key/Value server
//
// Keyed off a file descriptor
// So it is safe for use by concurrent threads and for use by child processes
//
// Returns 0 on success
int kvclient_open(const char *path, KVClient *kvclient_out);

// Insert a record into the server
//
// Returns 0 on success
int kvclient_insert(KVClient kvclient, const KVClientRecord *record);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
