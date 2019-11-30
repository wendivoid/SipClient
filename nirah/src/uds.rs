use async_trait::async_trait;
use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use serde_json::from_slice;
use serde_json::to_vec;

use nirah_core::prelude::*;

use std::io;
use std::path::PathBuf;

pub struct UdsRpcProvider(Option<UnixListener>);

impl UdsRpcProvider {

    pub fn new() -> UdsRpcProvider {
        UdsRpcProvider(None)
    }

    pub fn default_file_path() -> NirahResult<PathBuf> {
        use std::env::var;
        let mut nirah_dir = PathBuf::from("/tmp/nirah");
        let username = var("USER")?;
        nirah_dir.push(&format!("nirah-{}.socket", username));
        Ok(nirah_dir)
    }
}

impl Provider for UdsRpcProvider {
    fn nirah_provider_identifier(&self) -> &'static str {
        "UdsRpcProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<ConfigDefinition>> {
        Ok(vec![
          (
            VariableKey::new("socket_file"),
            Some(VariableValue::FilePath(UdsRpcProvider::default_file_path()?)),
            Some("Unix domain socket file used for rpc".into())
          )
        ])
    }
}

#[async_trait]
impl RpcProvider<UnixStream> for UdsRpcProvider {

    async fn connect(&mut self, cfg: &mut ConfigFuture) -> NirahResult<()> {
        use std::fs::create_dir;
        use std::fs::remove_file;

        if !&PathBuf::from("/tmp/nirah").exists() {
            create_dir("/tmp/nirah")?;
        }


        if let Some(VariableValue::FilePath(file_path)) = cfg.get_config_value(&VariableKey::new("socket_file")).await? {
            if file_path.exists() {
                remove_file(&file_path)?;
            }
            self.0 = Some(UnixListener::bind(file_path)?);
        } else {
            let file_path = UdsRpcProvider::default_file_path()?;
            if file_path.exists() {
                remove_file(&file_path)?;
            }
            self.0 = Some(UnixListener::bind(UdsRpcProvider::default_file_path()?)?);
        }
        Ok(())
    }

    async fn get(&mut self) -> NirahResult<(RpcRequest, UnixStream)> {
        if let Some(receiver) = &mut self.0 {
            let mut data = vec![];
            let (mut stream, _addr) = receiver.accept().await?;
            //trace!("Recieved connection from: {:?}", _addr);
            let mut reader = BufReader::new(&mut stream);
            let amt = reader.read_until('\n' as u8, &mut data).await?;
            //trace!("Read Data: {:?}", &data[..amt]);
            let out = from_slice(&data[..amt])?;
            //debug!("Got Request: {:?}", out);
            Ok((out, stream))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "listing socket has not been connected"))?
        }
    }

    async fn send(&mut self, res: RpcResponse, mut stream: UnixStream) -> NirahResult<()> {
        //debug!("Sending response: {:?}", res);
        let mut data = to_vec(&res)?;
        data.push('\n' as u8);
        //trace!("Sending Data: {:?}", data);
        stream.write_all(data.as_ref()).await?;
        Ok(())
    }
}
