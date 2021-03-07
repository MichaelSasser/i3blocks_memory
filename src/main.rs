#![crate_name = "i3blocks_memory"]
use regex::Regex;
use std::fs;

/// A representation of the random access memory.
struct Memory {
    total: f32,
    free: f32,
    buffers: f32,
    cache: f32,
    sreclaimable: f32,
    used: f32,
    percent: f32,
}

/// Initialize `Memory` with 0.0: f32
impl Default for Memory {
    fn default() -> Memory {
        Memory {
            total: 0.0,
            free: 0.0,
            buffers: 0.0,
            cache: 0.0,
            sreclaimable: 0.0,
            used: 0.0,
            percent: 0.0,
        }
    }
}

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("/proc/meminfo").expect("Unable to read from /proc/meminfo");
    let re = Regex::new(r"([\w()]+:)\s+(\d+)").unwrap();
    let mut lstr = String::from(" ");
    let mut rstr = String::from("");
    let mut mem = Memory {
        ..Default::default()
    };
    for line in data.lines() {
        let value = re
            .captures(line)
            .expect("<span color='red'> MEMORY NOT FOUND!</span>");
        match &value[1] {
            "MemTotal:" => mem.total = value[2].parse::<f32>().expect("0.0"),
            "MemFree:" => mem.free = value[2].parse::<f32>().expect("0.0"),
            "Buffers:" => mem.buffers = value[2].parse::<f32>().expect("0.0"),
            "Cached:" => mem.cache = value[2].parse::<f32>().expect("0.0"),
            "SReclaimable:" => mem.sreclaimable = value[2].parse::<f32>().expect("0.0"),
            _ => (),
        }
    }

    if mem.total == 0.0 {
        eprintln!("<span color='red'>ERROR, reading \"/proc/meminfo\"</span>");
        std::process::exit(1);
    }
    mem.cache += mem.sreclaimable;
    mem.used = (mem.total - mem.free - mem.buffers - mem.cache) * 1.0E-6;
    mem.total *= 1.0E-6;
    mem.percent = mem.used * 100.0 / mem.total;

    if mem.percent > 95.0 {
        lstr.push_str("<span color='red'>");
        rstr.push_str("</span>");
    } else if mem.percent > 80.0 {
        lstr.push_str("<span color='yellow'>");
        rstr.push_str("</span>");
    }
    println!(
        "{}{: >5.2}GB/{: >5.2}GB ({: >5.2}%{})",
        lstr, mem.used, mem.total, mem.percent, rstr
    );
    return Ok(());
}
