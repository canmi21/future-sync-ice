use crate::common::requirements::Error;
use bytes::Bytes;
use http_body_util::Full;
use http_body_util::combinators::BoxBody;
use hyper::body::Incoming;
use hyper::upgrade::OnUpgrade;
use std::pin::Pin;

#[doc = " A unified Body enum that bridges Hyper (H1/H2), H3 (Quinn), and Buffered data."]
pub(crate) enum VaneBody {
    #[doc = " Native Hyper Body (HTTP/1.1, HTTP/2)"]
    Hyper(Incoming),
    #[doc = " H3 Stream Wrapper"]
    H3(BoxBody<Bytes, Error>),
    #[doc = " Generic Stream Wrapper (Boxed, for plugins like CGI/FastCGI)"]
    Generic(BoxBody<Bytes, Error>),
    #[doc = " Buffered Memory (Lazy Buffer or Generated Content)"]
    Buffered(Full<Bytes>),
    #[doc = " Special State: Switching Protocols (WebSocket / Upgrade)"]
    #[doc = " Holds the upstream upgrade handle."]
    SwitchingProtocols(OnUpgrade),
    #[doc = " A bridge that executes a callback when polled."]
    #[doc = " Used to spawn the tunnel task at the exact right moment."]
    #[doc = " Added + Sync to the trait object to satisfy BodyExt::boxed() bounds."]
    UpgradeBridge {
        tunnel_task: Option<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
    },
    #[doc = " Empty Body"]
    Empty,
}
