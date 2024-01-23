mod error;
mod platform;

use leptos::{component, create_signal, IntoView, SignalGet, SignalSet, view};
use maplibre::environment::OffscreenKernelEnvironment;
use maplibre::event_loop::EventLoop;
use maplibre::io::source_client::{HttpSourceClient, SourceClient};
use maplibre::kernel::{Kernel, KernelBuilder};
use maplibre::map::Map;
use maplibre::render::builder::RendererBuilder;
use maplibre::style::Style;
use maplibre_winit::{WinitEnvironment, WinitMapWindowConfig};
use crate::platform::http_client::WHATWGFetchHttpClient;
use crate::platform::UsedOffscreenKernelEnvironment;


pub struct WHATWGOffscreenKernelEnvironment;

impl OffscreenKernelEnvironment for WHATWGOffscreenKernelEnvironment {
    type HttpClient = WHATWGFetchHttpClient;

    fn create() -> Self {
        WHATWGOffscreenKernelEnvironment
    }

    fn source_client(&self) -> SourceClient<Self::HttpClient> {
        SourceClient::new(HttpSourceClient::new(WHATWGFetchHttpClient::default()))
    }
}

#[cfg(not(target_feature = "atomics"))]
type CurrentEnvironment = WinitEnvironment<
    maplibre::io::scheduler::NopScheduler,
    WHATWGFetchHttpClient,
    UsedOffscreenKernelEnvironment,
    platform::singlethreaded::apc::PassingAsyncProcedureCall,
    (),
>;

#[cfg(target_feature = "atomics")]
type CurrentEnvironment = WinitEnvironment<
    platform::multithreaded::pool_scheduler::WebWorkerPoolScheduler,
    WHATWGFetchHttpClient,
    UsedOffscreenKernelEnvironment,
    maplibre::io::apc::SchedulerAsyncProcedureCall<
        UsedOffscreenKernelEnvironment,
        platform::multithreaded::pool_scheduler::WebWorkerPoolScheduler,
    >,
    (),
>;

pub type MapType = Map<CurrentEnvironment>;

// #[wasm_bindgen]
// pub async fn run_maplibre(new_worker: js_sys::Function) -> Result<(), JSError> {
pub async fn run_maplibre() {
    let mut kernel_builder = KernelBuilder::new()
        .with_map_window_config(WinitMapWindowConfig::new("maplibre".to_string()))
        .with_http_client(WHATWGFetchHttpClient::default());

    // #[cfg(target_feature = "atomics")]
    // {
    //     kernel_builder = kernel_builder
    //         .with_apc(maplibre::io::apc::SchedulerAsyncProcedureCall::new(
    //             platform::multithreaded::pool_scheduler::WebWorkerPoolScheduler::new(
    //                 new_worker.clone(),
    //             )?,
    //         ))
    //         .with_scheduler(
    //             platform::multithreaded::pool_scheduler::WebWorkerPoolScheduler::new(new_worker)?,
    //         );
    // }
    //
    // #[cfg(not(target_feature = "atomics"))]
    // {
    //     kernel_builder = kernel_builder
    //         .with_apc(platform::singlethreaded::apc::PassingAsyncProcedureCall::new(new_worker, 4))
    //         .with_scheduler(maplibre::io::scheduler::NopScheduler);
    // }

    let kernel: Kernel<WinitEnvironment<_, _, UsedOffscreenKernelEnvironment, _, ()>> =
        kernel_builder.build();

    let mut map: MapType = Map::new(
        Style::default(),
        kernel,
        RendererBuilder::new(),
        vec![
            Box::<maplibre::render::RenderPlugin>::default(),
            Box::<maplibre::vector::VectorPlugin<platform::UsedVectorTransferables>>::default(),
            // Box::new(RasterPlugin::<platform::UsedRasterTransferables>::default()),
            #[cfg(debug_assertions)] Box::<maplibre::debug::DebugPlugin>::default(),
        ],
    )
        .unwrap();
    map.initialize_renderer().await.unwrap();

    map.window_mut()
        .take_event_loop()
        .expect("Event loop is not available")
        .run(map, None);
}

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
