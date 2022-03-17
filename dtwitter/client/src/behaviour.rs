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
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            let command: PeerCommand = match serde_json::from_slice(&message.data) {
                Ok(v) => v,
                Err(_) => {
                    error!("PeerCommand expected");
                    return;
                }
            };

            handle_peer_command(self, command, message.source);
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
                    self.node_cache.insert(
                        peer,
                        NodeData {
                            is_beacon: false,
                            is_mirroring_my_data: false,
                        },
                    );
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                        self.node_cache.remove(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for ClientBehaviour {
    fn inject_event(&mut self, _event: PingEvent) {}
}
