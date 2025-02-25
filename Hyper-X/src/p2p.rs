use libp2p::{
  identity, mdns, noise, quic, swarm::SwarmEvent, PeerId, Swarm, Transport,
  gossipsub::{self, Gossipsub, GossipsubConfig, MessageAuthenticity, Topic},
};
use futures::StreamExt;
use std::error::Error;

pub struct P2PNode {
  pub peer_id: PeerId,
  pub swarm: Swarm<Gossipsub>,
}

impl P2PNode {
  pub async fn new() -> Result<Self, Box<dyn Error>> {
      let id_keys = identity::Keypair::generate_ed25519();
      let peer_id = PeerId::from(id_keys.public());

      let gossipsub_config = GossipsubConfig::default();
      let mut gossipsub = Gossipsub::new(
          MessageAuthenticity::Signed(id_keys.clone()),
          gossipsub_config,
      )?;
      let topic = Topic::new("hyperx-blockchain");
      gossipsub.subscribe(&topic)?;

      let transport = quic::async_std::Transport::new(noise::Config::new(&id_keys)?)
          .multiplex(libp2p::yamux::Config::default())
          .boxed();

      let mut swarm = libp2p::Swarm::new(transport, gossipsub, peer_id.clone());

      let mut mdns = mdns::Mdns::new(mdns::MdnsConfig::default()).await?;
      swarm.behaviour_mut().add_protocol(mdns);

      Ok(Self { peer_id, swarm })
  }

  pub async fn run(&mut self) {
      println!("🌐 Hyper-X Node Running: {}", self.peer_id);

      while let Some(event) = self.swarm.next().await {
          match event {
              SwarmEvent::Behaviour(gossipsub::GossipsubEvent::Message { message, .. }) => {
                  println!("📩 Received message: {}", String::from_utf8_lossy(&message.data));
              }
              _ => {}
          }
      }
  }

  pub async fn send_message(&mut self, message: &str) {
      let topic = Topic::new("hyperx-blockchain");
      self.swarm.behaviour_mut().publish(topic, message.as_bytes()).unwrap();
      println!("📤 Sent message: {}", message);
  }
}
