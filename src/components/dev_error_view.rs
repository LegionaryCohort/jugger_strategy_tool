use leptos::prelude::*;

pub fn dev_error_view(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view! {
        <h1>"Uh oh! Something went wrong!"</h1>
        <p>"Errors: "</p>
        <ul>
            {move || {
                errors
                    .get()
                    .into_iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            }}

        </ul>
    }
}
