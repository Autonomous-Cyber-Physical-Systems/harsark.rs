/// Takes a list of TaskIds and returns a BooleanVector corresponding to it.
pub struct TaskMask<const N: usize> {}

impl<const N: usize> TaskMask<N> {
    pub const fn generate(tasks: [u32; N]) -> u32{
        let mut task_mask: u32 = 0;
        let mut i = 0;
        while i < N {
            task_mask |= 1<<tasks[i];
            i += 1;
        }
        task_mask
    }
}