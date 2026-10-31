//! The browser host of Ferro Calculator.
//!
//! The host page (`wwwroot/main.js`) creates the WebAssembly module, hands it to the script side of
//! the platform and calls [`run_main`] with the address of the page; the calculator starts in the
//! element `out` of the page as a single view and the function returns, leaving the page to drive
//! it. The original C++ engine is compiled into the same module by the build script of
//! `ferro-calculator`.
//!
//! The browser has no system fonts: the embedded Inter font is the default family, beside the
//! original CalculatorIcons font that the calculator registers itself.
//!
//! The query string selects the renderer (`?RenderingMode=Software2D` draws without WebGL) and the
//! first page (`?scientific`, `?programmer`, `?dark`).

#![cfg_attr(target_os = "emscripten", no_main)]

#[cfg(target_os = "emscripten")]
use wasm_bindgen::prelude::*;

#[cfg(target_os = "emscripten")]
#[wasm_bindgen(js_name = runMain)]
pub fn run_main(href: &str) {
    use ferroui_base::logging::LogEventLevel;
    use ferroui_base::media::FontManagerOptions;
    use ferroui_browser::{BrowserAppBuilder, BrowserPlatformOptions, BrowserRenderingMode};
    use ferroui_controls::AppBuilder;
    use ferroui_fonts_inter::AppBuilderExtension;
    use std::rc::Rc;

    let mut options = BrowserPlatformOptions::default();
    if href.contains("RenderingMode=Software2D") {
        options.rendering_mode = vec![BrowserRenderingMode::Software2D];
    }
    let query = href.split_once('?').map_or("", |(_, query)| query);
    ferro_calculator::app::set_start_options(ferro_calculator::app::StartOptions {
        scientific: query.split('&').any(|p| p == "scientific"),
        programmer: query.split('&').any(|p| p == "programmer"),
        dark: query.split('&').any(|p| p == "dark"),
    });
    AppBuilder::configure::<ferro_calculator::app::App>()
        .with_inter_font()
        .with(Rc::new(FontManagerOptions {
            default_family_name: Some("fonts:Inter#Inter".to_owned()),
            ..Default::default()
        }))
        .log_to_text_writer(std::io::stdout(), LogEventLevel::Warning, &[])
        .start_browser_app("out", Some(options));
}

#[cfg(not(target_os = "emscripten"))]
fn main() {
    println!(
        "Build the browser site with: FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh ferro-calculator-browser"
    );
}
