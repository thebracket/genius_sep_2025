# Fearful Concurrency with Python

Python has been improving on this front, but let's run the following program:

```python
import threading

COUNTER = 0

def one():
    return 1;

def worker():
    global COUNTER
    for _ in range(500_000):
        COUNTER += one()


def main():
    threads = [threading.Thread(target=worker) for _ in range(10)]

    for t in threads:
        t.start()

    for t in threads:
        t.join()

    print("COUNTER:", COUNTER)

if __name__ == "__main__":
    main()
```

> The code is in `code/python/concurrency/oops.py`

Running it a few times on my office workstation gives:

```
COUNTER: 1797138
COUNTER: 2869379
COUNTER: 3179388
COUNTER: 3568894
COUNTER: 3265008
COUNTER: 2885382
```

It's never the same - and Python didn't try and warn you! This is a *race condition*. Incrementing the counter is actually a 3 step operation: read the value, add one, write it back. If two threads read the value at the same time, they will both write back the same value + 1, losing one of the increments.