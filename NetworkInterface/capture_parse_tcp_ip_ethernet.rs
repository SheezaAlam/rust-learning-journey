# 🔹 Big Picture (Why this program exists)

* In networking, everything travels as **packets** (Ethernet frames → IP packets → TCP segments).
* Tools like **Wireshark** or **tcpdump** do this job, but here you’re writing your own mini packet sniffer in Rust.
* Purpose:

  * Open a network interface (like `eth0` or `Wi-Fi`).
  * Capture raw frames.
  * Decode them into **Ethernet**, **IP**, and **TCP**.
  * Print useful details (source/destination IPs, ports).

---

# 🔹 Code Walkthrough

```rust
use clap::Parser;
```

👉 `clap` is for parsing command-line arguments (flags like `--list` or `--iface eth0`).
Without this, you’d hardcode the interface name.

---

```rust
use pnet::datalink::{self, Channel, Config, NetworkInterface};
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use std::time::Duration;
```

👉 `pnet` lets you capture raw packets.

* `datalink` = Layer 2 (Ethernet) access.
* `Channel` = a way to read/write packets.
* `EthernetPacket`, `Ipv4Packet`, `TcpPacket` = decoders for each layer.
* `Duration` = used to set read timeout (so program won’t block forever).

---

```rust
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    list: bool,
    #[arg(long)]
    iface: Option<String>,
    #[arg(long)]
    promisc: bool,
}
```

👉 Defines what command-line arguments are available:

* `--list` → show all interfaces.
* `--iface <name>` → pick one interface.
* `--promisc` → enable promiscuous mode (capture *all* packets, not just yours).

---

```rust
fn print_interfaces() {
    for iface in datalink::interfaces() {
        println!("{}  mac:{:?} ips:{:?}", iface.name, iface.mac, iface.ips);
    }
}
```

👉 Lists all network interfaces:

* Example: `eth0`, `wlan0`, `lo`.
* Shows MAC and IPs so you can choose.

---

```rust
fn main() {
    let cli = Cli::parse();
```

👉 Read user input (like `cargo run -- --list`).

---

```rust
    if cli.list {
        print_interfaces();
        return;
    }
```

👉 If user only wanted `--list`, show interfaces and exit.

---

```rust
    let iface_name = match cli.iface {
        Some(n) => n,
        None => {
            eprintln!("Please specify --iface <name> or use --list");
            return;
        }
    };
```

👉 Ensure the user picked an interface. If not → error.

---

```rust
    // find the interface
    let interfaces = datalink::interfaces();
    let iface = interfaces.into_iter()
        .find(|i| i.name == iface_name)
        .expect("Interface not found");
```

👉 Search for the chosen interface in all available ones.

---

```rust
    // configure capture
    let mut config = Config::default();
    config.read_timeout = Some(Duration::from_secs(1));
    config.promiscuous = cli.promisc;
```

👉 Configure how to capture:

* Timeout = 1 second (so `rx.next()` doesn’t block forever).
* Promiscuous mode if user enabled it.

---

```rust
    // open channel
    match datalink::channel(&iface, config) {
        Ok(Channel::Ethernet(_tx, mut rx)) => {
            println!("Listening on {}", iface.name);
```

👉 Open a channel to the NIC.

* `_tx` → would allow sending packets (unused here).
* `rx` → packet receiver.

---

```rust
            loop {
                match rx.next() {
                    Ok(frame) => {
```

👉 Infinite loop to keep capturing frames.

---

```rust
                        if let Some(eth) = EthernetPacket::new(frame) {
                            match eth.get_ethertype() {
                                EtherTypes::Ipv4 => {
```

👉 Interpret captured data as an **Ethernet frame**.
If it contains **IPv4**, then decode deeper.

---

```rust
                                    if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                                        let src = ipv4.get_source();
                                        let dst = ipv4.get_destination();
                                        println!("IPv4: {} -> {}", src, dst);
```

👉 Extract source and destination IPs from the IPv4 header.

---

```rust
                                        if ipv4.get_next_level_protocol() == pnet::packet::ip::IpNextHeaderProtocols::Tcp {
                                            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                                println!("  TCP: {} -> {}", tcp.get_source(), tcp.get_destination());
                                            }
                                        }
```

👉 If the IPv4 packet is carrying TCP:

* Decode as TCP.
* Print source and destination ports.
  (You just built a tiny **TCP connection logger** 🚀).

---

```rust
                                EtherTypes::Ipv6 => {
                                    println!("IPv6 packet (ignored in this demo)");
                                }
                                _ => { /* other ethertypes */ }
```

👉 Ignore IPv6 and other protocols for now.

---

```rust
                        } else {
                            eprintln!("Malformed ethernet packet");
                        }
```

👉 Handle corrupted frames.

---

```rust
                    }
                    Err(e) => {
                        eprintln!("Receive error: {:?}", e);
                    }
                }
            }
```

👉 Handle timeouts/errors but keep program running.

---

```rust
        }
        Ok(_) => {
            eprintln!("Unsupported channel type on this platform");
        }
        Err(e) => {
            eprintln!("Failed to open datalink channel: {}", e);
        }
    }
}
```

👉 Handle errors if channel cannot open.

---

# 🔹 Summary in Simple Words

* **Why**: Build your own Wireshark-like tool in Rust.
* **When**: Anytime you need raw packet sniffing (network debugging, security, protocol dev).
* **What**:

  1. Pick a network interface.
  2. Open a datalink channel.
  3. Capture raw Ethernet frames.
  4. Decode into IP, then TCP.
  5. Print details like source/destination IPs and ports.

---

👉 Example Run:

```sh
cargo run -- --list
# shows: eth0, wlan0, lo

cargo run -- --iface eth0 --promisc
# starts listening to packets
```

Output:

```
Listening on eth0
IPv4: 192.168.1.10 -> 8.8.8.8
  TCP: 54321 -> 80
IPv4: 8.8.8.8 -> 192.168.1.10
  TCP: 80 -> 54321
```
