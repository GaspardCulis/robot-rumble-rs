use std::{net::Ipv4Addr, time::Duration};

use lightyear::prelude::LinkConditionerConfig;
use serde::{Deserialize, Serialize};

/// Yoinked from [lightyear's example](https://github.com/cBournhonesque/lightyear/blob/main/examples/common/src/settings.rs)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientNetworkConfig {
    /// The client port to listen on
    pub client_port: u16,

    /// The ip address of the server
    pub server_addr: Ipv4Addr,

    /// The port of the server
    pub server_port: u16,

    /// Possibly add a conditioner to simulate network conditions
    pub conditioner: Option<Conditioner>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conditioner {
    /// One way latency in milliseconds
    pub latency_ms: u16,
    /// One way jitter in milliseconds
    pub jitter_ms: u16,
    /// Percentage of packet loss
    pub packet_loss: f32,
}

impl Into<LinkConditionerConfig> for Conditioner {
    fn into(self) -> LinkConditionerConfig {
        LinkConditionerConfig {
            incoming_latency: Duration::from_millis(self.latency_ms as u64),
            incoming_jitter: Duration::from_millis(self.jitter_ms as u64),
            incoming_loss: self.packet_loss,
        }
    }
}
