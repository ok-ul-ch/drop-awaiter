# drop-awaiter
Library that allows you to asynchronously wait for something to be dropped.

## Docs

[Rustdoc](https://docs.rs/drop-awaiter/0.1.0/drop_awaiter/index.html)

## Examples

```rust
    struct Task {
        id: &'static str,
        dn: DropNotifier,
    }
 
    async fn foo() {
        let (n, awaiter) = crate::new();

        let first_task = Task {
            id: "first",
            dn: n.clone(),
        };
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(5));
            println!("Completed task: {}", first_task.id);
            drop(first_task.dn);
        });

        let second_task = Task {
            id: "second",
            dn: n,
        };

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Completed task: {}", second_task.id);
            drop(second_task.dn);
        });

        // Simply await for completion of tasks (both sync and async) without 
        // any additional collections to coordinate and track statuses of tasks
        awaiter.await;
    }
```
