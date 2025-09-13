# Async in Rust vs. Other Languages

Async Rust is similar to async in other languages: tasks/futures represent units of work, and the executor is responsible for scheduling and running them. Rust's model is different from Erlang, C#, Python, JavaScript, etc. - like C++, Rust has to work on anything from a wristwatch to a supercomputer. So Rust *did not* include an executor/runtime. Instead, you have to choose one. Tokio is extremely popular, but there are others --- Embassy even scales down to microcontrollers.

Tokio --- which we'll use today --- is a full featured runtime, with timers, async file and network I/O, and more. It also includes a multi-threaded executor, so you can take advantage of multiple CPU cores.

In many ways, it's pretending to be Go (which is pretending to be Erlang without the reliability guarantees). It uses a work-stealing scheduler to distribute tasks across multiple threads. This means that if one thread is busy, another thread can "steal" tasks from it to keep things moving.