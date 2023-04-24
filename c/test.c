#include <stdio.h>
#include "info.h"

int main(void) {
	printf("os type: %s\n", get_os_type());
	printf("os release: %s\n", get_os_release());

	printf("\ncpu num: %u\n", (unsigned int) get_cpu_num());
	printf("cpu speed: %lu\n", (unsigned long) get_cpu_speed());

	LoadAvg la = get_loadavg();
	printf("\nloadavg: %f %f %f\n", la.one, la.five, la.fifteen);

	printf("proc total: %lu\n", (unsigned long) get_proc_total());

	MemInfo mi = get_mem_info();
	printf("\nmem:\ntotal %llu, avail %llu, free %llu\n",
		(unsigned long long) mi.total, (unsigned long long) mi.avail,
		(unsigned long long) mi.free);
	printf("buffers %llu, cached %llu\n",
		(unsigned long long) mi.buffers, (unsigned long long) mi.cached);
	printf("swap: total %llu, free %llu\n",
		(unsigned long long) mi.swap_total, (unsigned long long) mi.swap_free);

	DiskInfo di = get_disk_info();
	printf("\ndisk: total %llu, free %llu\n",
		(unsigned long long) di.total, (unsigned long long) di.free);

	return 0;
}
