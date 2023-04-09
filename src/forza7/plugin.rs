use crate::forza7::telemetry_model::ForzaTelemtry;
use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::error::Error;
use tokio::net::UdpSocket;

const MAX_DATAGRAM_SIZE: usize = 311;

pub struct Forza7Plugin;

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<Vec<u8>>);
pub struct StreamEvent {
    pub telemetry: ForzaTelemtry
}

impl Plugin for Forza7Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StreamEvent>()
            .add_startup_system(setup_udp)
            .add_system(read_stream);
    }
}

fn setup_udp(mut commands: Commands) {
    let (tx, rx) = bounded::<Vec<u8>>(10);
    std::thread::spawn(move || {
        telemetry_stream(tx).unwrap();
    });

    commands.insert_resource(StreamReceiver(rx));
}

#[tokio::main]
async fn telemetry_stream(tx: Sender<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut data: Vec<u8> = vec![0; MAX_DATAGRAM_SIZE];
    loop {
        let len = socket.recv(&mut data).await?;
        tx.send(data[..len].to_vec()).unwrap();
    }
}

// This system reads from the receiver and sends events to Bevy
fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    for from_stream in receiver.try_iter() {
        let mut telemetry = ForzaTelemtry::default();
        telemetry.update(from_stream);
        events.send(StreamEvent { telemetry });
    }
}
