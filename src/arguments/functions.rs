use crate::config::functions::copy_default_config;

pub fn handle_arguments(args: Vec<String>) -> bool {
    let mut early_exit = false;

    for arg in args {
        early_exit = match arg.as_str() {
            "--help" | "-h" => print_usage(),
            "--version" | "-v" => print_version(),
            "--copy-default-config" => copy_default_config(),
            _ => continue,
        }
    }

    early_exit
}

fn print_usage() -> bool {
    println!("Usage: {} [options]", env!("CARGO_PKG_NAME"));
    println!("Options:");
    println!("  --help, -h       Show this help message");
    println!("  --version, -v    Show version information");

    true
}

fn print_version() -> bool {
    println!("version {}", env!("CARGO_PKG_VERSION"));

    true
}