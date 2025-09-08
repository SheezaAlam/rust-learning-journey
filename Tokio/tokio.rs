
* **Rust alone** runs one thing at a time.
* **Tokio** is an async **runtime** and utilities so many tasks can run concurrently by *pausing* on I/O/timers without blocking the OS thread.
* `async` marks a function or block that *returns a Future* — it may pause.
* `.await` pauses the current async task until the awaited Future is ready (but the runtime can run other tasks while paused).
* `tokio::spawn(...)` schedules an async task to run concurrently (like a lightweight thread).
* `#[tokio::main]` sets up and runs a Tokio runtime and allows `async fn main()`.

Good mental model: Tokio is a traffic manager — it pauses tasks waiting for I/O/timers and runs other tasks in the meantime.

---

## Complete, ready-to-run echo server (clean & fully commented)

Create `Cargo.toml` with Tokio dependency:

```toml
[package]
name = "tokio_echo"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
```

`src/main.rs` (copy this exactly):

```rust
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind a TCP listener on localhost:8080 (only reachable from this machine).
    // `.await` yields to the runtime while the OS sets up the socket.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server listening on 127.0.0.1:8080");

    // Accept loop: run forever, waiting for incoming connections.
    loop {
        // Wait for a client. When a client connects we get a socket and peer address.
        let (mut socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);

        // Spawn a new task to handle this client concurrently.
        // `async move` moves `socket` and `addr` into the task so each connection owns its state.
        tokio::spawn(async move {
            // small buffer for reading data from the client
            let mut buf = [0u8; 1024];

            loop {
                // Read from the socket asynchronously into the buffer.
                // `n` is the number of bytes read.
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        // `Ok(0)` means the client closed the connection cleanly.
                        break;
                    }
                    Ok(n) => {
                        // Echo the exact bytes back to the client.
                        // `write_all` ensures all bytes are sent (it is async).
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("Failed to write to {}: {}", addr, e);
                            break;
                        }
                    }
                    Err(e) => {
                        // Read failed (connection reset or other I/O error).
                        eprintln!("Failed to read from {}: {}", addr, e);
                        break;
                    }
                }
            } // inner loop: handle single client until it disconnects
            println!("Client {} disconnected", addr);
        }); // end tokio::spawn
    } // end accept loop
}
```

**Why this exact structure?**

* `TcpListener::bind(...).await?` — bind is async and may fail (port in use), so we `await` and `?` propagate errors to `main`.
* `listener.accept().await?` — accept blocks until a client connects (non-blocking to the OS thread because runtime schedules).
* `tokio::spawn(async move { ... })` — each client runs in its own task so many can be served concurrently.
* `socket.read(...).await` / `socket.write_all(...).await` — async I/O calls; the task yields while waiting for network.
* `Ok(0)` from `read` — indicates graceful disconnect.
* `async move` — moves socket into the spawned closure so the task owns it (avoids borrow/lifetime issues).

---

## How to run + test on Windows (step-by-step)

1. Build & run the server (PowerShell):

   ```powershell
   cargo run
   ```

   In server window you should see:

   ```
   Echo server listening on 127.0.0.1:8080
   ```

2. Open a new terminal (PowerShell) for the client tests.

### Option A — If you have `nc` (netcat) or `ncat`:

```powershell
nc 127.0.0.1 8080
# type: hello
# you should see: hello (echoed back)
```

### Option B — Using Python (works on any Windows with Python installed)

Run this one-liner:

```powershell
python - <<'PY'
import socket
s = socket.socket()
s.connect(("127.0.0.1", 8080))
s.sendall(b"hello tokio\n")
print(s.recv(1024))  # prints the echoed bytes
s.close()
PY
```

### Option C — `curl` (client will connect, send a request, then exit)

```powershell
curl --data "hello tokio" http://127.0.0.1:8080
```

This will send data; your server will accept and echo, and `curl` will display the response.

### Option D — PowerShell TcpClient quick test

```powershell
# connect, send "hello", read response
$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1",8080)
$stream = $client.GetStream()
$bytes = [System.Text.Encoding]::UTF8.GetBytes("hello from PS`n")
$stream.Write($bytes,0,$bytes.Length)
$buffer = New-Object byte[] 1024
$read = $stream.Read($buffer, 0, $buffer.Length)
[System.Text.Encoding]::UTF8.GetString($buffer,0,$read)
$stream.Close(); $client.Close()
```

---

## Common pitfalls & how to avoid them

* **Using `std::thread::sleep` in async code** — that blocks the OS thread. Always use `tokio::time::sleep(...).await`.
* **Import name collisions** (e.g., `sleep` from both std and tokio). Import only the one you need.
* **Blocking operations inside tasks** — avoid CPU-heavy blocking (use `tokio::task::spawn_blocking` if necessary).
* **Forgetting `move` in `async move`** — without `move`, borrowed values may not live long enough. `async move` transfers ownership into the task.
* **Not handling partial reads** — `read` may return fewer bytes than requested; code above slices to `&buf[..n]`.
* **Exiting main prematurely** — if `main` ends, background tasks may be cancelled. Server keeps `main` alive with the accept loop.

---

## Next steps / practice tasks (tiny progression)

1. **Add welcome message** — send `"Welcome!\n"` to a client immediately after `accept`.
2. **Log message length** — count bytes received per client and print summary when they disconnect.
3. **Make a line-based echo** — use `tokio::io::BufReader` + `read_line` to echo line-by-line so sending newline triggers immediate echo.
4. **Graceful shutdown** — detect Ctrl+C and close listener gracefully (`tokio::signal::ctrl_c()`).
5. **Chat server** — store a list of connected clients (use `tokio::sync::broadcast` or `Mutex<Vec<Sender>>`) and broadcast each message to all others.
6. **Use codecs** — learn `tokio-util::codec::{Framed, LinesCodec}` for clean line-based protocols.

---

## Short cheat-sheet (what you’ll use every day)

* `#[tokio::main]` — start runtime so `async fn main()` works.
* `tokio::spawn(async move { ... })` — run tasks concurrently.
* `TcpListener::bind(...).await?` — open server socket.
* `listener.accept().await?` — wait for a client.
* `socket.read(&mut buf).await` — async read, returns `Ok(n)` or `Ok(0)` (closed) or `Err`.
* `socket.write_all(&buf[..n]).await` — async write all bytes.
* `?` — propagate errors upward.
* Use `tokio::time::sleep(...).await` for non-blocking sleeps.

---

