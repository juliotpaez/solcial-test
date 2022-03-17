use crate::floodsub::Topic;
use libp2p::PeerId;
use std::collections::HashMap;

pub type NodeCache = HashMap<PeerId, NodeData>;

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct GeneraCache {
    /// The peer id of the current node.
    pub peer_id: PeerId,

    /// The floodsub topic used to communicate with other peers.
    pub floodsub_topic: Topic,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct NodeData {
    /// The list of messages if mirroring (Some).
    pub messages: Option<Vec<String>>,
}
