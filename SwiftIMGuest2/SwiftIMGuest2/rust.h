#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*RecvCallback)(const char*);

void init_im(const char *db_path,
             const char *id,
             const char *host,
             int32_t port,
             const char *recv_topic,
             RecvCallback recv_callback);

void send_msg(const char *from_id, const char *to_id, const char *send_topic, const char *msg);
