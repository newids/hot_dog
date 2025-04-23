use dioxus::prelude::*;
use log; // Keep the log import for other parts of the code
use tracing::Level; // Import Level from tracing
use wasm_bindgen_futures::spawn_local; // Import spawn_local for WASM

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    // Init logger using tracing::Level as indicated by the compiler error
    dioxus_logger::init(Level::INFO).expect("failed to init logger"); // Use tracing's Level::INFO
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(serde::Deserialize)]
struct CatApi {
    id: String,
    url: String,
    width: u32,  // Use u32 for dimensions
    height: u32, // Use u32 for dimensions
}

#[component]
fn DogView() -> Element {
    // const DOG_API_URL = "https://dog.ceo/api/breeds/image/random".to_string();
    let CAT_API_URL = "https://api.thecatapi.com/v1/images/search";

    let mut img_src =
        use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg".to_string());

    // Define the async logic separately
    let fetch_new_img = move || async move {
        log::info!("Fetching new cat image...");
        match reqwest::get(CAT_API_URL).await {
            Ok(response) => {
                log::debug!("Fetch successful, parsing JSON...");
                match response.json::<Vec<CatApi>>().await {
                    Ok(cat_images) => {
                        if let Some(first_cat) = cat_images.first() {
                            log::info!("Got image URL: {}", first_cat.url);
                            img_src.set(first_cat.url.clone()); // Clone the String to set the signal
                        } else {
                            log::warn!("API returned empty array");
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to fetch from Cat API: {}", e);
            }
        }
    };

    // This closure is passed to onclick. It spawns the async task.
    let fetch_new_handler = move |_| {
        spawn_local(fetch_new_img()); // Use spawn_local for WASM
    };

    let skip = move |_| {
        log::info!("Skip button clicked"); // Added log for skip
    };
    // let save = move |evt| {};

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            // Use the handler that spawns the task
            button { onclick: fetch_new_handler, id: "save", "save!" }
        }
    }
}
