pub fn generate_task_mask(tasks: &[u32]) -> u32 {
    let mut task_mask = 0;
    for tid in tasks {
        task_mask |= 1 << *tid;
    }
    task_mask
}
