# Wrap Up: Fearless Concurrency

Rust provides a *two* powerful concurrency models: system threads and async. Each has its strengths and weaknesses, and each is suited to different tasks. But both are *fearless*---they prevent data races and ensure memory safety without needing a garbage collector.