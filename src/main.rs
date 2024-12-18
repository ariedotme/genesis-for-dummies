use bevy::prelude::*;
use genesis_for_dummies::config::Config;
use genesis_for_dummies::server::*;
use tokio::{runtime::Runtime, sync::mpsc};

fn main() {
    let config = Config::load();

    let runtime = Runtime::new().unwrap();
    let (tx, rx) = mpsc::channel(1024);

    runtime.spawn(async move {
        udp_server(tx).await;
    });

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(UdpReceiver { rx })
        .insert_resource(config)
        .add_systems(Startup, setup_entities);

    app.run();
}

fn setup_entities(mut commands: Commands, config: Res<Config>) {
    if let Some(entities) = &config.entities {
        for entity in entities {
            println!(
                "Spawning entity: {:?} - Description: {:?}",
                entity.name,
                entity.description.as_deref().unwrap_or("No description")
            );

            commands.spawn((
                Name::new(entity.name.clone()),
                Transform::default(),
                GlobalTransform::default(),
            ));
        }
    } else {
        eprintln!("No entities found in the configuration.");
    }
}
