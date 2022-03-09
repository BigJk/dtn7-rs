use super::RoutingAgent;
use crate::core::bundlepack::BundlePack;
use crate::{ClaSenderTask, PEERS};
use async_trait::async_trait;

/// Simple flooding-basic routing.
/// All bundles are sent to all known peers again and again.
#[derive(Default, Debug)]
pub struct FloodingRoutingAgent {}

impl FloodingRoutingAgent {
    pub fn new() -> FloodingRoutingAgent {
        FloodingRoutingAgent {}
    }
}
impl std::fmt::Display for FloodingRoutingAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FloodingRoutingAgent")
    }
}

#[async_trait]
impl RoutingAgent for FloodingRoutingAgent {
    async fn sender_for_bundle(&mut self, _bp: &BundlePack) -> (Vec<ClaSenderTask>, bool) {
        let mut clas = Vec::new();
        for (_, p) in (*PEERS.lock()).iter() {
            if let Some(cla) = p.first_cla() {
                clas.push(cla);
            }
        }
        (clas, false)
    }
}
