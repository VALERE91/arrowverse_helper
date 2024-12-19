use leptos::prelude::*;

use crate::backend::shows::TVShow;

#[component]
pub fn Shows() -> impl IntoView {
    let shows_res = LocalResource::new(move || TVShow::get_all());
    let async_shows = move || {
        shows_res
            .get()
            .as_deref()
            .or(Some(&Vec::<TVShow>::new()))
            .unwrap()
            .clone()
    };

    view! {
        <div class="grid grid-cols-4 gap-20">
            {
                async_shows().into_iter()
                    .map(|show| view! {
                        <div class="card bg-base-100 w-96 shadow-xl">
                            <figure>
                            <img src={show.poster.clone()} alt={show.name.clone()} />
                            </figure>
                            <div class="card-body">
                            <h2 class="card-title">{show.name.clone()}</h2>
                            <div class="card-actions justify-end">
                                <button class="btn btn-primary">Details</button>
                            </div>
                            </div>
                        </div>
                    })
                    .collect_view()
            }
        </div>
    }
}
