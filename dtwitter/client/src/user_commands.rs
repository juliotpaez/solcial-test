use crate::cache::NodeData;
use crate::ClientBehaviour;
use commons::peer_commands::PeerCommand;
use commons::prelude::log::error;
use libp2p::PeerId;
use std::str::FromStr;

pub fn handle_user_command(behaviour: &mut ClientBehaviour, line: String) {
    let mut args = line.splitn(2, ' ');

    match args.next() {
        Some("GET") => {
            let peer_id = {
                match args.next() {
                    Some(v) => match PeerId::from_str(v) {
                        Ok(v) => v,
                        Err(_) => {
                            error!("Expected peer_id");
                            return;
                        }
                    },
                    None => {
                        // If empty it is me.
                        behaviour.general_cache.peer_id
                    }
                }
            };

            // Print my messages in case the user requests the current node.
            if peer_id == behaviour.general_cache.peer_id {
                let messages = &behaviour.general_cache.messages;

                if messages.is_empty() {
                    println!("Empty");
                } else {
                    for (i, message) in messages.iter().enumerate() {
                        println!("[{}] {}", i, message);
                    }
                }

                return;
            }

            let response = PeerCommand::Get {
                node: peer_id.to_base58(),
            };
            let response = serde_json::to_vec(&response).unwrap();

            behaviour
                .floodsub
                .publish(behaviour.general_cache.floodsub_topic.clone(), response);
        }
        Some("POST") => {
            let text = {
                match args.next() {
                    Some(v) => v,
                    None => {
                        error!("Expected text");
                        return;
                    }
                }
            };

            behaviour.general_cache.messages.push(text.to_string());

            // Mirror the messages to the associated beacon if there's any.
            if behaviour
                .node_cache
                .iter()
                .any(|(_, v)| v.is_mirroring_my_data)
            {
                let response = PeerCommand::UpdateMessages {
                    messages: behaviour.general_cache.messages.clone(),
                };
                let response = serde_json::to_vec(&response).unwrap();

                behaviour
                    .floodsub
                    .publish(behaviour.general_cache.floodsub_topic.clone(), response);
            }
        }
        Some("LINK_BEACON") => {
            let peer_id = {
                match args.next() {
                    Some(v) => match PeerId::from_str(v) {
                        Ok(v) => v,
                        Err(_) => {
                            error!("Expected peer_id");
                            return;
                        }
                    },
                    None => {
                        // If empty it is me.
                        behaviour.general_cache.peer_id
                    }
                }
            };

            let mut beacon: Option<&mut NodeData> = None;

            for (id, value) in &mut behaviour.node_cache {
                if id == &peer_id {
                    // Ignore if it is not a beacon.
                    if !value.is_beacon {
                        error!("Not a beacon");
                        return;
                    }

                    beacon = Some(value);
                    break;
                }
            }

            let beacon = match beacon {
                Some(v) => v,
                None => {
                    error!("Undefined peer");
                    return;
                }
            };

            // Ignore if already linking to the beacon.
            if beacon.is_mirroring_my_data {
                return;
            }

            // Update data.
            beacon.is_mirroring_my_data = true;

            let response = PeerCommand::LinkBeacon {
                destination: peer_id.to_base58(),
                messages: behaviour.general_cache.messages.clone(),
            };
            let response = serde_json::to_vec(&response).unwrap();

            behaviour
                .floodsub
                .publish(behaviour.general_cache.floodsub_topic.clone(), response);
        }
        Some("UNLINK_BEACON") => {
            // Ignore if not linking with any beacon
            if !behaviour
                .node_cache
                .iter()
                .any(|(_, v)| v.is_mirroring_my_data)
            {
                return;
            }

            let response = PeerCommand::UnlinkBeacon;
            let response = serde_json::to_vec(&response).unwrap();

            behaviour
                .floodsub
                .publish(behaviour.general_cache.floodsub_topic.clone(), response);
        }
        _ => {
            error!(
                "expected command:\
            \n  - GET\
            \n  - GET <peer_id>\
            \n  - POST <text>\
            \n  - LINK_BEACON <peer_id>\
            \n  - UNLINK_BEACON"
            );
        }
    }
}
