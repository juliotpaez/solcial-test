use crate::{info, ClientBehaviour};
use commons::peer_commands::PeerCommand;
use commons::prelude::log::error;
use libp2p::PeerId;
use std::str::FromStr;

pub fn handle_peer_command(behaviour: &mut ClientBehaviour, command: PeerCommand, origin: PeerId) {
    match command {
        PeerCommand::Get { node } => {
            info!("Received PeerCommand::Get");

            let node = match PeerId::from_str(&node) {
                Ok(v) => v,
                Err(_) => {
                    error!("Expected peer_id");
                    return;
                }
            };

            let node = match behaviour.node_cache.get(&node) {
                Some(v) => v,
                None => {
                    // Ignore if missing.
                    return;
                }
            };

            // Process only if mirroring the node.
            let messages = match &node.messages {
                Some(v) => v,
                None => {
                    return;
                }
            };

            let response = PeerCommand::GetResponse {
                destination: origin.to_base58(),
                messages: messages.clone(),
            };
            let response = serde_json::to_vec(&response).unwrap();

            behaviour
                .floodsub
                .publish(behaviour.general_cache.floodsub_topic.clone(), response);
        }
        PeerCommand::UpdateMessages { messages } => {
            info!("Received PeerCommand::UpdateMessages");

            let node = match behaviour.node_cache.get_mut(&origin) {
                Some(v) => v,
                None => {
                    // Ignore if missing.
                    return;
                }
            };

            // Process only if mirroring the node.
            let old_messages = match &mut node.messages {
                Some(v) => v,
                None => {
                    return;
                }
            };

            *old_messages = messages;
        }
        PeerCommand::LinkBeacon {
            destination,
            messages,
        } => {
            info!("Received PeerCommand::LinkBeacon");

            let destination = match PeerId::from_str(&destination) {
                Ok(v) => v,
                Err(_) => {
                    error!("Expected peer_id");
                    return;
                }
            };

            let node = match behaviour.node_cache.get_mut(&origin) {
                Some(v) => v,
                None => {
                    // Ignore if missing.
                    return;
                }
            };

            // Link the beacon to the node if the message is for me or unlink it in case the node is
            // using another beacon.
            if destination == behaviour.general_cache.peer_id {
                node.messages = Some(messages);
            } else {
                node.messages = None;
            }
        }
        PeerCommand::UnlinkBeacon => {
            info!("Received PeerCommand::UnlinkBeacon");

            let node = match behaviour.node_cache.get_mut(&origin) {
                Some(v) => v,
                None => {
                    // Ignore if missing.
                    return;
                }
            };

            node.messages = None;
        }
        PeerCommand::IAmBeacon | PeerCommand::GetResponse { .. } => {
            // These commands are ignored because they are intended to be processed only in clients.
        }
    }
}
