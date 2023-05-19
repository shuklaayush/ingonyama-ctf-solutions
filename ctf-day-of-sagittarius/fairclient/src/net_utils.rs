use sagittarius_game::types::Position;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result, BufReader, AsyncBufReadExt};

const MESSAGE_DELIMITER: u8 = 0x1E; // "Record Separator" in ASCII

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClusterMessage {
    pub ul: Position,
    pub dr: Position,
    pub seed: u8
}

impl ClusterMessage {
    pub fn check(&self) {
        self.ul.check();
        self.dr.check();
    }
}

pub async fn chunk_write(stream: &mut TcpStream, message: &[u8]) -> Result<()> {
    let len = message.len() as u32;
    stream.write_u32(len).await?;

    let chunk_size = 1024;
    let mut chunks = message.chunks(chunk_size);

    // Send each chunk
    while let Some(chunk) = chunks.next() {
        stream.write_all(chunk).await?;
    }
    
    Ok(())
}

pub async fn chunk_read(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let len: u32 = stream.read_u32().await?;
    let mut message = vec![0u8; len as usize];
    stream.read_exact(&mut message).await?;

    Ok(message)
}

pub async fn write_message(stream: &mut TcpStream, message: &[u8]) -> Result<()> {
    let mut bytes = vec![];
    bytes.extend_from_slice(message);
    bytes.push(MESSAGE_DELIMITER);
    stream.write_all(&bytes).await
}

pub async fn read_message(stream: &mut TcpStream) -> Result<String> {
    let mut bytes = vec![];
    let mut reader = BufReader::new(stream);
    reader.read_until(MESSAGE_DELIMITER, &mut bytes).await?;
    bytes.pop();
    Ok(String::from_utf8(bytes).expect("Non utf-8 message!"))
}

pub async fn get_coords_from_server(stream: &mut TcpStream) -> Result<Position> {
    let mut coords_bytes = vec![];
    stream.read_buf(&mut coords_bytes).await?;

    Ok(serde_cbor::from_slice(&coords_bytes).expect("Error while deser"))
}

pub async fn get_cluster_message(stream: &mut TcpStream) -> Result<ClusterMessage> {
    let mut cluster_bytes = vec![];
    stream.read_buf(&mut cluster_bytes).await?;

    Ok(serde_cbor::from_slice(&cluster_bytes).expect("Error while deser"))
}