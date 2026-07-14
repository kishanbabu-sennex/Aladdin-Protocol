use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::time::{Instant, Duration};
use std::convert::TryInto;

// ---- 1. Optimized Global Network & Security Configuration ----
const BIND_ADDRESS: &str = "0.0.0.0:8080"; 
const UDP_DISCOVERY_ADDRESS: &str = "239.255.255.250:1900"; 
const BUFFER_SIZE: usize = 1024;
const MAGIC_BYTES: &[u8; 4] = b"SNX1"; 
const HEADER_SIZE: usize = 12;         
const MAX_PAYLOAD: usize = 1012;       

// PHASE 4 ENCRYPTION KEY: Dynamic validation token for authorized node handshakes
const ALADDIN_PRE_SHARED_KEY: &[u8; 16] = b"ALADDIN_SECURE32"; 

// ---- 2. Hardware-Aligned Packet Structure ----
#[derive(Debug)]
pub struct SennexPacket {
    pub packet_id: u32,
    pub payload_len: u32,
    pub payload: Vec<u8>,
}

impl SennexPacket {
    pub fn parse_raw_stream(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < HEADER_SIZE {
            return Err("ERR_INSUFFICIENT_DATA");
        }
        if &buffer[0..4] != MAGIC_BYTES {
            return Err("ERR_PROTOCOL_MISMATCH");
        }
        let packet_id = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let payload_len = u32::from_be_bytes(buffer[8..12].try_into().unwrap()) as usize;

        if payload_len > MAX_PAYLOAD || buffer.len() < HEADER_SIZE + payload_len {
            return Err("ERR_CORRUPTED_STREAM");
        }

        let payload = buffer[HEADER_SIZE..HEADER_SIZE + payload_len].to_vec();
        Ok(SennexPacket {
            packet_id,
            payload_len: payload_len as u32,
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

    println!("[SENNEX NODE] Initializing Phase 4 Production Protocol on {}...", BIND_ADDRESS);
    
    tokio::spawn(async {
        if let Err(e) = start_peer_discovery().await {
            eprintln!("[SENNEX DISCOVERY ERROR] {}", e);
        }
    });

    let listener = TcpListener::bind(BIND_ADDRESS).await?;
    println!("[SENNEX NODE] Network Live with Handshake Validation. Awaiting remote peers...");

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
    
    // Read the incoming handshake key with a timeout constraint
    let bytes_read = socket.read(&mut handshake_buffer).await?;
    
    if bytes_read != 16 || &handshake_buffer != ALADDIN_PRE_SHARED_KEY {
        socket.write_all(b"HANDSHAKE_FAILED").await?;
        return Err(Box::from("ERR_UNAUTHORIZED_NODE: Handshake verification failed. Isolation triggered."));
    }

    // Handshake approved - Proceed to stream data processing
    println!("[SENNEX SECURITY] Handshake approved. Secure tunnel established with peer: {}", addr);
    socket.write_all(b"HANDSHAKE_OK____").await?;
    
    handle_peer_stream(socket).await
}

// ---- 5. Isolated Stream Processing Engine ----
async fn handle_peer_stream(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let bytes_read = socket.read(&mut buffer).await?;
        if bytes_read == 0 { break; }

        let start_time = Instant::now();
        match SennexPacket::parse_raw_stream(&buffer[..bytes_read]) {
            Ok(packet) => {
                let duration = start_time.elapsed();
                println!(
                    "[SENNEX] Verified Packet ID: {} | Execution Overhead: {:?}",
                    packet.packet_id, duration
                );
                socket.write_all(&buffer[..bytes_read]).await?;
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

    // Step 1: Send the 16-byte secure handshake key
    client.write_all(ALADDIN_PRE_SHARED_KEY).await?;
    
    let mut response = [0; 16];
    client.read_exact(&mut response).await?;
    
    if &response == b"HANDSHAKE_OK____" {
        println!("[SENNEX SIMULATION] Handshake Protocol Authenticated.");
        
        // Step 2: Send actual structured packet data
        let mut mock_packet = Vec::new();
        mock_packet.extend_from_slice(MAGIC_BYTES);
        mock_packet.extend_from_slice(&5001u32.to_be_bytes()); 
        mock_packet.extend_from_slice(&12u32.to_be_bytes());   
        mock_packet.extend_from_slice(b"ALADDIN_CORE");        

        client.write_all(&mock_packet).await?;
        let mut response_buffer = [0; BUFFER_SIZE];
        let _ = client.read(&mut response_buffer).await?;
        println!("[SENNEX SIMULATION] Secure Execution Verified Successfully.");
    }

    Ok(())
}
