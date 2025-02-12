mod cgroups_integration;

fn main() {
    // Print RDR2-inspired welcome message
    println!("░▒▓▓▓▒░  OXIDEBOX  ░▒▓▓▓▒░");
    println!("    The Container Frontier\n");
    println!("Welcome to OxideBox, partner. Your journey into the container frontier begins now...\n");

    // Call the Cgroups integration function
    if let Err(e) = cgroups_integration::create_cgroup() {
        eprintln!("Failed to create cgroup: {}", e);
    }
}
