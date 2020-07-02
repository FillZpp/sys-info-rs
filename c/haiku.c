#include <ctype.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include <OS.h>
#include <fs_info.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>

#include "info.h"

static const char *os_type = "Haiku";

const char *get_os_type(void) {
	return os_type;
}

unsigned int get_cpu_num(void) {
	return 1;
}

DiskInfo get_disk_info(void) {
	fs_info info;
	dev_t device;
	DiskInfo di;

	// fix: Handle other paths
	device = dev_for_path("/");

	fs_stat_dev(device, &info);

	di.total = info.total_blocks;
	di.free = info.free_blocks;
	return di;
}

LoadAvg get_loadavg(void) {
	LoadAvg la;

	la.one = 0.0;
	la.five = 0.0;
	la.fifteen = 0.0;
	return la;
}

MemInfo get_mem_info(void) {
	MemInfo mi;

	system_info systemInfo;
	get_system_info(&systemInfo);

	mi.total       = 100 * systemInfo.used_pages / systemInfo.max_pages;
	mi.avail       = systemInfo.free_memory;
	mi.free        = systemInfo.free_memory;
	mi.buffers     = 0;
	mi.cached      = 0;
	mi.swap_total  = 0;
	mi.swap_free   = 0;

	return mi;
}

unsigned long get_cpu_speed(void) {
	// TODO: better handling
	return (long)get_rounded_cpu_speed();
}

int64 get_uptime(void) {
	bigtime_t uptime = system_time();

	return uptime;
}

