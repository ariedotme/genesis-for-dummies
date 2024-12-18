use std::net::SocketAddr;

use bevy::{
    core::Name,
    prelude::{Commands, GlobalTransform, ResMut, Resource, Transform},
};
use serde::{Deserialize, Serialize};
use tokio::{net::UdpSocket, sync::mpsc};

/*

TODO:
- Write proper packets with single responsibility
- Use Protobuf for serialization
- Add some environment variables

*/

#[derive(Resource)]
pub struct UdpReceiver {
    pub rx: mpsc::Receiver<(SocketAddr, UdpMessage)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UdpMessage {
    pub entity_id: String,
    pub action: String,
}

pub async fn udp_server(tx: mpsc::Sender<(SocketAddr, UdpMessage)>) {
    let socket = UdpSocket::bind("127.0.0.1:8080").await.unwrap();
    println!("UDP server running on 127.0.0.1:8080");

    let mut buf = [0; 1024];
    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            let data = &buf[..len];
            if let Ok(msg) = serde_json::from_slice::<UdpMessage>(data) {
                println!("Received from {}: {:?}", addr, msg);
                tx.send((addr, msg)).await.unwrap();
            }
        }
    }
}

// Refactor this so it doesn't directly spawns the commands
pub fn process_udp_messages(mut udp_receiver: ResMut<UdpReceiver>, mut commands: Commands) {
    while let Ok((addr, msg)) = udp_receiver.rx.try_recv() {
        println!("Processing UDP message from {}: {:?}", addr, msg);

        commands.spawn((
            Name::new(format!("Entity: {}", msg.entity_id)),
            Transform::default(),
            GlobalTransform::default(),
        ));
    }
}
