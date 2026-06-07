use iroh::{endpoint::presets, protocol::Router, Endpoint, EndpointAddr, EndpointId, SecretKey};
use iroh_gossip::{
    api::{Event, GossipReceiver, GossipSender},
    Gossip, TopicId,
};
use n0_error::{Result, StdResultExt};
use n0_future::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::structs::{HostArgs, JoinArgs};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    body: MessageBody,
    nonce: [u8; 16],
}

#[derive(Debug, Serialize, Deserialize)]
enum MessageBody {
    Message { text: String },
}

impl Message {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(|e| e.to_string().into())
    }

    pub fn new(body: MessageBody) -> Self {
        Self {
            body,
            nonce: rand::random(), // I just wanted to write a comment that i think this is so
                                   // awesome you do this because gossip makes it so that the same
                                   // message can't be either sent or received twice.
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

fn secret_key_from_passphrase(passphrase: &str) -> SecretKey {
    let hash = Sha256::digest(passphrase.as_bytes());
    SecretKey::from_bytes(&hash.into())
}
fn get_secret(hostargs: &HostArgs) -> SecretKey {
    secret_key_from_passphrase(&("pomotimer".to_owned() + &hostargs.room))
}
pub async fn create_sender_receiver(
    hostargs: &HostArgs,
) -> Result<(GossipSender, GossipReceiver, Router)> {
    let endpoint = Endpoint::builder(presets::N0).bind().await?;
    let gossip = Gossip::builder().spawn(endpoint.clone());
    let router = Router::builder(endpoint)
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();
    let topic_id = TopicId::from_bytes([23u8; 32]);

    let (sender, receiver) = gossip
        .subscribe(topic_id, vec![get_secret(hostargs).public()])
        .await?
        .split();
    Ok((sender, receiver, router))
}

pub async fn create_entry(hostargs: &HostArgs) -> Result<(GossipReceiver, Router)> {
    let endpoint = Endpoint::builder(presets::N0)
        .secret_key(get_secret(hostargs))
        .bind()
        .await?;
    let gossip = Gossip::builder().spawn(endpoint.clone());
    let router = Router::builder(endpoint)
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();
    let topic_id = TopicId::from_bytes([23u8; 32]);
    let (_, receiver) = gossip.subscribe(topic_id, vec![]).await?.split();
    Ok((receiver, router))
}

async fn send_message(sender: GossipSender, message: Message) -> Result {
    sender.broadcast(message.to_vec().into()).await?;
    Ok(())
}
async fn receive_message(mut receiver: GossipReceiver, routers: Vec<Router>) -> Result {
    while let Some(event) = receiver.next().await {
        match event? {
            Event::Received(message) => {
                match Message::from_bytes(&message.content) {
                    Ok(msg) => {
                        println!("received: {:?}", msg.body);
                    }
                    Err(e) => println!("failed to deserialize: {}", e),
                }
                for i in &routers {
                    i.shutdown().await.std_context("shutdown router")?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub async fn host_iroh(hostargs: HostArgs) -> Result {
    let (mut entryreceiver, entryrouter) = create_entry(&hostargs).await?;
    let (sender, mut receiver, router) = create_sender_receiver(&hostargs).await?;

    entryreceiver.joined().await?;

    receive_message(receiver, vec![entryrouter, router]).await?;

    Ok(())
}
pub async fn join_iroh(joinargs: JoinArgs) -> Result {
    // let (sender, mut receiver, router) = create_sender_receiver(&hostargs).await?;
    Ok(())
}
