#[macro_export]
macro_rules! spawn {
    ($task_name: ident, $priority: expr, $stack: expr, $var: ident, $param: expr, $handler_fn: block) => {
        create_task(
            $priority,
            unsafe{ &mut $stack },
            |$var| loop {
                $handler_fn
                task_exit();
        },&$param).unwrap();
        static $task_name: TaskId = $priority;
    };
    ($task_name: ident, $priority: expr, $stack: expr, $handler_fn: block) => {
        create_task(
            $priority,
            unsafe{ &mut $stack },
            |_| loop {
                $handler_fn
                task_exit();
        },&0).unwrap();
        static $task_name: TaskId = $priority;
    };
}

#[macro_export]
macro_rules! init {
    ($preemptive: expr, $stack: expr, $param: expr, $handler_fn: expr) => {
        init($preemptive, false);
        create_task(0, unsafe{ &mut $stack },$handler_fn, &$param).unwrap();
    };
    ($preemptive: expr) => {
        init($preemptive, true);
    };
}
