# Println Debugging - There's Nothing Wrong with It

From the Linux Kernel to large-scale web applications, `println` debugging is everywhere. It's simple, effective, and requires no special tools or setup. I tend to use it as my first line of defense when tracking down a bug.

When you have a bug, you are usually trying to trace a specific set of inputs that gave an invalid output (or failed). So you might add a `println!` in the suspect function that logs the inputs and a second `println!` that logs the output. If you have a series of branches, you might add a `println!` in each branch to see which one is taken. You can also log intermediate values to see where things go wrong.

This isn't rocket science - but it's *remarkably effective*. It also avoids attaching debuggers to production systems!

Here's a simple example:

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    println!("divide() called with numerator: {}, denominator: {}", numerator, denominator);
    if denominator == 0.0 {
        println!("denominator is zero, returning None");
        None
    } else {
        let result = numerator / denominator;
        println!("denominator is non-zero, returning Some({})", result);
        Some(result)
    }
}
```

In this example, we log the inputs to the `divide` function, and then log which branch is taken (denominator is zero or non-zero) and the result if applicable. This can help us trace through the function and see where things might be going wrong.