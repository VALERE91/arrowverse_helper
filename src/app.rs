use crate::components::*;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use navbar::NavBar;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <main>
                <Routes fallback=home::Home>
                    <Route
                        path=path!("")
                        view=home::Home />
                    <Route
                        path=path!("shows")
                        view=shows::Shows />
                    <Route
                        path=path!("next_episode")
                        view=next_episode::NextEpisode />
                </Routes>
            </main>
        </Router>
    }
}
