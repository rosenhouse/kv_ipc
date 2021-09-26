#include <stdio.h>
#include "libkv_ipc_client.h"

int main() {
   KVConn kvconn;
   int rc = open_kvconn("some-name", &kvconn);
   printf("c program got rc=%d\n", rc);
   printf("c program set fd=%d\n", kvconn.fd);
}
