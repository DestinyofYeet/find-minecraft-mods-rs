mod projects;

mod modrinth {
    pub mod modrinth;
}

use serde::{Deserialize, Serialize};
use modrinth::modrinth::Modrinth;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub name: String,
    pub id: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WantedProjects {
    pub mods: Vec<ProjectData>
}

#[tokio::main]
async fn main() {
    let file_handler = std::fs::read_to_string("data/list_of_mods_wanted.json");

    if file_handler.is_err() {
        panic!("Failed to read file list_of_mods_wanted.json")
    }

    let content = file_handler.unwrap();

    let wanted = serde_json::from_str::<WantedProjects>(content.as_str()).unwrap();

    let mut ids: Vec<String> = Vec::new();
    ids.reserve_exact(wanted.mods.len());


    for project in wanted.mods.iter() {
        ids.push(project.id.clone());
    }

    println!("{:?}", ids);
    let mut api = Modrinth::new(String::from("ModFinder (ole@uwuwhatsthis.de)"));
    let projects = api.get_projects(ids).await;

    // for project in projects.iter() {
    //     println!("Name: {} | Game versions: {:?} | Mod loaders: {:?}", project.title, project.game_versions, project.loaders)
    // }

    projects::find_most_compatible_mods(&projects).await;
}