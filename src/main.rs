use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    projects: Option<Vec<Project>>,
    vaults: Option<Vec<Vault>>,
}

#[derive(Deserialize)]
struct Project {
    name: String,
    path: String,
}

#[derive(Deserialize)]
struct Vault {
    name: String,
    path: String,
}

fn main() {
    // Read config file
    let config_str = std::fs::read_to_string("devsheet.toml")
        .expect("Could not find devsheet.toml — make sure it exists in the same folder");
    
    let config: Config = toml::from_str(&config_str)
        .expect("Could not parse devsheet.toml — check the format");

    // Process projects
    if let Some(projects) = &config.projects {
        for project in projects {
            println!("\n=== {} ===\n", project.name);
            
            // Git commits
            let output = Command::new("git")
                .args(["log", "--pretty=format:%s", "-5"])
                .current_dir(&project.path)
                .output()
                .expect("Failed to run git");
            
            println!("Last 5 commits:");
            for (i, line) in String::from_utf8_lossy(&output.stdout).lines().enumerate() {
                println!("  {}. {}", i + 1, line);
            }

            // Endpoints
            let endpoints = Command::new("grep")
                .args(["-r", "--include=*.rs", "route(", &format!("{}/Backend", project.path)])
                .output()
                .expect("Failed to grep endpoints");
            
            println!("\nEndpoints:");
            for line in String::from_utf8_lossy(&endpoints.stdout).lines() {
                if line.contains(".route(") {
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() >= 3 {
                        let method = if line.contains("get(") { "GET" } else { "POST" };
                        println!("  {} {}", method, parts[1]);
                    }
                }
            }

            // Source files
            let files = Command::new("find")
                .args([&project.path, "-type", "f", "(", "-name", "*.rs", "-o", "-name", "*.tsx", ")", "-not", "-path", "*/target/*"])
                .output()
                .expect("Failed to run find");
            
            println!("\nSource files:");
            for line in String::from_utf8_lossy(&files.stdout).lines() {
                let filename = line.split('/').last().unwrap_or(line);
                println!("  {}", filename);
            }
        }
    }

    // Process vaults
    if let Some(vaults) = &config.vaults {
        for vault in vaults {
            println!("\n=== {} (Second Brain) ===\n", vault.name);
            
            let notes = Command::new("find")
                .args([&vault.path, "-type", "f", "-name", "*.md"])
                .output()
                .expect("Failed to read vault");
            
            println!("Notes:");
            for line in String::from_utf8_lossy(&notes.stdout).lines() {
                let filename = line.split('/').last().unwrap_or(line);
                println!("  {}", filename);
            }
        }
    }
}