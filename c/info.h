#ifndef INFO_H_
#define INFO_H_

typedef struct LoadAvg {
        double one;
        double five;
        double fifteen;
} LoadAvg;

typedef struct MemInfo {
        unsigned long long total;
        unsigned long long avail;
        unsigned long long free;

        unsigned long long buffers;
        unsigned long long cached;

        unsigned long long swap_total;
        unsigned long long swap_free;
} MemInfo;

typedef struct DiskInfo {
        double total;
        double avail;
} DiskInfo;

char *get_os_type(void);
char *get_os_release(void);

unsigned int get_cpu_num(void);
unsigned int get_cpu_physical_num(void);
unsigned long long get_cpu_speed(void);

LoadAvg get_loadavg(void);
unsigned int get_proc_total(void);

MemInfo get_mem_info(void);
DiskInfo get_disk_info(void);

#endif
  
