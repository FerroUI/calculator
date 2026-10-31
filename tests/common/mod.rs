//! Helpers of the UI scenarios (tests/ui.rs, tests/reveal_gap.rs): each runs in a process of its
//! own, because the headless platform belongs to the first scenario of a process.
#![allow(dead_code)]
use ferroui::headless::HeadlessWindowExtensions;
use ferroui::{
    base::{media::imaging::BitmapEncoderOptions, threading::Dispatcher},
    prelude::*,
};
pub fn settle(window: &Ref<Window>) {
    window.update_layout();
    Dispatcher::ui_thread().run_jobs(None);
    window.update_layout();
}
pub fn capture(window: &Ref<Window>, name: &str) {
    settle(window);
    if let Ok(dir) = std::env::var("FERRO_CALC_CAPTURE_DIR") {
        std::fs::create_dir_all(&dir).unwrap();
        // Capture the compositor's actual frame: RenderTargetBitmap bypasses the
        // material/light render walk and cannot verify Reveal or acrylic.
        let bitmap = window.capture_rendered_frame().expect("composited frame");
        bitmap
            .save_to_file(
                &format!("{dir}/{name}.png"),
                &BitmapEncoderOptions::Png(Default::default()),
            )
            .unwrap();
    }
}
pub fn region_pixels(window: &Ref<Window>, rect: ferroui::base::PixelRect) -> Vec<u8> {
    // Callers describe layout regions in DIP; capture pixels follow the DPI.
    let scale = window.render_scaling();
    let rect = ferroui::base::PixelRect::new(
        (f64::from(rect.x) * scale) as i32,
        (f64::from(rect.y) * scale) as i32,
        (f64::from(rect.width) * scale) as i32,
        (f64::from(rect.height) * scale) as i32,
    );
    let bitmap = window.capture_rendered_frame().expect("composited frame");
    assert_eq!(bitmap.format().unwrap().bits_per_pixel(), 32);
    let stride = rect.width * 4;
    let mut pixels = vec![0; (stride * rect.height) as usize];
    bitmap.copy_pixels(rect, &mut pixels, stride);
    pixels
}
