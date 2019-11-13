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

// Ensure that is_privileged has been imported into scope
#[macro_export]
macro_rules! priv_execute {
    ($handler: block) => {
        return match is_privileged() {
            false => Err(KernelError::AccessDenied),
            true => $handler,
        };
    };
}
