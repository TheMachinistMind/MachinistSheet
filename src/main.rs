use std::process::Command;
use std::env;

fn main() {
    // First argument is the project path, defaults to current directory
    let path = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    
    println!("=== DEVSHEET ===");
    println!("Project: {}\n", path);
    
    // Get last 5 git commits
    let output = Command::new("git")
        .args(["log", "--pretty=format:%s", "-5"])
        .current_dir(&path)
        .output()
        .expect("Failed to run git");
    
    println!("Last 5 commits:");
    for (i, line) in String::from_utf8_lossy(&output.stdout).lines().enumerate() {
        println!("  {}. {}", i + 1, line);
    }
    
    // Find route definitions, exclude compiled files
    let endpoints = Command::new("grep")
        .args(["-r", "--include=*.rs", "route(", &format!("{}/Backend", path)])
        .output()
        .expect("Failed to grep endpoints");
    
    println!("Endpoints:");
    for line in String::from_utf8_lossy(&endpoints.stdout).lines() {
        if line.contains(".route(") {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 3 {
                let path_part = parts[1];
                let method = if line.contains("get(") { "GET" } else { "POST" };
                println!("  {} {}", method, path_part);
            }
        }
    }
    
    // List source files, exclude target folder
    let files = Command::new("find")
        .args([&path, "-type", "f", "(", "-name", "*.rs", "-o", "-name", "*.tsx", ")", "-not", "-path", "*/target/*"])
        .output()
        .expect("Failed to run find");
    
    println!("\nSource files:");
    for line in String::from_utf8_lossy(&files.stdout).lines() {
        let filename = line.split('/').last().unwrap_or(line);
        println!("  {}", filename);
    }
}

