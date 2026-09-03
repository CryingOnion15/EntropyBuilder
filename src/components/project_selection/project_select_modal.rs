use dioxus::prelude::*;

const PSM_CSS: Asset = asset!("assets/styling/project_select_modal.css");

#[component]
pub fn ProjectSelectModal(
    on_click: EventHandler,
    project_name: String,
    project_location: String,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PSM_CSS }

        div { id: "project-select-window",
            div { id: "project-select-container",
                div {
                    p { "Project Name: {project_name}" }
                }
                div {
                    p { "Project Location: {project_location}" }
                }
                button { onclick: move |_| on_click.call(()), "Launch Project" }
            }
        }
    }
}
