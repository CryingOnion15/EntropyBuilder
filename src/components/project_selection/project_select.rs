use super::add_existing::AddExistingModal;
use super::create_new::CreateNewModal;
use super::project_select_modal::ProjectSelectModal;
use crate::library::misc;
use crate::library::EntropyProject;
use crate::GlobalState;
use dioxus::prelude::*;
use directories::ProjectDirs;
use serde_json;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// Resource References
const PS_CSS: Asset = asset!("assets/styling/project_select.css");
const GREEN_PLUS: Asset = asset!("assets/sprites/GreenPlus.png");
const RED_MINUS: Asset = asset!("assets/sprites/RedMinus.png");

#[component]
pub fn ProjectSelect() -> Element {
    //State
    let mut projects: Signal<Vec<Vec<String>>> = use_signal(|| find_project_history());
    let mut selected_project: Signal<Option<usize>> = use_signal(|| None);
    let mut show_create = use_signal(|| false);
    let mut show_add_existing = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: PS_CSS }

        div { class: "window",
            // side menu
            div { class: "side-menu",
                // menu buttons.
                div { id: "side-menu-buttons",
                    // Create
                    button {
                        class: "side-button",
                        onclick: move |_| {
                            show_create.set(true);
                            show_add_existing.set(false);
                            selected_project.set(None);
                        },
                        img {
                            style: "max-width: 100%; height: 80%;",
                            src: GREEN_PLUS,
                        }
                        p { class: "button-text", "Create New." }
                    }
                    // add existing
                    button {
                        class: "side-button",
                        onclick: move |_| {
                            show_create.set(false);
                            show_add_existing.set(true);
                            selected_project.set(None);
                        },
                        img {
                            style: "max-width: 100%; height: 80%;",
                            src: GREEN_PLUS,
                        }
                        p { class: "button-text", "Add existing." }
                    }
                    // remove selected
                    button {
                        class: "side-button",
                        onclick: move |_| {
                            if selected_project() != None {
                                remove_project_from_history(selected_project().expect("Not an integer"));
                                projects.set(find_project_history());
                                selected_project.set(None);
                                show_create.set(false);
                            }
                        },
                        img {
                            style: "max-width: 100%; height: 80%;",
                            src: RED_MINUS,
                        }
                        p { class: "button-text", "Remove From List." }
                    }
                }
                // List of previous projects.
                div { id: "side-menu-projects",
                    if projects.is_empty() {
                        h2 { style: "color: var(--primary-font-color)",
                            "There are no recent projects."
                        }
                    } else {
                        ul { class: "project-list",
                            for (index , item) in projects.iter().enumerate() {
                                li {
                                    class: if selected_project() == Some(index) { "project-item active" } else { "project-item" },
                                    onclick: move |_| {
                                        if (selected_project() == Some(index)) {
                                            selected_project.set(None);
                                        } else {
                                            selected_project.set(Some(index));
                                        }
                                        show_create.set(false);
                                        show_add_existing.set(false);
                                    },
                                    h2 { "{item[0]}" }
                                    p { "{item[1]}" }
                                }
                            }
                        }
                    }
                }
            }

            // main viewer
            div { class: "project-viewer",
                if show_create() {
                    CreateNewModal {
                        on_submit: move |(name, location)| {
                            create_new_world_file(name, location);
                            projects.set(find_project_history());
                            show_create.set(false);
                        },
                    }
                } else if show_add_existing() {
                    AddExistingModal {
                        on_submit: move |location| {
                            add_existing_project_to_hist(location);
                            projects.set(find_project_history());
                            show_add_existing.set(false);
                        },
                    }
                } else if let Some(index) = selected_project() {
                    if let Some(project) = projects().get(index).cloned() {
                        ProjectSelectModal {
                            on_click: {
                                let location = project[1].clone();

                                move |_| {
                                    launch_selected_project(location.clone());
                                }
                            },
                            project_name: project[0].clone(),
                            project_location: project[1].clone(),
                        }
                    }
                }
            }
        }

    }
}

