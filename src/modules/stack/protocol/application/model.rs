use crate::modules::plugins::model::ProcessingStep;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

#[doc = " Represents the configuration for a specific L7 application protocol (e.g., \"httpx\")."]
#[doc = ""]
#[doc = " L7 protocols handle the request/response lifecycle after TLS/QUIC termination."]
#[doc = " The `pipeline` defines the middleware chain (Request -> Upstream -> Response)."]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) struct ApplicationConfig {
    pub(crate) pipeline: ProcessingStep,
}

#[doc = " A global, thread-safe registry of active application configurations."]
#[doc = " Key: Protocol Name (e.g., \"httpx\")"]
#[doc = " Value: The parsed configuration"]
pub(crate) static APPLICATION_REGISTRY: Lazy<ArcSwap<DashMap<String, Arc<ApplicationConfig>>>> =
    Lazy::new(|| ArcSwap::new(Arc::new(DashMap::new())));
