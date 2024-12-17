use bevy::prelude::*;
use genesis_for_dummies::config::Config;

fn main() {
    let config = Config::load();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(config)
        .add_systems(Startup, setup_entities)
        .run();
}

fn setup_entities(mut commands: Commands, config: Res<Config>) {
    match config.entities.clone() {
        Some(entities) => {
            for entity in entities {
                println!("Spawning entity: {:?}", entity.name);
                commands.spawn((
                    Name::new(entity.name),
                    Transform::default(),
                    GlobalTransform::default(),
                ));
            }
        }
        None => {}
    }
}
