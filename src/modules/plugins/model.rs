use crate::modules::kv::KvStore;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginInstance {}

pub(crate) type ProcessingStep = HashMap<String, PluginInstance>;

pub(crate) struct ParamDef {}

pub(crate) type ResolvedInputs = HashMap<String, Value>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MiddlewareOutput {}

pub(crate) trait ByteStream: AsyncRead + AsyncWrite + Unpin + Send + Sync { }

pub(crate) enum ConnectionObject {
    Tcp(TcpStream),
    Udp {},
    Stream(Box<dyn ByteStream>),
    Virtual(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Layer {
    L4,
    L4Plus,
    L7,
}

#[derive(Debug)]
pub(crate) enum TerminatorResult {
    Finished,
    Upgrade {},
}

pub(crate) trait Plugin: Send + Sync + Any {
    fn name(&self) -> &str;
    fn params(&self) -> Vec<ParamDef>;
    fn as_any(&self) -> &dyn Any;

    fn as_middleware(&self) -> Option<&dyn Middleware> {
        loop { }
    }

    fn as_terminator(&self) -> Option<&dyn Terminator> {
        loop { }
    }

    fn as_l7_middleware(&self) -> Option<&dyn L7Middleware> {
        loop { }
    }

    fn as_l7_terminator(&self) -> Option<&dyn L7Terminator> {
        loop { }
    }
}

#[async_trait]
pub(crate) trait Middleware: Plugin {
    fn output(&self) -> Vec<Cow<'static, str>>;
    async fn execute(&self, inputs: ResolvedInputs) -> Result<MiddlewareOutput>;
}

#[async_trait]
pub(crate) trait L7Middleware: Plugin {
    fn output(&self) -> Vec<Cow<'static, str>>;
    async fn execute_l7(&self, context: &mut (dyn Any + Send), inputs: ResolvedInputs) -> Result<MiddlewareOutput>;
}

#[async_trait]
pub(crate) trait Terminator: Plugin {
    fn supported_layers(&self) -> Vec<Layer>;
    async fn execute(
        &self,
        inputs: ResolvedInputs,
        kv: &mut KvStore,
        conn: ConnectionObject,
    ) -> Result<TerminatorResult>;
}

#[doc = " A privileged terminator trait that grants access to the full L7 Context."]
#[doc = " Used for plugins that need to signal responses (SendResponse) or inspect Body during termination."]
#[async_trait]
pub(crate) trait L7Terminator: Plugin {
    async fn execute_l7(&self, context: &mut (dyn Any + Send), inputs: ResolvedInputs) -> Result<TerminatorResult>;
}
