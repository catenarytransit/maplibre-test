mod entry;
mod vector_tiles;
mod common;

use leptos::{component, create_signal, IntoView, SignalGet, SignalSet, view};


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        // <button
        //     on:click=move |_| {
        //         set_count.set(3);
        //     }
        // >
        //     "Click me: "
        //     {move || count.get()}
        // </button>
        <canvas id="maplibre-rs" style="width:100vw; height:100vh; background-color: red;">
        </canvas>
    }
}

fn main() {
    leptos::mount_to_body(|| view!{
        <App/>
    })
}
