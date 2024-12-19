use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            <h1>{"Welcome to Arrowverse"}</h1>
            <p>{"This is a demo app to showcase the Arrowverse TV shows."}</p>
        </div>
    }
}
