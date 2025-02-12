use cgroups_rs::{cgroup_builder::CgroupBuilder, hierarchies::auto, CpuController, MemoryController};

pub fn create_cgroup() -> Result<(), Box<dyn std::error::Error>> {
    // Acquire a handle for the cgroup hierarchy.
    let hier = auto();

    // Create a control group named "example" in the V1 hierarchy.
    let cg = CgroupBuilder::new("example")
        .cpu()
        .shares(85)
        .done()
        .memory()
        .limit_in_bytes(100 * 1024 * 1024) // 100 MB
        .done()
        .build(hier);

    // Get handles to the controllers.
    let cpu: &CpuController = cg.controller_of().unwrap();
    let memory: &MemoryController = cg.controller_of().unwrap();

    // Apply resource limits.
    cpu.set_shares(85);
    memory.set_limit_in_bytes(100 * 1024 * 1024);

    // Add the current process to the cgroup.
    let pid = std::process::id();
    cg.add_task(pid)?;

    Ok(())
}
