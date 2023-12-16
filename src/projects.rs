use crate::modrinth::modrinth::Project;

#[derive(Debug)]
pub struct Result {
    pub version: String,
    pub loader: String,
    pub count: i32,
    pub mods: Vec<String>
}

impl Result {
    pub fn new(version: String, loader: String) -> Result {
        Result {
            version,
            loader,
            count: 0,
            mods: Vec::new()
        }
    }

    pub fn add(&mut self, project_name: String) {
        self.count += 1;
        self.mods.push(project_name)
    }
}

pub async fn find_most_compatible_mods(projects: &Vec<Project>) -> Vec<Result> {
    // Scan through all the projects. Find all projects who have a certain mod loader. Then find all mods who have a certain game version
    // Then repeat this step for all mod loaders and all mods
    // Then print out the result

    let mut loaders_vec: Vec<String> = Vec::new();
    let mut versions_vec: Vec<String> = Vec::new();

    for project in projects {
        for loader in project.loaders.as_ref().unwrap() {
            if !loaders_vec.contains(loader) {
                loaders_vec.push(loader.clone());
            }
        }

        for version in project.game_versions.as_ref().unwrap() {
            if !versions_vec.contains(version) {
                versions_vec.push(version.clone());
            }
        }
    }

    let mut results_vec: Vec<Result> = Vec::with_capacity(loaders_vec.len() * versions_vec.len());

    println!("Built vec with capacity {}!", results_vec.capacity());

    for loader in loaders_vec.iter() {
        for version in versions_vec.iter() {
            results_vec.push(Result::new(version.clone(), loader.clone()));
        }
    }

    for project in projects {
        for result in results_vec.iter_mut() {
            if project.game_versions.as_ref().unwrap().contains(&result.version)
                && project.loaders.as_ref().unwrap().contains(&result.loader) {
                result.add(project.title.clone());
            }
        }
    }

    println!("Done adding! Reordering...");

    results_vec.sort_by(|a, b| b.count.cmp(&a.count));

    println!("Reorder done!");

    for result in results_vec.iter() {
        println!("Version: {}, Loader: {}, Count: {}/{}, Mods: {:?}", result.version, result.loader, result.count, projects.len(), result.mods);
    }

    return results_vec;
}