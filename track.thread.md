## thread

创建线程：使用 thread::spawn 创建新线程。该函数接收一个闭包，闭包中的代码将在新线程中执行。
线程通信：通过消息传递（如 std::sync::mpsc 频道）或共享状态（如 `Arc<Mutex<T>>`）来在线程间通信。
线程同步：使用 join 方法等待线程结束，确保主线程不在子线程完成之前退出。

```rs
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    // 创建一个线程并等待其完成
    let handle = thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    handle.join().unwrap(); // 等待线程结束

    // 使用 mpsc 频道在线程间传递消息
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from another thread!").unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());

    // 使用 Arc 和 Mutex 实现共享状态
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
```

关键概念解释：
thread::spawn：创建新线程并执行闭包中的代码。
join：主线程等待子线程完成。
mpsc：多生产者，单消费者通道，用于线程间通信。
Arc（Atomic Reference Counting）：允许多线程安全地共享数据。
Mutex（互斥量）：用于保护共享数据，确保同一时间只有一个线程能访问数据。

使用场景：
当需要并发处理任务时，如 I/O 操作或并行计算。
当需要在不同线程间共享数据时，使用 Arc 和 Mutex 进行同步。
