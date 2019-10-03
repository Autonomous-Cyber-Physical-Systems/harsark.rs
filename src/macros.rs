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
        },0).unwrap();
        static $task_name: TaskId = $priority;
    };
}
