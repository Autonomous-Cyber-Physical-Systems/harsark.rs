/// A helper struct to generate Boolean vector corresponding to an array of TaskIds at compile time.
pub struct TaskMask<const N: usize> {}

impl<const N: usize> TaskMask<N> {
    /// Takes an array of TaskIds and returns a BooleanVector corresponding to it.
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


pub const fn get_msb_const(val: u32) -> usize {
    let mut res = 0;
    let mut i = 0;
    while i < 32 {
        if val & (1<<i) > 0 {
            res = i;
        }
        i += 1;
    }
    return res;
}