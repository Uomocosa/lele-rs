#![allow(dead_code)]

use std::error;
use std::time::Duration;
use futures::select;
use futures::StreamExt;
use libp2p::{
    self, identity::{self, Keypair}, noise, ping, tcp, yamux, Swarm, SwarmBuilder
};

type Result_<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(libp2p::swarm::NetworkBehaviour)]
struct Behaviour {
    ping: ping::Behaviour,
}

fn create_local_peer() -> Keypair {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();
    println!("local_peer_id: {local_peer_id:?}");
    local_key
}

fn create_swarm(peer_key: Keypair) -> Result_<Swarm<Behaviour>> {
    let swarm = SwarmBuilder::with_existing_identity(peer_key)
        .with_async_std()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| Behaviour { ping: ping::Behaviour::default() })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}

async fn handle_swarm_events(swarm: &mut Swarm<Behaviour>) {
    loop {
        select! {
            event = swarm.next() => match event {
                // Handle events here in the future
                _ => { println!("{event:?}") }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn create_local_peer_test() {
        let peer_key = create_local_peer();
        println!("peer_key: {peer_key:?}");
    }

    #[test]
    fn create_swarm_test() {
        let peer_key = create_local_peer();
        let _ = create_swarm(peer_key);
        println!("swarm created!");
    }

    #[ignore]
    #[test]
    fn handle_swarm_events_test() {
        let peer_key = create_local_peer();
        let mut swarm = create_swarm(peer_key).unwrap();
        println!("swarm created!");
        block_on(handle_swarm_events(&mut swarm));
    }
}
