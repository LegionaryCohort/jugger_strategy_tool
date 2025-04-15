use crate::components::{counter_btn::Button, dev_error_view::dev_error_view};
use leptos::prelude::*;

const LOGO: &'static str =
    "https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg";
const LOGO_DARK: &'static str = "https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg";

/// Default Home Page
#[component]
pub fn DesktopPage() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=dev_error_view>
            <div class="container">

                <picture>
                    <source srcset=LOGO_DARK media="(prefers-color-scheme: dark)" />
                    <img src=LOGO alt="Leptos Logo" height="200" width="400" />
                </picture>

                <h1>"Desktop"</h1>

                <div class="buttons">
                    <Button />
                    <Button increment=5 />
                </div>

                <a href="/mobile">Switch to mobile version</a>
            </div>
        </ErrorBoundary>
    }
}
