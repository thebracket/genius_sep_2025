# Counting Identical Items

A common task in programming is to count the number of occurrences of identical items in a collection. This can be done in several ways, but some methods are more efficient than others.

For example, say you have a number of strings, and you want to count how many times each string appears.

The most common implementation would be something like this:

```rust
use std::collections::HashMap;

fn count_items(items: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for item in items {
        let counter = counts.entry(item.clone()).or_insert(0);
        *counter += 1;
    }
    counts
}

fn main() {
    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apple".to_string(),
        "orange".to_string(),
        "banana".to_string(),
        "apple".to_string(),
        "grape".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
        "peach".to_string(),
        "pear".to_string(),
        "plum".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
        "strawberry".to_string(),
        "cherry".to_string(),
    ];

    let start = std::time::Instant::now();
    let counts = count_items(&items);
    let duration = start.elapsed();
    println!("Time elapsed in count_items() is: {:?}", duration);

    for (item, count) in counts {
        println!("{item}: {count}");
    }
}
```

It's *massively* faster to sort the strings collection first, and then count the identical items in a single pass:

```rust
// Sort first, then count identical adjacent items in one pass
fn count_items_sorted(mut items: Vec<String>) -> Vec<(String, usize)> {
    // Sorting groups identical strings together
    items.sort_unstable();

    let mut out: Vec<(String, usize)> = Vec::new();
    let mut iter = items.into_iter();

    if let Some(mut current) = iter.next() {
        let mut count = 1usize;
        for s in iter {
            if s == current {
                count += 1;
            } else {
                out.push((current, count));
                current = s;
                count = 1;
            }
        }
        // Push the last run
        out.push((current, count));
    }

    out
}

fn main() {
    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apple".to_string(),
        "orange".to_string(),
        "banana".to_string(),
        "apple".to_string(),
        "grape".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
        "peach".to_string(),
        "pear".to_string(),
        "plum".to_string(),
        "kiwi".to_string(),
        "mango".to_string(),
        "strawberry".to_string(),
        "cherry".to_string(),
    ];

    let start = std::time::Instant::now();
    let counts = count_items_sorted(items);
    let duration = start.elapsed();
    println!("Time elapsed in count_items_sorted() is: {:?}", duration);

    for (item, count) in counts {
        println!("{item}: {count}");
    }
}
```
