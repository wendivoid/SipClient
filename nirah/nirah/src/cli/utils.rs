use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use serde_json::to_vec;
use serde_json::from_slice;

use crate::uds::UdsRpcProvider;
use nirah_core::core::NirahResult;
use nirah_core::rpc::RpcRequest;
use nirah_core::rpc::RpcResponse;

use std::fmt;

pub struct OptionalDisplay<T: fmt::Display>(pub Option<T>);

impl <T: fmt::Display> fmt::Display for OptionalDisplay<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(t) => write!(f, "{}", t),
            None => write!(f, " ")
        }
    }
}

pub async fn print_response(req: RpcRequest) -> NirahResult<()> {
    let mut stream = UnixStream::connect(UdsRpcProvider::default_file_path()?).await?;
    let mut data = to_vec(&req)?;
    data.push('\n' as u8);
    trace!("Writing Data: {:?}", data);
    stream.write_all(&data).await?;

    let mut buf = vec![];
    let mut reader = BufReader::new(stream);
    reader.read_until('\n' as u8, &mut buf).await?;
    trace!("Recieved Raw Data: {:?}", &buf);
    let res: RpcResponse = from_slice(&buf)?;
    println!("{:#?}", res);
    Ok(())
}

pub async fn get_response(req: RpcRequest) -> NirahResult<RpcResponse> {
    let mut stream = UnixStream::connect(UdsRpcProvider::default_file_path()?).await?;
    let mut data = to_vec(&req)?;
    data.push('\n' as u8);
    trace!("Writing Data: {:?}", data);
    stream.write_all(&data).await?;

    let mut buf = vec![];
    let mut reader = BufReader::new(stream);
    reader.read_until('\n' as u8, &mut buf).await?;
    trace!("Recieved Raw Data: {:?}", &buf);
    let res: RpcResponse = from_slice(&buf)?;
    Ok(res)
}
