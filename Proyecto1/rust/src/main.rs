use std::process::Command;
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
struct Container {
    pid: u32,
    name: String,
    cmd: String,
    memoria_kb: u32,
    uso_cpu_porcentaje: u32,
    comando: String,
    rss: u32,
    vsz: u32,
    uso_memoria_porcentaje: f32,
    created_at: String,
}

fn parse_sysinfo() -> Result<Value, Box<dyn std::error::Error>> {
    let output = Command::new("cat")
        .arg("/proc/sysinfo_202111849")
        .output()?;
    
    let sysinfo_json: Value = serde_json::from_slice(&output.stdout)?;
    Ok(sysinfo_json)
}

fn parse_docker_containers() -> Result<Vec<Container>, Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg("\"{{.ID}} {{.Names}} {{.Command}} {{.Size}} {{.Status}} {{.CreatedAt}}\"")
        .output()?;
    
    let docker_output = String::from_utf8_lossy(&output.stdout);
    let containers = docker_output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 6 {
                return None;
            }
            Some(Container {
                pid: parts[0].parse().unwrap_or(0),
                name: parts[1].to_string(),
                cmd: parts[2].to_string(),
                memoria_kb: parts[3].parse().unwrap_or(0),
                uso_cpu_porcentaje: parts[4].parse().unwrap_or(0),
                comando: parts[5].to_string(),
                rss: parts[6].parse().unwrap_or(0),
                vsz: parts[7].parse().unwrap_or(0),
                uso_memoria_porcentaje: parts[8].parse().unwrap_or(0.0),
                created_at: parts[9].to_string(),
            })
        })
        .collect::<Vec<Container>>();

    Ok(containers)
}

fn filter_and_delete_containers(containers: &mut Vec<Container>) {
    let mut grouped: HashMap<String, Vec<Container>> = HashMap::new();

    // Group containers by type (CPU, RAM, DISK, IO)
    for container in containers.iter() {
        if container.name.starts_with("stress") {
            let category = if container.name.contains("RAM") {
                "RAM"
            } else if container.name.contains("CPU") {
                "CPU"
            } else if container.name.contains("DISK") {
                "DISK"
            } else if container.name.contains("IO") {
                "IO"
            } else {
                continue;
            };
            grouped.entry(category.to_string())
                .or_insert(Vec::new())
                .push(container.clone());
        }
    }

    // Ensure one container per category is kept
    for (category, mut group) in grouped.iter_mut() {
        group.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        let latest = group.pop().unwrap(); // Keep the latest one
        println!("\nRetained container: {} ({})", latest.name, category);
        
        // Remove all other containers of that category
        group.iter().for_each(|container| {
            println!("Removing container: {} ({})", container.name, category);
            let _ = Command::new("docker")
                .arg("rm")
                .arg("-f")
                .arg(container.name.clone())
                .output();
        });
    }
}

fn main() {
    loop {
        match parse_sysinfo() {
            Ok(sysinfo) => {
                println!("\n--- SYSINFO ---");
                println!("Memory: Total_KB: {}, Libre_KB: {}, En_uso_KB: {}", 
                         sysinfo["Memoria"]["Total_KB"], 
                         sysinfo["Memoria"]["Libre_KB"], 
                         sysinfo["Memoria"]["En_uso_KB"]);
                println!("CPU Usage: {}%", sysinfo["CPU"]["Uso_Porcentaje"]);

                match parse_docker_containers() {
                    Ok(mut containers) => {
                        filter_and_delete_containers(&mut containers);
                    },
                    Err(e) => {
                        eprintln!("Error parsing Docker containers: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error parsing sysinfo: {}", e);
            }
        }

        // Wait for 10 seconds before checking again
        thread::sleep(Duration::from_secs(10));
    }
}
