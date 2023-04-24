#ifndef INFO_H_
#define INFO_H_

#include <stdint.h>

typedef struct LoadAvg {
        double one;
        double five;
        double fifteen;
} LoadAvg;

typedef struct MemInfo {
        uint64_t total;
        uint64_t free;
        uint64_t avail;

        uint64_t buffers;
        uint64_t cached;

        uint64_t swap_total;
        uint64_t swap_free;
} MemInfo;

typedef struct DiskInfo {
        uint64_t total;
        uint64_t free;
} DiskInfo;

const char *get_os_type(void);
const char *get_os_release(void);

uint32_t get_cpu_num(void);
uint64_t get_cpu_speed(void);

LoadAvg get_loadavg(void);
uint64_t get_proc_total(void);

MemInfo get_mem_info(void);
DiskInfo get_disk_info(void);

#endif
  
