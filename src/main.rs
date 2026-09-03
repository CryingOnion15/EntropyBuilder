use components::loading::Loader;
use components::project_editing::ProjectExplorer;
use components::project_selection::ProjectSelect;
use dioxus::prelude::*;
use library::EntropyProject;

use crate::library::misc;

/// Modules.
mod components;
mod library;

// Import the css files.
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const COZETTE: Asset = asset!("/assets/fonts/CozetteVector.ttf");

// Application State
#[derive(Clone)]
pub struct GlobalState {
    pub active_project: Option<EntropyProject>,
    pub application_state: misc::AppState,
}

fn main() {
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // App signals.
    let global_state = use_signal(|| GlobalState {
        active_project: None,
        application_state: misc::AppState::SELECTION,
    });
    // let mut active_project = use_signal(|| None::<EntropyProject>);
    // let mut application_state: Signal<misc::AppState> = use_signal(|| misc::AppState::SELECTION);

    use_context_provider(|| global_state);

    rsx! {
        style {
            "@font-face {{
                font-family: 'PixelFont';
                src: url('{COZETTE}') format('truetype');
                font-weight: normal;
                font-style: normal;
            }}"
        }

        // Doc links
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        ApplicationView {}

    }
}

#[component]
fn ApplicationView() -> Element {
    let global_state = use_context::<Signal<GlobalState>>();
    let app_state = global_state.read().application_state;

    match app_state {
        misc::AppState::SELECTION => rsx! {
            ProjectSelect {}
        },

        misc::AppState::LOADING => rsx! {
            Loader {}
        },

        misc::AppState::OPENPROJECT => rsx! {
            ProjectExplorer {}
        },
    }
}
