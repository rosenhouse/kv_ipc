#include <stdio.h>
#include "libkv_ipc_client.h"

int main (int argc, char *argv[]){
   if (argc <= 1) {
      fprintf(stderr, "usage: %s <socket path>\n", argv[0]);
      return 1;
   }

   KVClient client;
   int rc = kvclient_open(argv[1], &client);
   if (rc != 0) {
      fprintf(stderr, "open error: %d\n", rc);
      return 1;
   }

   unsigned char key[] = {10, 20, 30};
   unsigned char value[] = {100, 200};

   KVClientRecord record = {
      .table_id = 42,
      .key_size = sizeof(key),
      .key_data = key,
      .value_size = sizeof(value),
      .value_data = value,
   };

   rc = kvclient_insert(client, &record);
   if (rc != 0) {
      fprintf(stderr, "insert error: %d\n", rc);
      return 1;
   }
   return 0;
}
