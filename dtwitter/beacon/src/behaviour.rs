use crate::cache::{GeneraCache, NodeCache, NodeData};
use crate::error;
use crate::peer_commands::handle_peer_command;
use commons::peer_commands::PeerCommand;
use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::ping::{Ping, PingEvent};
use libp2p::swarm::NetworkBehaviourEventProcess;
use libp2p::NetworkBehaviour;

/// The behaviour of the node.
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct ClientBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    pub ping: Ping,

    #[behaviour(ignore)]
    pub node_cache: NodeCache,

    #[behaviour(ignore)]
    pub general_cache: GeneraCache,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for ClientBehaviour {
    // Called when `floodsubc` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        match message {
            FloodsubEvent::Message(message) => {
                let command: PeerCommand = match serde_json::from_slice(&message.data) {
                    Ok(v) => v,
                    Err(_) => {
                        error!("PeerCommand expected");
                        return;
                    }
                };

                handle_peer_command(self, command, message.source);
            }
            FloodsubEvent::Subscribed { .. } => {
                // Tell others I am a beacon
                let response = PeerCommand::IAmBeacon;
                let response = serde_json::to_vec(&response).unwrap();
                self.floodsub
                    .publish(self.general_cache.floodsub_topic.clone(), response)
            }
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for ClientBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                    self.node_cache.insert(peer, NodeData { messages: None });
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for ClientBehaviour {
    fn inject_event(&mut self, _event: PingEvent) {}
}
