use libp2p::identity::Keypair;
use libp2p::noise::{AuthenticKeypair, X25519Spec};
use libp2p::{identity, noise, PeerId};

/// Creates a random peer id to identify a node in the network.
pub fn random_peer_id() -> (Keypair, PeerId) {
    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    (keypair, peer_id)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Creates a new keypair to secure the transport using an encryption protocol
/// based on the Noise Protocol Framework.
pub fn noise_keypair(keypair: &Keypair) -> AuthenticKeypair<X25519Spec> {
    noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(keypair)
        .expect("Signing libp2p-noise static DH keypair failed.")
}
