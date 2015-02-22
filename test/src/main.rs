extern crate "sys-info" as sys_info;

fn main() {

    println!("os: {} {}", sys_info::os_type().unwrap(), sys_info::os_release().unwrap());
    println!("cpu: {} cores, {} MHz", sys_info::cpu_num().unwrap(), sys_info::cpu_speed().unwrap());
    println!("proc total: {}", sys_info::proc_total().unwrap());
    let load = sys_info::loadavg().unwrap();
    println!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = sys_info::mem_info().unwrap();
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
             mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
    println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
    let disk = sys_info::disk_info().unwrap();
    println!("disk: total {} KB, free {} KB", disk.total, disk.free);

}
