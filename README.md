# maplibre-test
Experimental repo for testing open-source mapping alternative (MapLibre) and hopefully a fully functional Rust-based frontend.

This is using trunk serve to run the localhost.

TODO
1. Get a basic website localhost done using Leptos
2. Render the map with maplibre-rs
3. Render the map with galileo (vector tiles)
4. Decide which library is better

## Mapping Backend
[Maplibre-RS](https://github.com/maplibre/maplibre-rs) (/src)
- maplibre-rs is a portable and performant vector maps renderer.
- Experimental Maps for Web, Mobile and Desktop

[Galileo](https://github.com/Maximkaaa/galileo) (galileo-src/src)
- *General purpose* cross-platform GIS-rendering library written in Rust
- Galileo is a cross-platform map rendering engine. It supports raster and vector layers, custom and flexible styling, working with different coordinate systems and projects.

## UI Frontend
[Leptos](https://leptos.dev/)

[Dioxus](https://dioxuslabs.com/)
