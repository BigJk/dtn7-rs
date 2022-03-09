pub mod epidemic;
pub mod erouting;
pub mod external;
pub mod flooding;
pub mod sink;

use crate::cla::ClaSenderTask;
use crate::core::bundlepack::BundlePack;
use async_trait::async_trait;
use bp7::Bundle;
use bp7::EndpointID;
use derive_more::*;
use enum_dispatch::enum_dispatch;
use epidemic::EpidemicRoutingAgent;
use external::ExternalRoutingAgent;
use flooding::FloodingRoutingAgent;
use sink::SinkRoutingAgent;
use std::fmt::Debug;
use std::fmt::Display;

pub enum RoutingNotifcation<'a> {
    SendingFailed(&'a str, &'a str),
    IncomingBundle(&'a Bundle),
    IncomingBundleWithoutPreviousNode(&'a str, &'a str),
    EncounteredPeer(&'a EndpointID),
    DroppedPeer(&'a EndpointID),
}

#[enum_dispatch]
#[derive(Debug, Display)]
pub enum RoutingAgentsEnum {
    EpidemicRoutingAgent,
    FloodingRoutingAgent,
    SinkRoutingAgent,
    ExternalRoutingAgent,
}

/*
impl std::fmt::Display for RoutingAgentsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
*/

#[async_trait]
#[enum_dispatch(RoutingAgentsEnum)]
pub trait RoutingAgent: Debug + Display {
    fn notify(&mut self, _notification: RoutingNotifcation) {}
    async fn sender_for_bundle(&mut self, _bp: &BundlePack) -> (Vec<ClaSenderTask>, bool) {
        unimplemented!();
    }
}

pub fn routing_algorithms() -> Vec<&'static str> {
    vec!["epidemic", "flooding", "sink", "external"]
}

pub fn new(routingagent: &str) -> RoutingAgentsEnum {
    match routingagent {
        "flooding" => FloodingRoutingAgent::new().into(),
        "epidemic" => EpidemicRoutingAgent::new().into(),
        "sink" => SinkRoutingAgent::new().into(),
        "external" => ExternalRoutingAgent::new().into(),
        _ => panic!("Unknown routing agent {}", routingagent),
    }
}
