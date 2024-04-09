use std::sync::{mpsc, Mutex, Once};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static INIT: Once = Once::new();
static mut EXECUTOR: Option<Mutex<Executor>> = None;

pub struct Task {
    kind: Box<dyn FnOnce() + Send + 'static>,
}

impl Task {
    pub fn new<F: FnOnce() + Send + 'static>(f: F) -> Self {
        Task { kind: Box::new(f) }
    }
}

pub struct Executor {
    task_sender: Sender<Task>,
}

impl Executor {
    pub fn new() -> Self {
        let (tx, rx): (Sender<Task>, Receiver<Task>) = mpsc::channel();
        thread::spawn(move || {
            for task in rx {
                (task.kind)();
            }
        });
        Executor { task_sender: tx }
    }

    pub fn spawn<F: FnOnce() + Send + 'static>(&self, f: F) {
        let task = Task::new(f);
        self.task_sender.send(task).expect("Task send failed");
    }
}

pub fn global_executor() -> &'static Mutex<Executor> {
    unsafe {
        INIT.call_once(|| {
            EXECUTOR = Some(Mutex::new(Executor::new()));
        });
        EXECUTOR.as_ref().unwrap()
    }
}

pub fn spawn<F: FnOnce() + Send + 'static>(f: F) {
    global_executor().lock().unwrap().spawn(f);
}