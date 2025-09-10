use std::collections::HashMap;
use serde_json;
use sysinfo::System;

const CPUS_MAX:i32=4;
const RAMS_MAX:u64=8589934592;
const UPTIME_MAX:u64=7200;
fn get_cpus()->i32{
    let cpus=System::physical_core_count();
    match cpus {
        Some(res)=>{return res as i32;},
        None=>{return 0;}
    }
}
pub fn judge_cpus()->bool{
    let cpus=get_cpus();
    if cpus>=CPUS_MAX{
        true
    }else{
        false
    }
}
fn get_rams(sys:&System)->u64{
    sys.total_memory()
}
pub fn judge_rams(sys:&System)->bool{
    let rams=get_rams(sys);
    if rams>=RAMS_MAX{
        true
    }else{
        false
    }
}
fn get_uptime()->u64{
    System::uptime()
}
pub fn judge_uptime()->bool{
    let uptime=get_uptime();
    if uptime>=UPTIME_MAX{
        true
    }else{
        false
    }
}
fn get_os_name()->String{
    let os_name=System::long_os_version();
    match os_name {
        Some(res)=>{res},
        None=>{"N/A".to_string()}
    }
}
fn get_host_name()->String{
    let host_name=System::host_name();
    match host_name {
        Some(res)=>{res},
        None=>{"N/A".to_string()}
    }
}
fn get_cpu_arch()->String{
    System::cpu_arch()
}
fn get_all(sys:&System)->HashMap<String,String>{
    let mut res:HashMap<String,String>=HashMap::new();

    let host_name=get_host_name();
    let cpus=get_cpus().to_string();
    let rams=(get_rams(sys)/(1024*1024*1024)).to_string()+"GB";
    let os_name=get_os_name();
    let cpu_arch=get_cpu_arch();
    let uptime=(get_uptime()/(60*60)).to_string()+"hours";

    res.insert("host_name".to_string(), host_name);
    res.insert("os".to_string(), os_name);
    res.insert("cpu_arch".to_string(),cpu_arch);
    res.insert("cpus".to_string(), cpus);
    res.insert("rams".to_string(), rams);
    res.insert("uptime".to_string(), uptime);

    res
}
pub fn get_all_json(sys:&System)->Result<String,serde_json::Error>{
    let all=get_all(sys);
    serde_json::to_string_pretty(&all)
}
pub fn is_sandbox(sys:&System)->bool{
    let cpus=judge_cpus();
    let rams=judge_rams(sys);
    let uptime=judge_uptime();

    return !(cpus&rams&uptime)
}