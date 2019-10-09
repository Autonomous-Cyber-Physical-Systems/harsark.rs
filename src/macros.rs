#[macro_export]
macro_rules! spawn {
    ($task_name: ident, $priority: expr, $var: ident, $param: expr, $handler_fn: block) => {
        create_task($priority,|$var| loop {
            $handler_fn
            task_exit();
        },&$param).unwrap();
        static $task_name: TaskId = $priority;
    };
    ($task_name: ident, $priority: expr, $handler_fn: block) => {
        create_task($priority,|_| loop {
            $handler_fn
            task_exit();
        },&0).unwrap();
        static $task_name: TaskId = $priority;
    };
}

#[macro_export]
macro_rules! init {
    ($preemptive: expr, $param: expr, $handler_fn: expr) => {
        init($preemptive, false);
        create_task(0, $handler_fn, &$param).unwrap();
    };
    ($preemptive: expr) => {
        init($preemptive, true);
    };
}
