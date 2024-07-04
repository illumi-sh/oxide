use std::path::PathBuf;
use crate::OxideProject;

pub fn find_root_project_dir(current_dir: PathBuf) -> PathBuf {
    let parent = current_dir.parent();
    return match parent {
        None => current_dir,
        Some(it) => {
            let parent_dir = it.to_path_buf();
            if is_project(parent_dir.clone()) {
                find_root_project_dir(parent_dir.clone())
            } else {
                current_dir
            }
        }
    }
}

pub fn is_project(current_directory: PathBuf) -> bool {
    return current_directory.join("../../../../oxide.toml").exists();
}

pub fn print_project(project: OxideProject, depth: usize) {
    if depth == 0 {
        println!("{} [root]", project.name);
    } else {
        println!("{}{}", "| ".repeat(depth), project.name);
    }

    match project.subprojects {
        None => {} // do nothing
        Some(subprojects) => {
            for subproject in subprojects {
                print_project(subproject, depth + 1);
            }
        }
    }
}