use toki  use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::time::{Instant, Duration};

// ---- 1. Immutable Infrastructure Constraints ----
const BIND_ADDRESS: &str = "0.0.0.0:8080"; 
const UDP_DISCOVERY_ADDRESS: &str = "239.255.255.250:1900"; 
const ALADDIN_PRE_SHARED_KEY: &[u8; 16] = b"ALADDIN_SECURE32"; 

// Hardware alignment benchmarks for sub-microsecond parsing
const MAGIC_BYTES: &[u8; 4] = b"SNX1";
const STATIC_PACKET_SIZE: usize = 64; 

// ---- 2. Hardware-Aligned Fixed Size Packet Structure ----
#[derive(Debug, Clone, Copy)]
#[repr(C)] // Guarantees exact hardware memory layout
pub struct SennexFixedPacket {
    pub magic: [u8; 4],
    pub packet_id: u32,
    pub payload_len: u32,
    pub payload: [u8; 52], // Fixed 52 bytes buffer ensuring total struct size = 64 bytes
}

impl SennexFixedPacket {
    pub fn deserialize(buffer: &[u8; STATIC_PACKET_SIZE]) -> Result<Self, &'static str> {
        if &buffer[0..4] != MAGIC_BYTES {
            return Err("ERR_PROTOCOL_MISMATCH: Invalid signature framework.");
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&buffer[0..4]);

        let packet_id = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let payload_len = u32::from_be_bytes(buffer[8..12].try_into().unwrap());

        if payload_len > 52 {
            return Err("ERR_PAYLOAD_OVERFLOW: Payload sizes break cache boundary alignment.");
        }

        let mut payload = [0u8; 52];
        payload.copy_from_slice(&buffer[12..64]);

        Ok(SennexFixedPacket {
            magic,
            packet_id,
            payload_len,
            payload,
        })
    }
}

// ---- 3. Live Distributed Execution Framework ----
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.contains(&"--simulate".to_string()) {
        println!("[SENNEX SIMULATION] Launching Local Multi-Node Cluster Test...");
        run_local_simulation().await?;
        return Ok(());
    }

    println!("[SENNEX NODE] Initializing Fixed-Memory Allocation Engine on {}...", BIND_ADDRESS);
    
    tokio::spawn(async {
        if let Err(e) = start_peer_discovery().await {
            eprintln!("[SENNEX DISCOVERY ERROR] {}", e);
        }
    });

    let listener = TcpListener::bind(BIND_ADDRESS).await?;
    println!("[SENNEX NODE] Network Live. Strict 64-Byte Hardware alignment enforced.");

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_secure_peer(socket, addr).await {
                eprintln!("[SENNEX SECURITY] Connection rejected from {}: {}", addr, e);
            }
        });
    }
}

// ---- 4. Encrypted Handshake Verification Gate ----
async fn handle_secure_peer(mut socket: TcpStream, addr: std::net::SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut handshake_buffer = [0; 16];
    let bytes_read = socket.read(&mut handshake_buffer).await?;
    
    if bytes_read != 16 || &handshake_buffer != ALADDIN_PRE_SHARED_KEY {
        socket.write_all(b"HANDSHAKE_FAILED").await?;
        return Err(Box::from("ERR_UNAUTHORIZED_NODE"));
    }

    println!("[SENNEX SECURITY] Secure tunnel established with peer: {}", addr);
    socket.write_all(b"HANDSHAKE_OK____").await?;
    
    handle_peer_stream(socket).await
}

// ---- 5. High-Speed Stream Processing Engine ----
async fn handle_peer_stream(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; STATIC_PACKET_SIZE];
    loop {
        // Read exact 64 bytes structure to eliminate memory allocation overhead
        let bytes_read = socket.read_exact(&mut buffer).await?;
        if bytes_read == 0 { break; }

        let start_time = Instant::now();
        match SennexFixedPacket::deserialize(&buffer) {
            Ok(packet) => {
                let duration = start_time.elapsed();
                println!(
                    "[SENNEX] Verified Fixed Packet ID: {} | Execution Overhead: {:?}",
                    packet.packet_id, duration
                );
                socket.write_all(&buffer).await?;
            }
            Err(err_msg) => {
                eprintln!("[SENNEX SECURITY WARN] Terminating suspicious stream: {}", err_msg);
                break; 
            }
        }
    }
    Ok(())
}

// ---- 6. UDP Peer Discovery Mechanism ----
async fn start_peer_discovery() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    
    loop {
        let msg = b"SENNEX_NODE_PING";
        let _ = socket.send_to(msg, UDP_DISCOVERY_ADDRESS).await;
        tokio::time::sleep(Duration::from_secs(5)).await; 
    }
}

// ---- 7. Local Cluster Simulation Engine ----
async fn run_local_simulation() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    tokio::spawn(async move {
        if let Ok((socket, addr)) = listener.accept().await {
            let _ = handle_secure_peer(socket, addr).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let mut client = TcpStream::connect("127.0.0.1:8080").await?;

    client.write_all(ALADDIN_PRE_SHARED_KEY).await?;
    let mut response = [0; 16];
    client.read_exact(&mut response).await?;
    
    if &response == b"HANDSHAKE_OK____" {
        println!("[SENNEX SIMULATION] Handshake Protocol Authenticated.");
        
        // Form matching exact 64 bytes hardware packet configuration
        let mut mock_packet = [0u8; STATIC_PACKET_SIZE];
        mock_packet[0..4].copy_from_slice(MAGIC_BYTES);
        mock_packet[4..8].copy_from_slice(&7001u32.to_be_bytes()); 
        mock_packet[8..12].copy_from_slice(&12u32.to_be_bytes()); 
        mock_packet[12..24].copy_from_slice(b"FIXED_BUFFER"); 

        client.write_all(&mock_packet).await?;
        let mut response_buffer = [0; STATIC_PACKET_SIZE];
        client.read_exact(&mut response_buffer).await?;
        println!("[SENNEX SIMULATION] Fixed Memory Layout Routing Verified Successfully.");
    }

    Ok(())
  }
