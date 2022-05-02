#include <OS.h>

#include <fs_info.h>

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "info.h"

static const char *os_type = "Haiku";


/* Get information */

const char *get_os_type(void) {
	return os_type;
}

const char *get_os_release(void) {
	return "";
}

unsigned int get_cpu_num(void) {
	system_info sysInfo;

  if (get_system_info(&sysInfo) == B_OK) {
    return sysInfo.cpu_count;
  }

  return 1;
}


/*
  get_cpu_speed
*/

unsigned long get_cpu_speed(void) {
  uint32 topologyNodeCount = 0;
	cpu_topology_node_info* topology = NULL;
	get_cpu_topology_info(NULL, &topologyNodeCount);
	if (topologyNodeCount != 0)
		topology = (cpu_topology_node_info*)calloc(topologyNodeCount, sizeof(cpu_topology_node_info));
	get_cpu_topology_info(topology, &topologyNodeCount);

	uint64 cpuFrequency = 0;
	for (uint32 i = 0; i < topologyNodeCount; i++) {
		if (topology[i].type == B_TOPOLOGY_CORE) {
				cpuFrequency = topology[i].data.core.default_frequency;
				break;
		}
	}
	free(topology);

  return cpuFrequency / 1000000;
}

/*
  get_loadavg & get_proc_total
  /proc/loadavg
*/

LoadAvg get_loadavg(void) {
    static LoadAvg avg;
    return avg;
}

unsigned long get_proc_total(void) {
    system_info sysInfo;

  if (get_system_info(&sysInfo) == B_OK) {
    return sysInfo.used_teams;
  }

  return 1;
}

/*
  get_mem_info
*/

MemInfo get_mem_info(void) {
    MemInfo mi;

    system_info sysInfo;

    int factor = B_PAGE_SIZE / 1024;

    if (get_system_info(&sysInfo) == B_OK) {
      mi.total = (uint64)(sysInfo.max_pages) * factor;
      mi.free = (uint64)(sysInfo.max_pages - sysInfo.used_pages) * factor;
      mi.avail = 0;
      mi.cached = (uint64)(sysInfo.cached_pages) * factor;
      mi.buffers = 0;
      mi.swap_total = (uint64)(sysInfo.max_swap_pages) * factor;
      mi.swap_free = (uint64)(sysInfo.free_swap_pages) * factor;
    } else {
      memset(&mi, 0, sizeof(mi));
    }

    return mi;
}

DiskInfo get_disk_info(void) {
	DiskInfo di;

  fs_info info;
  int32 cookie = 0;

  dev_t handle = next_dev(&cookie);

  while (fs_stat_dev(handle, &info) >= 0) {
    // just count the native FS
    if (strcmp(info.fsh_name, "bfs") == 0) {
      unsigned long long free = info.free_blocks;
      unsigned long long total = info.total_blocks;
      free *= info.block_size;
      free /= 1024;
      total *= info.block_size;
      total /= 1024;
      di.free += free;
      di.total += total;
    }

    handle = next_dev(&cookie);
  }

  return di;
}
