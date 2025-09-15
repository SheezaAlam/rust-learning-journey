Beginner — Goal: list interfaces & let user pick one

Objectives:

learn pnet::datalink::interfaces()

show name, MAC, IPs

simple CLI --list and --iface <name>

Beginner code: list + pick (complete)

Create a new project:

cargo new pnet_ifaces --bin
cd pnet_ifaces
# edit Cargo.toml to add:
# pnet = "0.34.0"
# clap = { version = "4", features = ["derive"] }
cargo build


Replace src/main.rs with:

use clap::Parser;
use pnet::datalink::{self, NetworkInterface};

/// Simple tool: list interfaces or pick one by name
#[derive(Parser)]
struct Cli {
    /// List available interfaces
    #[arg(long)]
    list: bool,

    /// Interface name to show details for (e.g., eth0, Wi-Fi, lo)
    #[arg(long)]
    iface: Option<String>,
}

fn print_interfaces() {
    // datalink::interfaces() returns Vec<NetworkInterface>
    let ifaces = datalink::interfaces();
    for iface in ifaces {
        // print basic info: name, mac(optional), and ips
        println!("Name: {}", iface.name);
        match iface.mac {
            Some(mac) => println!("  MAC: {}", mac),
            None => println!("  MAC: (none)"),
        }
        for ip in iface.ips {
            println!("  IP: {}", ip);
        }
        println!();
    }
}

fn find_interface_by_name(name: &str) -> Option<NetworkInterface> {
    // Search exact match by name
    datalink::interfaces().into_iter().find(|i| i.name == name)
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        print_interfaces();
        return;
    }

    if let Some(name) = cli.iface {
        match find_interface_by_name(&name) {
            Some(iface) => {
                println!("Selected interface: {}", iface.name);
                println!("MAC: {:?}", iface.mac);
                println!("IPs: {:?}", iface.ips);
            }
            None => {
                eprintln!("Interface '{}' not found. Use --list to see names.", name);
                std::process::exit(1);
            }
        }
    } else {
        println!("No arguments given. Use --list to see interfaces or --iface <name>.");
    }
}

Line-by-line (key lines)

datalink::interfaces() — returns Vec<NetworkInterface> containing .name, .mac: Option<MacAddr>, .ips: Vec<IpNetwork>.

iface.mac — optional MAC address; not all interfaces expose MAC (loopback usually doesn’t).

iface.ips — IP addresses assigned (can contain IPv4 and IPv6 entries).

find(|i| i.name == name) — exact name lookup; you can adapt to match prefix or case-insensitive.

How to run

List: cargo run -- --list

Show specific: cargo run -- --iface eth0 (use name from --list output)
