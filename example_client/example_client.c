#include <stdio.h>
#include "libkv_ipc_client.h"

int main (int argc, char *argv[]){
   if (argc <= 1) {
      fprintf(stderr, "usage: %s <socket path>\n", argv[0]);
      exit(1);
   }

   KVClient client;
   int rc = kvclient_open(argv[1], &client);
   if (rc != 0) {
      fprintf(stderr, "open error: %d\n", rc);
      exit(1);
   }

   printf("c program set fd=%d\n", client.fd);
}
