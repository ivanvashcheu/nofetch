use clap::Parser;
use colored::*;
use std::env;
use sysinfo::{Disks, System};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    no_icons: bool,
}

fn main() {
    let args = Args::parse();

    let mut sys = System::new();
    sys.refresh_memory();
    
    let uptime_seconds = System::uptime();
    let kernel_version = System::kernel_version().unwrap_or("unknown".to_string());

    let user = env::var("USER").unwrap_or_else(|_| "User".to_string());
    
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
    let shell = shell_path.split('/').last().unwrap_or(&shell_path);

    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    let disks = Disks::new_with_refreshed_list();
    let root_disk = disks.list().iter().find(|d| d.mount_point() == std::path::Path::new("/"));
    
    let disk_info = match root_disk {
        Some(d) => {
            let total_space = d.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let available_space = d.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_space = total_space - available_space;
            format!("{:.1}/{:.1} GB", used_space, total_space)
        },
        None => "N/A".to_string(),
    };

    let up_hours = uptime_seconds / 3600;
    let up_mins = (uptime_seconds % 3600) / 60;

    let (ic_user, ic_kernel, ic_up, ic_shell, ic_mem, ic_disk) = if args.no_icons {
        ("", "", "", "", "", "")
    } else {
        ("👋 ", "🐧 ", "⏱️  ", "🐚 ", "💾 ", "💿 ")
    };

    println!(
        "{}{} {} {} {} {} {} {} {} {} {} {}",
        ic_user.yellow(), user.bold().yellow(),
        "|".dimmed(),
        format!("{}{}", ic_kernel, kernel_version).blue(),
        "|".dimmed(),
        format!("{}{:02}h:{:02}m", ic_up, up_hours, up_mins).green(),
        "|".dimmed(),
        format!("{}{}", ic_shell, shell).magenta(),
        "|".dimmed(),
        format!("{}{:.1}/{:.1} GiB", ic_mem, used_mem, total_mem).cyan(),
        "|".dimmed(),
        format!("{}{}", ic_disk, disk_info).red()
    );
}