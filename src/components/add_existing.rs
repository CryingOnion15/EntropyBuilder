use dioxus::prelude::*;

const AE_CSS: Asset = asset!("assets/styling/add_existing.css");

#[component]
pub fn AddExistingModal(on_submit: EventHandler<(String)>) -> Element {
    let mut project_location = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: AE_CSS }

        div { id: "add-existing-window",
            form {
                id: "add-existing-form",
                onsubmit: move |evt| {
                    evt.prevent_default();
                    on_submit.call(project_location());
                },

                div {
                    label { "Project Location: " }
                    input {
                        r#type: "file",
                        accept: ".ebuild",
                        id: "project_location",
                        name: "project_location",
                        onchange: move |evt| {
                            let path = evt.files().first().unwrap().path().to_string_lossy().to_string();
                            project_location.set(path);
                            let test = project_location();
                            println!("Test {test}");
                        },
                    }
                }

                button { r#type: "submit", "Add Existing" }
            }
        }
    }
}
