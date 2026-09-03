//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.
mod project_select;
pub use project_select::ProjectSelect;

mod create_new;
pub use create_new::CreateNewModal;

mod add_existing;
pub use add_existing::AddExistingModal;

mod project_select_modal;
pub use project_select_modal::ProjectSelectModal;

mod loader;
pub use loader::Loader;

mod project_explorer;
pub use project_explorer::ProjectExplorer;
