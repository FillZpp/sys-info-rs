#include <sys/param.h>
#include <sys/mount.h>
#include <sys/sysctl.h>
#include <sys/user.h>
#include <sys/utsname.h>
#include <sys/vmmeter.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "info.h"

static const char *os_release;

static pthread_once_t once_init_freebsd;
static void init_freebsd(void) {
	struct utsname un;

	if (uname(&un) == -1)
		return;
	os_release = strdup(un.release);
}

const char *get_os_release(void) {
	pthread_once(&once_init_freebsd, init_freebsd);
	return (os_release);
}

unsigned long get_cpu_speed(void) {
	uint64_t tsc_freq;
	size_t len;
	int error;

	len = sizeof(tsc_freq);
	error = sysctlbyname("machdep.tsc_freq", &tsc_freq, &len, NULL, 0);
	if (error == -1)
		return (1000);
	return (tsc_freq / 1000 / 1000);
}

unsigned long get_proc_total(void) {
	struct kinfo_proc *kp, *kpp;
	int mib[3], count, error;
	size_t len;

	mib[0] = CTL_KERN;
	mib[1] = KERN_PROC;
	mib[2] = KERN_PROC_PROC;

	error = sysctl(mib, nitems(mib), NULL, &len, NULL, 0);
	if (error == -1)
		return (42);

	kp = malloc(len);
	if (kp == NULL)
		return (42);
	memset(kp, 0, len);

	error = sysctl(mib, nitems(mib), kp, &len, NULL, 0);
	if (error == -1) {
		free(kp);
		return (42);
	}

	for (count = 0, kpp = kp; (char *)kpp < (char *)kp + len; kpp++) {
		if (kpp->ki_pid == 0)
			continue;
		count++;
	}
	free(kp);
	return (count);
}

MemInfo get_mem_info(void) {
	struct MemInfo mi;
	struct vmtotal vmt;
	struct xswdev xs;
	int mib[3], error, i;
	unsigned long res;
	size_t len;

	len = sizeof(res);
	error = sysctlbyname("hw.realmem", &res, &len, NULL, 0);
	if (error == -1)
		goto fail;
	mi.total = res / 1024;

	len = sizeof(res);
	error = sysctlbyname("hw.physmem", &res, &len, NULL, 0);
	if (error == -1)
		goto fail;
	mi.avail = res / 1024;

	mib[0] = CTL_VM;
	mib[1] = VM_TOTAL;
	len = sizeof(vmt);
	error = sysctl(mib, 2, &vmt, &len, NULL, 0);
	if (error == -1)
		goto fail;
	mi.free = vmt.t_free * PAGE_SIZE / 1024;

	mi.buffers = 0;
	mi.cached = 0;

	mi.swap_total = 0;
	mi.swap_free = 0;
	len = nitems(mib);
	if (sysctlnametomib("vm.swap_info", mib, &len) == -1)
		goto fail;
	for (i = 0; ; i++) {
		mib[2] = i;
		len = sizeof(xs);
		error = sysctl(mib, 3, &xs, &len, NULL, 0);
		if (error == -1)
			break;
		mi.swap_total += (uint64_t)xs.xsw_nblks * PAGE_SIZE / 1024;
		mi.swap_free += ((uint64_t)xs.xsw_nblks - xs.xsw_used) *
		    PAGE_SIZE / 1024;
	}
	return (mi);

fail:
	memset(&mi, 0, sizeof(mi));
	return (mi);
}

DiskInfo get_disk_info(void) {
	DiskInfo di;
	struct statfs *sfs, *sf;
	int i, nmounts;
	uint64_t dtotal, dfree;

	di.total = 0;
	di.free = 0;
	dtotal = 0;
	dfree = 0;
	sfs = NULL;

	nmounts = getfsstat(NULL, 0, MNT_WAIT);
	if (nmounts == -1)
		goto fail;
	sfs = calloc(nmounts, sizeof(*sfs));
	if (sfs == NULL)
		goto fail;
	nmounts = getfsstat(sfs, nmounts * sizeof(*sfs), MNT_WAIT);
	if (nmounts == -1)
		goto fail;

	for (i = 0; i < nmounts; i++) {
		sf = &sfs[i];
		if ((sf->f_flags & (MNT_LOCAL | MNT_IGNORE)) != MNT_LOCAL)
			continue;
		dtotal += sf->f_blocks * sf->f_bsize;
		dfree += sf->f_bfree * sf->f_bsize;
	}

	di.total = dtotal / 1000;
	di.free = dfree / 1000;

fail:
	free(sfs);
	return (di);
}
