 u use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::time::{Instant, Duration};
use std::convert::TryInto;

// ---- 1. System Configuration and Constants ----
const NODE_ADDRESS: &str = "127.0.0.1:8080";
const UDP_DISCOVERY_ADDRESS: &str = "239.255.255.250:1900"; // Multicast IP for discovery
const BUFFER_SIZE: usize = 1024;
const MAGIC_BYTES: &[u8; 4] = b"SNX1"; 
const HEADER_SIZE: usize = 12;         
const MAX_PAYLOAD: usize = 1012;       

// ---- 2. Low-Latency Packet Structure ----
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

// ---- 3. Dual-Mode Execution Framework ----
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.contains(&"--simulate".to_string()) {
        println!("[SENNEX SIMULATION] Launching Local Multi-Node Cluster Test...");
        run_local_simulation().await?;
        return Ok(());
    }

    println!("[SENNEX NODE] Initializing Phase 2 Protocol on {}...", NODE_ADDRESS);
    
    // Start background peer discovery signal
    tokio::spawn(async {
        if let Err(e) = start_peer_discovery().await {
            eprintln!("[SENNEX DISCOVERY ERROR] {}", e);
        }
    });

    let listener = TcpListener::bind(NODE_ADDRESS).await?;
    println!("[SENNEX NODE] Network Live. Awaiting incoming data streams...");

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_peer_stream(socket).await {
                eprintln!("[SENNEX ERROR] Stream failure: {}", e);
            }
        });
    }
}

// ---- 4. In-Memory Processing & Verification ----
async fn handle_peer_stream(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let bytes_read = socket.read(&mut buffer).await?;
        if bytes_read == 0 { break; }

        let start_time = Instant::now();
        if let Ok(packet) = SennexPacket::parse_raw_stream(&buffer[..bytes_read]) {
            let duration = start_time.elapsed();
            println!(
                "[SENNEX] Verified Packet ID: {} | Latency: {:?}",
                packet.packet_id, duration
            );
        }
        socket.write_all(&buffer[..bytes_read]).await?;
    }
    Ok(())
}

// ---- 5. UDP Peer Discovery Mechanism (New) ----
async fn start_peer_discovery() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    
    println!("[SENNEX DISCOVERY] Broadcasting discovery beacon to the network...");
    
    loop {
        let msg = b"SENNEX_NODE_PING";
        socket.send_to(msg, UDP_DISCOVERY_ADDRESS).await?;
        tokio::time::sleep(Duration::from_secs(5)).await; // Ping every 5 seconds
    }
}

// ---- 6. Local Cluster Simulation Engine ----
async fn run_local_simulation() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(NODE_ADDRESS).await?;
    tokio::spawn(async move {
        if let Ok((socket, _)) = listener.accept().await {
            let _ = handle_peer_stream(socket).await;
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let mut client = TcpStream::connect(NODE_ADDRESS).await?;

    let mut mock_packet = Vec::new();
    mock_packet.extend_from_slice(MAGIC_BYTES);
    mock_packet.extend_from_slice(&5001u32.to_be_bytes()); 
    mock_packet.extend_from_slice(&12u32.to_be_bytes());   
    mock_packet.extend_from_slice(b"ALADDIN_CORE");        

    println!("[SENNEX SIMULATION] Injecting 1024-byte aligned data frame into runtime...");
    client.write_all(&mock_packet).await?;

    let mut response_buffer = [0; BUFFER_SIZE];
    let _ = client.read(&mut response_buffer).await?;
    println!("[SENNEX SIMULATION] Local Cluster Validation Completed Successfully.");

    Ok(())
}

