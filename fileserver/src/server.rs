use futures::StreamExt;
use libp2p::{
    identity::{self}, 
    PeerId, 
    ping,
    Swarm, 
    swarm::{
        keep_alive,
        SwarmEvent
    }
};
use libp2p_swarm_derive::NetworkBehaviour;
use log::info;

#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour
}

pub struct Node {
    local_peer_id: PeerId,
    swarm: Swarm<Behaviour>
}

impl Node {
    pub async fn init() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.clone().public()); 
        let transport = libp2p::development_transport(local_key).await.unwrap();
        let mut swarm = Swarm::with_async_std_executor(
            transport, Behaviour::default(), local_peer_id);
        
        swarm.listen_on(
            "/ip4/127.0.0.1/tcp/0".parse().unwrap()
        ).unwrap();

        Node { 
            local_peer_id, 
            swarm
        }
    }

    pub async fn run(&mut self){
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } 
                    => {
                        info!("Peer Id: {:?}", self.local_peer_id.to_string());
                        info!("Listening on address: {:?}", address.to_string())
                    }
                _ => {}
            }
        }
    }
}
