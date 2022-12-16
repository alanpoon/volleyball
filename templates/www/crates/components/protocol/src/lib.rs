pub mod command;
pub mod event;
pub use command::*;
pub use event::*;
use nats_lite::nats;
use std::io::Write;
use std::io::{BufReader, BufWriter};
const BUF_CAPACITY: usize = 128 * 1024;
use log::info;
use std::io;
pub fn handle_server_op(msg: Vec<u8>) -> io::Result<nats::proto::ServerOp> {
    let mut reader = BufReader::with_capacity(BUF_CAPACITY, &*msg);
    let server_op: Option<nats::proto::ServerOp> = nats::proto::decode(&mut reader)?;
    server_op.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
}
pub fn handle_client_op(client_op: nats::proto::ClientOp) -> io::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = vec![];
    let mut writer = BufWriter::with_capacity(BUF_CAPACITY, &mut *bytes);
    let mut client_op2= client_op.clone();
    match client_op2{
        nats::proto::ClientOp::Sub{ref mut subject,..}=>{
            *subject=format!("{}.{}",multitenary::UNIQUE,subject);
        }
        nats::proto::ClientOp::Pub{ref mut subject,..}=>{
            *subject=format!("{}.{}",multitenary::UNIQUE,subject);
        }
        _=>{}
    }
    
    nats::proto::encode(&mut writer, client_op2.clone())?;
    if let Ok(_) = writer.flush() {}
    Ok(writer.buffer().to_vec())
}
