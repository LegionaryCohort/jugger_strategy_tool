mod components;
mod pages;

use crate::pages::desktop::DesktopPage;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::not_found::NotFound;

pub const CANVAS_ID: &'static str = "bevy-canvas";

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <WebApp /> }
    })
}

#[component]
pub fn WebApp() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Title text="Welcome to Leptos CSR" />

        <script type="module">"import init from './jugger_strategy_tool.js'; init();"</script>

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=DesktopPage />
            </Routes>
        </Router>
    }
}
