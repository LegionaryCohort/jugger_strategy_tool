use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::not_found::NotFound;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::desktop::DesktopPage;
use crate::pages::mobile::MobilePage;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=DesktopPage />
                <Route path=path!("/mobile") view=MobilePage />
            </Routes>
        </Router>
    }
}