fn find_project_history() -> Vec<Vec<String>> {
    let dirs = ProjectDirs::from("com", "KingdomDaydreamers", "EntropyBuilder")
        .expect("Could not find application data directory");
    let save_dir = dirs.data_dir().join("save-data");
    let proj_hist = save_dir.join("project-history.txt");

    fs::create_dir_all(&save_dir).expect("Could not create save directory.");

    if !proj_hist.exists() {
        fs::File::create(&proj_hist).expect("Could not create save file.");
    }

    let file = fs::File::open(&proj_hist).expect("Count not open project history");
    let reader = BufReader::new(file);
    let mut projects = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        let split: Vec<String> = line.split("|").map(String::from).collect();
        println!("{:?}", split);
        projects.push(split);
    }

    println!("{:?}", projects);
    return projects;
}

fn add_existing_project_to_hist(location: String) {
    // Load exiting project.
    let mut project = EntropyProject::new();
    project.load_from_build_file(&location);

    // Get Project History file.
    let dirs = ProjectDirs::from("com", "KingdomDaydreamers", "EntropyBuilder")
        .expect("Could not find application data directory");
    let save_dir = dirs.data_dir().join("save-data");
    let proj_hist = save_dir.join("project-history.txt");

    let file = fs::File::open(&proj_hist).expect("Could not open project history");

    let reader = BufReader::new(file);

    let target = format!("{}|{}", project.project_name, &location);

    let already_exists = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line)| line.ok().map(|line| (i, line)))
        .find(|(_, line)| line == &target)
        .map(|(i, _)| i);

    // If already in the list remove it and move it to the top.
    if already_exists != None {
        remove_project_from_history(already_exists.unwrap());
    }

    let existing_content = fs::read_to_string(&proj_hist).unwrap_or_default();
    let new_line = format!("{}|{}\n", project.project_name, &location);
    let new_content = format!("{}{}", new_line, existing_content);

    fs::write(&proj_hist, new_content).expect("Could not write to project history.");
}

fn remove_project_from_history(index: usize) {
    let dirs = ProjectDirs::from("com", "KingdomDaydreamers", "EntropyBuilder")
        .expect("Could not find application data directory");
    let save_dir = dirs.data_dir().join("save-data");
    let proj_hist = save_dir.join("project-history.txt");

    let file = fs::File::open(&proj_hist).expect("Could not open project history");

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .filter_map(|(i, line)| if i == index { None } else { Some(line) })
        .collect();

    // Reopen the file, truncating the existing contents.
    let file = fs::File::create(&proj_hist).expect("Could not create project history");

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line).expect("Could not write project history");
    }
}

fn create_new_world_file(project_name: String, project_location: String) {
    // Project file path.
    let path = Path::new(&project_location);
    let file_name = format!("{}.ebuild", project_name);
    let file = path.join(file_name);

    // Initialize some data here.
    let mut new_ebuild = EntropyProject::new();
    new_ebuild.update_fields(serde_json::json!({"project_name": format!("{}", project_name)}));

    // Write/Create project name to new file.
    fs::write(file, new_ebuild.serialize()).expect("Could not Create and write to ebuild file.");

    // Update recent project history
    let dirs = ProjectDirs::from("com", "KingdomDaydreamers", "EntropyBuilder")
        .expect("Could not find application data directory");
    let save_dir = dirs.data_dir().join("save-data");
    let proj_hist = save_dir.join("project-history.txt");

    let existing_content = fs::read_to_string(&proj_hist).unwrap_or_default();
    let new_line = format!("{}|{}\n", project_name, project_location);
    let new_content = format!("{}{}", new_line, existing_content);

    fs::write(&proj_hist, new_content).expect("Could not write to project history.");
}

fn launch_selected_project(project_location: String) {
    let mut global_state = use_context::<Signal<GlobalState>>();
    global_state.write().application_state = misc::AppState::LOADING;

    let mut project = EntropyProject::new();
    project.load_from_build_file(&project_location);

    global_state.write().active_project = Some(project);
    global_state.write().application_state = misc::AppState::OPENPROJECT;
}
