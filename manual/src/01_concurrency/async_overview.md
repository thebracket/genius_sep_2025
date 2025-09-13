# Async

System threads are heavyweight, relying on the operating system for scheduling. Switching between threads involves a *context switch*: store the current thread's state, load the new thread's state, and switch execution to the new thread. This is a relatively expensive operation.

You probably don't want to have thousands of threads. On some systems, hundreds of threads can be a problem.

Async, however, is designed for massive numbers of lightweight "tasks"---and can even be run single-threaded. Instead of relying on the operating system to schedule threads, async uses a runtime to schedule tasks. This is often called "green threading" or "user-space threading".