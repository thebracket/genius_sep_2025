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
