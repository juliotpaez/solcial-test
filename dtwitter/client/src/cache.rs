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

    /// The list of messages of the node.
    pub messages: Vec<String>,
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct NodeData {
    /// Whether the node is a beacon or not.
    pub is_beacon: bool,

    /// Whether the beacon is mirroring my data or not.
    pub is_mirroring_my_data: bool,
}
