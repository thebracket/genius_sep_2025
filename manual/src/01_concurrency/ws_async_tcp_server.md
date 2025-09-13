# WS: Async TCP Server

So with what we've learned, let's turn our threaded TCP server into an async TCP server.

```rust
use tokio::io::AsyncWriteExt; // Instead of std::io::Write
use tokio::io::AsyncReadExt; // Instead of std::io::Read

async fn server() -> anyhow::Result<()> {
    // Use Tokio's TcpListener instead of std::net::TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        // Accept a new connection asynchronously
        // This will yield until a connection is available
        let (mut socket, _) = listener.accept().await?;

        // We're spawning a new task for each connection instead of a thread
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => break, // Connection closed
                    Ok(n) => {
                        if socket.write_all(&buffer[..n]).await.is_err() {
                            break; // Write error
                        }
                    }
                    Err(_) => break, // Read error
                }
            }
        });
    }
    //Ok(()) - Unreachable, server runs forever
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // We spawn the server in a task, so we can also run a client in the same program
    tokio::spawn(async {
        if let Err(e) = server().await {
            eprintln!("Server error: {:?}", e);
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Give server time to start

    // Using tokio's TcpStream instead of std::net::TcpStream
    match tokio::net::TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let msg = b"Hello, world!";
            stream.write_all(msg).await?;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
        }
        Err(e) => {
            eprintln!("Failed to connect: {:?}", e);
        }
    }

    Ok(())
}
```

Notice that it's basically the same, but with `await` everywhere and async versions of the types and methods. The server can now handle many connections concurrently without spawning a thread for each one, making it much more scalable.

> We even specified single thread. You can handle thousands of clients on a single thread!