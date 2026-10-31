//! MATERIAL-01, the Reveal light on the borders of neighbouring keys, in a process of its own:
//! the headless platform belongs to the first scenario of a process (tests/ui.rs has the other).
#![cfg(feature = "headless")]
mod common;
use common::{capture, region_pixels, settle};
use ferro_calculator::{app::App, views::MainPage};
use ferroui::headless::HeadlessWindowExtensions;
use ferroui::{
    headless::{FerroHeadlessPlatformExtensions, FerroHeadlessPlatformOptions},
    prelude::*,
    skia::SkiaApplicationExtensions,
};
/// MATERIAL-01: with the pointer in the gap between two keys, the source's
/// unhovered 1-DIP Reveal borders of both neighbours light up (fixed upstream in
/// FerroUI; this was the reproducer).
#[test]
fn reveal_gap_lights_both_unhovered_neighbor_borders() {
    AppBuilder::configure::<App>()
        .use_headless(FerroHeadlessPlatformOptions {
            use_headless_drawing: false,
            should_render_on_ui_thread: false,
            ..Default::default()
        })
        .use_skia()
        .setup_without_starting();
    Application::current()
        .unwrap()
        .set_requested_theme_variant(Some(ferroui::base::styling::ThemeVariant::dark()));
    let view = MainPage::new().unwrap();
    let window = Window::new();
    window.set_width(420.0);
    window.set_height(680.0);
    window.set_content(Some(Control::boxed(view.root.clone())));
    view.attach(&window);
    window.show();
    settle(&window);
    let ops = view.operators.borrow().clone().unwrap();
    let key = ops.get_control::<Button>("PlusButton");
    let root = key
        .get_visual_descendants()
        .find_map(|v| {
            v.cast::<Border>()
                .filter(|v| v.name().as_deref() == Some("RootGrid"))
        })
        .unwrap();
    for scaling in [1.0, 2.0] {
        window.set_render_scaling(scaling);
        Application::current()
            .unwrap()
            .set_requested_theme_variant(Some(ferroui::base::styling::ThemeVariant::dark()));
        settle(&window);
        verify_gap_reveal(&window, &ops, &key, &root, true);
        MainPage::toggle_theme();
        settle(&window);
        verify_gap_reveal(&window, &ops, &key, &root, false);
    }
    window.close();
}

fn verify_gap_reveal(
    window: &Ref<Window>,
    ops: &Ref<ferroui::controls::UserControl>,
    key: &Ref<Button>,
    root: &Ref<Border>,
    dark: bool,
) {
    use ferroui::base::{PixelRect, Point, input::RawInputModifiers, media::MaterialPolicy};
    use ferroui::controls::media::{RevealBorderBrush, RevealBrush, RevealBrushState};
    let origin = key.translate_point(Point::new(0.0, 0.0), window).unwrap();
    let neighbor = ops.get_control::<Button>("MinusButton");
    let neighbor_origin = neighbor
        .translate_point(Point::new(0.0, 0.0), window)
        .unwrap();
    window.mouse_move(
        Point::new(origin.x + 8.0, origin.y + 8.0),
        RawInputModifiers::NONE,
    );
    // A 1-DIP margin on each key leaves a 2-DIP gap. The pointer is outside
    // both keys, so no hover background can account for the border lighting.
    window.mouse_move(
        Point::new(origin.x + key.bounds().width / 2.0, origin.y - 1.0),
        RawInputModifiers::NONE,
    );
    assert_eq!(RevealBrush::get_state(root), RevealBrushState::Normal);
    assert!(!key.is_pointer_over() && !neighbor.is_pointer_over());
    let presenter = key
        .get_visual_descendants()
        .find_map(|v| v.cast::<ferroui::controls::presenters::ContentPresenter>())
        .expect("original content presenter");
    let brush = presenter.border_brush().expect("border brush");
    assert!(
        brush
            .as_object()
            .unwrap()
            .downcast_ref::<RevealBorderBrush>()
            .unwrap()
            .is_connected()
    );
    // Pump actual compositor frames through the border-light fade; merely
    // sleeping can leave its first server-side animation batch unprocessed.
    let started = std::time::Instant::now();
    while started.elapsed() < std::time::Duration::from_millis(400) {
        let _ = window.capture_rendered_frame();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(
        window.lights().to_vec().iter().any(|light| {
            light
                .downcast_ref::<ferroui::controls::media::RevealBorderLight>()
                .is_some_and(|light| {
                    light.is_light_theme() == !dark
                        && light.is_on()
                        && light.spot_light().is_some_and(|spot| spot.is_enabled())
                })
        }),
        "native border light for the current theme must be enabled"
    );
    let top = PixelRect::new(
        origin.x as i32 + 3,
        origin.y as i32,
        key.bounds().width as i32 - 6,
        1,
    );
    let bottom = PixelRect::new(
        neighbor_origin.x as i32 + 3,
        (neighbor_origin.y + neighbor.bounds().height) as i32 - 1,
        neighbor.bounds().width as i32 - 6,
        1,
    );
    let center = PixelRect::new(origin.x as i32 + 8, origin.y as i32 + 12, 4, 4);
    let lit_top = region_pixels(window, top);
    let lit_bottom = region_pixels(window, bottom);
    let unhovered_center = region_pixels(window, center);
    capture(
        window,
        if dark {
            "reveal-gap-dark"
        } else {
            "reveal-gap-light"
        },
    );
    MaterialPolicy::set_simulate_disabled_by_policy(true);
    std::thread::sleep(std::time::Duration::from_millis(350));
    let top_changed = lit_top != region_pixels(window, top);
    let bottom_changed = lit_bottom != region_pixels(window, bottom);
    assert!(
        unhovered_center == region_pixels(window, center),
        "unhovered key interiors must keep their normal fill"
    );
    assert!(
        top_changed && bottom_changed,
        "gap pointer must illuminate both borders: lower edge changed={top_changed}, upper neighbor changed={bottom_changed}"
    );
    MaterialPolicy::set_simulate_disabled_by_policy(false);
    window.mouse_move(Point::new(-10.0, -10.0), RawInputModifiers::NONE);
}
