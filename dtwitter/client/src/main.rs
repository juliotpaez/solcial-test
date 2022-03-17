use crate::behaviour::ClientBehaviour;
use crate::cache::GeneraCache;
use crate::cache::NodeCache;
use crate::user_commands::handle_user_command;
use commons::errors::AppResult;
use commons::init_logger;
use commons::p2p::{noise_keypair, random_peer_id};
use commons::prelude::log::{error, info};
use libp2p::core::transport::upgrade;
use libp2p::floodsub::Floodsub;
use libp2p::futures::StreamExt;
use libp2p::mdns::Mdns;
use libp2p::ping::{self, Ping};
use libp2p::swarm::{SwarmBuilder, SwarmEvent};
use libp2p::tcp::TokioTcpConfig;
use libp2p::{floodsub, mplex, noise, Transport};
use tokio::io::AsyncBufReadExt;

mod behaviour;
mod cache;
mod peer_commands;
mod user_commands;

#[tokio::main]
async fn main() {
    init_logger();

    if let Err(e) = application().await {
        error!("{}", e);
        std::process::exit(1)
    }
}

async fn application() -> AppResult<()> {
    let (keypair, peer_id) = random_peer_id();
    info!("Peer id: {}", peer_id.to_base58());

    let noise_keys = noise_keypair(&keypair);

    // Create a tokio-based TCP transport use noise for authenticated
    // encryption and Mplex for multiplexing of substreams on a TCP stream.
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    // Create a Floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat");

    // Create a Swarm to manage peers and events.
    let mut swarm = {
        let mdns = Mdns::new(Default::default()).await?;
        let mut behaviour = ClientBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns,
            ping: Ping::new(ping::Config::new().with_keep_alive(true)),
            node_cache: NodeCache::new(),
            general_cache: GeneraCache {
                peer_id,
                floodsub_topic: floodsub_topic.clone(),
                messages: Vec::new(),
            },
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());

        SwarmBuilder::new(transport, behaviour, peer_id)
            // We want the connection background tasks to be spawned
            // onto the tokio runtime.
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };

    // Read full lines from stdin
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Kick it off
    loop {
        tokio::select! {
            line = stdin.next_line() => handle_user_command(swarm.behaviour_mut(), line?.expect("stdin closed")),
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    info!("Listening on {:?}", address);
                }
            }
        }
    }
}
