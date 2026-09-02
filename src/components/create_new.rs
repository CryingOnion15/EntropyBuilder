use dioxus::prelude::*;

const CN_CSS: Asset = asset!("assets/styling/create_new.css");

#[component]
pub fn CreateNewModal(on_submit: EventHandler<(String, String)>) -> Element {
    let mut project_name = use_signal(String::new);
    let mut project_location = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: CN_CSS }

        div { id: "create-new-window",
            form {
                id: "create-new-form",
                onsubmit: move |evt| {
                    evt.prevent_default();
                    on_submit.call((project_name(), project_location()));
                },

                div {
                    label { "Project Name: " }
                    input {
                        r#type: "text",
                        id: "project_name",
                        name: "project_name",
                        value: "{project_name}",
                        oninput: move |evt| {
                            project_name.set(evt.value());
                        },
                    }
                }

                div {
                    label { "Project Location: " }
                    input {
                        r#type: "file",
                        directory: true,
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

                button { r#type: "submit", "Create" }
            }
        }
    }
}
