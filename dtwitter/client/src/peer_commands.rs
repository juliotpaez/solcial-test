use crate::cache::NodeData;
use crate::{error, ClientBehaviour};
use commons::peer_commands::PeerCommand;
use commons::prelude::log::info;
use libp2p::PeerId;
use std::str::FromStr;

pub fn handle_peer_command(behaviour: &mut ClientBehaviour, command: PeerCommand, origin: PeerId) {
    match command {
        PeerCommand::IAmBeacon => {
            info!("Received PeerCommand::IAmBeacon");

            // Update the flag.
            for (id, data) in &mut behaviour.node_cache {
                if id == &origin {
                    data.is_beacon = true;
                    return;
                }
            }

            // Add it to the cache.
            behaviour.node_cache.insert(
                origin,
                NodeData {
                    is_beacon: true,
                    is_mirroring_my_data: false,
                },
            );
        }
        PeerCommand::Get { node } => {
            info!("Received PeerCommand::Get");

            let node = match PeerId::from_str(&node) {
                Ok(v) => v,
                Err(_) => {
                    error!("Expected peer_id");
                    return;
                }
            };

            // Process only if the message is for me.
            if node != behaviour.general_cache.peer_id {
                return;
            }

            // Do not answer if the node has an associated beacon because the beacon will answer for me.
            if behaviour
                .node_cache
                .iter()
                .any(|(_, v)| v.is_mirroring_my_data)
            {
                return;
            }

            let response = PeerCommand::GetResponse {
                destination: origin.to_base58(),
                messages: behaviour.general_cache.messages.clone(),
            };
            let response = serde_json::to_vec(&response).unwrap();

            behaviour
                .floodsub
                .publish(behaviour.general_cache.floodsub_topic.clone(), response);
        }
        PeerCommand::GetResponse {
            destination,
            messages,
        } => {
            info!("Received PeerCommand::GetResponse");

            let destination = match PeerId::from_str(&destination) {
                Ok(v) => v,
                Err(_) => {
                    error!("Expected peer_id");
                    return;
                }
            };

            // Process only if the message is for me.
            if destination == behaviour.general_cache.peer_id {
                if messages.is_empty() {
                    println!("Empty");
                } else {
                    for (i, message) in messages.iter().enumerate() {
                        println!("[{}] {}", i, message);
                    }
                }
            }
        }
        PeerCommand::UpdateMessages { .. }
        | PeerCommand::LinkBeacon { .. }
        | PeerCommand::UnlinkBeacon { .. } => {
            // These commands are ignored because they are intended to be processed only in beacons.
        }
    }
}
