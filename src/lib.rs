use std::{cd 
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
};

use futures::{task::AtomicWaker, Future};

/// ```
/// async fn foo() {
///     let (notifier_1, awaiter) = drop_awaiter::spawn();
///     
///     let notifier_2 = notifier_1.clone();
///
///     std::thread::spawn(move || {
///         // Perform task 1
///         // ...
///         drop(notifier_1);    
///     });
///     
///     std::thread::spawn(move || {
///         // Perform task 2    
///         // ...
///         drop(notifier_2);    
///     });
///     
///
///     awaiter.await
/// }

pub fn spawn() -> (DropNotifier, DropAwaiter) {
    let state = Arc::new(State {
        awaiter_waker: AtomicWaker::new(),
        notifiers_count: AtomicUsize::new(1),
    });

    (
        DropNotifier {
            state: state.clone(),
        },
        DropAwaiter { state },
    )
}

pub struct DropAwaiter {
    state: Arc<State>,
}

pub struct DropNotifier {
    state: Arc<State>,
}

impl Future for DropAwaiter {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.notifiers_count.load(Ordering::SeqCst) == 0 {
            Poll::Ready(())
        } else {
            self.state.awaiter_waker.register(cx.waker());
            Poll::Pending
        }
    }
}

impl Clone for DropNotifier {
    fn clone(&self) -> Self {
        self.state.notifiers_count.fetch_add(1, Ordering::Relaxed);

        Self {
            state: self.state.clone(),
        }
    }
}

impl Drop for DropNotifier {
    fn drop(&mut self) {
        if self.state.notifiers_count.fetch_sub(1, Ordering::AcqRel) != 1 {
            return;
        }

        self.state.awaiter_waker.wake();
    }
}

struct State {
    notifiers_count: AtomicUsize,
    awaiter_waker: AtomicWaker,
}