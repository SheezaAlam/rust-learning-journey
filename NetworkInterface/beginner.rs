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



o/p:
     Running `target\debug\first_app.exe --list`
Name:\Device\NPF_{343A0F0C-3659-4C02-B81A-7A1E8142864B}
MAC: 74:4c:a1:76:21:02
IP:0.0.0.0/0

Name:\Device\NPF_{42088AD8-9316-4CE8-A66E-91EB31C3AEDC}
MAC: 74:4c:a1:76:21:01
IP:192.168.18.187/24

Name:\Device\NPF_{4F07CA78-6230-4791-9F30-AD5E4C61B005}
MAC: 00:50:56:c0:00:08
IP:192.168.52.1/24

Name:\Device\NPF_{A34FC79E-A36C-4497-9BA9-9A66C4767CA3}
MAC: 00:50:56:c0:00:01
IP:192.168.75.1/24

Name:\Device\NPF_{5D6791B3-7623-4C17-B8CE-DE97F471C581}
MAC: 76:4c:a1:76:11:31
IP:0.0.0.0/0

Name:\Device\NPF_{D14A4BB2-691E-420B-8338-ACD4FC4C7275}
MAC: 76:4c:a1:76:01:21
IP:0.0.0.0/0

Name:\Device\NPF_{B005AB59-F42B-45BC-9EF1-B2523F32974C}
MAC: 08:8f:c3:15:81:27
IP:0.0.0.0/0


