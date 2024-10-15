use crate::task::Task;
use std::{
    cell::RefCell,
    future::Future,
    sync::{mpsc, Arc},
};

thread_local! {
    static MINI_TOKIO_INSTANCE: RefCell<Option<mpsc::Sender<Arc<Task>>>> =
        RefCell::new(None);
}

// An equivalent to `tokio::spawn`. When entering the mini-tokio executor, the
// `CURRENT` thread-local is set to point to that executor's channel's Send
// half. Then, spawning requires creating the `Task` harness for the given
// `future` and pushing it into the scheduled queue.
pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    MINI_TOKIO_INSTANCE.with(|cell| {
        let borrow = cell.borrow();
        let sender = borrow.as_ref().unwrap();
        Task::spawn(future, sender);
    });
}

/// A very basic futures executor based on a channel. When tasks are woken, they
/// are scheduled by queuing them in the send half of the channel. The executor
/// waits on the receive half and executes received tasks.
///
/// When a task is executed, the send half of the channel is passed along via
/// the task's Waker.
pub struct MiniTokio {
    // Receives scheduled tasks. When a task is scheduled, the associated future
    // is ready to make progress. This usually happens when a resource the task
    // uses becomes ready to perform an operation. For example, a socket
    // received data and a `read` call will succeed.
    scheduled: mpsc::Receiver<Arc<Task>>,

    // Send half of the scheduled channel.
    pub sender: mpsc::Sender<Arc<Task>>,
}

impl MiniTokio {
    /// Initialize a new mini-tokio instance.
    pub fn new() -> MiniTokio {
        let (sender, scheduled) = mpsc::channel();
        MiniTokio { scheduled, sender }
    }

    /// Spawn a future onto the mini-tokio instance.
    ///
    /// The given future is wrapped with the `Task` harness and pushed into the
    /// `scheduled` queue. The future will be executed when `run` is called.
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    /// Run the executor.
    ///
    /// This starts the executor loop and runs it indefinitely. No shutdown
    /// mechanism has been implemented.
    ///
    /// Tasks are popped from the `scheduled` channel receiver. Receiving a task
    /// on the channel signifies the task is ready to be executed. This happens
    /// when the task is first created and when its waker has been used.
    pub fn run(&self) {
        // Set the CURRENT thread-local to point to the current executor.
        //
        // Tokio uses a thread-local variable to implement `tokio::spawn`. When
        // entering the runtime, the executor stores necessary context with the
        // thread-local to support spawning new tasks.
        MINI_TOKIO_INSTANCE.with(|cell| {
            *cell.borrow_mut() = Some(self.sender.clone());
        });

        // The executor loop. Scheduled tasks are received. If the channel is
        // empty, the thread blocks until a task is received.
        while let Ok(task) = self.scheduled.recv() {
            // Execute the task until it either completes or cannot make further
            // progress and returns `Poll::Pending`.
            task.poll();
        }
    }
}
