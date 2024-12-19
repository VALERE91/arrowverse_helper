use leptos::*;
use wasm_bindgen::prelude::*;

use crate::navbar::NavBar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <NavBar />
            <h1>"Welcome to Tauri + Leptos"</h1>
        </main>
    }
}
