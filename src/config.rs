use bevy::ecs::system::Resource;
use serde::Deserialize;
use serde_yaml;
use std::fs;
use std::path::Path;

/*

The game will be data oriented, and the yaml configs play a big role on this.
Every single yaml file inside of data (and sub-folders) is read.
On the future yaml files will be able to reference lua scripts, and other types of configuration
will allow this to be versatile and powerful.

*/

#[derive(Debug, Deserialize, Resource, Clone)]
pub struct Config {
    pub entities: Option<Vec<Entity>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub components: Option<Vec<Component>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Component {
    pub r#type: String,
    pub resistances: Option<String>,
    pub thresholds: Option<Vec<Threshold>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Threshold {
    pub trigger: Trigger,
    pub behaviors: Vec<Behavior>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Trigger {
    pub damage: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Behavior {
    pub r#type: String,
    pub entity: Option<String>,
    pub count: Option<u32>,
    pub action: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        return Self::load_all_from_directory("assets/data");
    }

    fn load_all_from_directory(directory: &str) -> Self {
        let mut all_entities = Vec::new();

        visit_dirs(Path::new(directory), &mut |file_path| {
            if file_path.extension().and_then(|ext| ext.to_str()) == Some("yaml") {
                let yaml_data = fs::read_to_string(file_path)
                    .unwrap_or_else(|_| panic!("Failed to read file: {:?}", file_path));
                let config: Config = serde_yaml::from_str(&yaml_data)
                    .unwrap_or_else(|_| panic!("Failed to parse YAML: {:?}", file_path));

                if let Some(entities) = config.entities {
                    all_entities.extend(entities);
                }
            }
        });

        Self {
            entities: (!all_entities.is_empty()).then_some(all_entities),
        }
    }
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&std::path::Path)) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb);
            } else {
                cb(&path);
            }
        }
    }
}
