use std::fs;
use std::path::Path;

fn main() {
    let cpu_max_path = "/sys/fs/cgroup/cpu.max";
    println!("---performance Spy---");

    if Path::new(cpu_max_path).exists() {
        match fs::read_to_string(cpu_max_path) {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 2 {
                    let quota = parts[0];
                    let period = parts[1];

                    println!("Quota: {}, Period: {}", quota, period);

                    if quota != "max" {
                        let q: f64 = quota.parse().unwrap_or(0.0);
                        let p: f64 = period.parse().unwrap_or(100000.0);
                        println!("Calculated Limit: {:.2} cores", q / p);
                    } else {
                        println!("Result: Unlimited (No Throttling risk)");
                    }
                }
            }
            Err(e) => println!("Error reading cgroup: {}", e),
        }
    } else {
        println!("Status: Running on Local (Cgroup v2 not found)");
        println!("Note: To test this, we need a Linux container environment");
    }
}
