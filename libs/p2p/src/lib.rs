
mod std_async_tests;

use libp2p::{ping, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    ping: ping::Behaviour,
}


#[cfg(test)]
mod tests {
    // use super::*;
    use libp2p::identity;

    #[test]
    fn create_local_peer(){
        let local_key = identity::Keypair::generate_ed25519();
        println!("local_key: {local_key:?}")
    }
}
