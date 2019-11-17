use async_trait::async_trait;

use crate::prelude::*;

mod request;
pub use self::request::RpcRequest;

mod response;
pub use self::response::RpcResponse;

mod handler;
pub use self::handler::DefaultRpcHandler;

#[async_trait]
pub trait RpcProvider<T>: Provider {
    async fn get(&mut self) -> NirahResult<(RpcRequest, T)>;

    async fn send(&mut self, req: RpcResponse, peer: T) -> NirahResult<()>;

    async fn connect(&mut self, cfg: &mut ConfigFuture) -> NirahResult<()>;
}

#[async_trait]
pub trait RpcHandlerProvider: Provider {
    async fn handle<'a>(&mut self, req: RpcRequest, _ctx: ServerCtx<'a>) -> NirahResult<RpcResponse>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
