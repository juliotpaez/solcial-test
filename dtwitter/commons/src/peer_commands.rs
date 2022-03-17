use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PeerCommand {
    /// Command to identify a node as a beacon.
    IAmBeacon,

    /// Command to get a node's messages.
    Get { node: String },

    /// Response of `Get`.
    GetResponse {
        destination: String,
        messages: Vec<String>,
    },

    /// Updates the messages in a beacon.
    UpdateMessages { messages: Vec<String> },

    /// Links a beacon.
    LinkBeacon {
        destination: String,
        messages: Vec<String>,
    },

    /// Unlinks a beacon.
    UnlinkBeacon,
}
