use crate::views::MainPage;
use ferroui::{
    base::{
        controls::ResourceDictionary,
        styling::{Styles, ThemeVariant},
    },
    markup_xaml_loader::FerroRuntimeXamlLoader,
    prelude::*,
};
use std::rc::Rc;

#[repr(C)]
pub struct App {
    base: Application,
}
ferro_class!(App: Application);
ferro_impl_classes!(App: FerroObjectImpl);
impl NewApplication for App {
    fn new_application() -> Ref<Self> {
        instantiate(Self {
            base: Application::construct(),
        })
    }
}
impl ApplicationImpl for App {
    fn initialize(this: &Self) {
        Self::parent_initialize(this);
        this.set_name(Some("Ferro Calculator".into()));
        this.styles().add(WinUITheme::new().as_style());
        ferroui::base::platform::register_assets(
            "FerroCalc",
            &[(
                "/Assets/CalculatorIcons.ttf",
                include_bytes!("../upstream/calculator/src/Calculator/Assets/CalculatorIcons.ttf"),
            )],
        );
        ferroui::base::media::FontManager::current().add_font_collection(Rc::new(
            ferroui::base::media::fonts::EmbeddedFontCollection::new(
                ferroui::base::utilities::Uri::absolute("fonts:Calculator")
                    .expect("font collection URI"),
                ferroui::base::utilities::Uri::absolute("ferres://FerroCalc/Assets")
                    .expect("asset URI"),
            ),
        ));
        let resources: Ref<ResourceDictionary> =
            FerroRuntimeXamlLoader::parse_as(include_str!("styles/CalculatorResources.xaml"), None)
                .expect("Calculator resources");
        this.set_resources(resources);
        let styles: Ref<Styles> =
            FerroRuntimeXamlLoader::parse_as(include_str!("styles/CalculatorStyles.xaml"), None)
                .expect("Calculator styles");
        this.styles().add(styles);
        this.set_requested_theme_variant(Some(ThemeVariant::light()));
    }
    fn on_framework_initialization_completed(this: &Self) {
        if let Some(lifetime) = this
            .application_lifetime()
            .as_ref()
            .and_then(|l| l.as_classic_desktop_style_application_lifetime())
        {
            lifetime.set_main_window(Some(create_main_window()));
        }
        Self::parent_on_framework_initialization_completed(this);
    }
}
pub fn create_main_window() -> Ref<Window> {
    let window = Window::new();
    window.set_title(Some("Calculator — FerroUI".into()));
    window.set_width(420.0);
    window.set_height(680.0);
    window.set_min_width(336.0);
    window.set_min_height(500.0);
    // Transparent NavigationView content exposes the platform's window material.
    window.set_system_backdrop(Some(ferroui::controls::media::MicaBackdrop::new().upcast()));
    let view = MainPage::new().expect("original calculation engine and XAML");
    window.set_content(Some(Control::boxed(view.root.clone())));
    view.attach(&window);
    if std::env::args().any(|s| s == "--scientific") {
        view.navigate(crate::views::Page::Calculator(
            crate::engine::Mode::Scientific,
        ));
        window.set_height(800.0);
    }
    if std::env::args().any(|s| s == "--programmer") {
        view.navigate(crate::views::Page::Calculator(
            crate::engine::Mode::Programmer,
        ));
        window.set_height(840.0);
    }
    if std::env::args().any(|s| s == "--dark") {
        Application::current()
            .unwrap()
            .set_requested_theme_variant(Some(ThemeVariant::dark()));
    }
    if let Some(ms) = std::env::var("FERROUI_SMOKE_EXIT_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
    {
        let window = window.clone();
        ferroui::base::threading::DispatcherTimer::run_once(
            move || window.close(),
            std::time::Duration::from_millis(ms),
            ferroui::base::threading::DispatcherPriority::NORMAL,
        );
    }
    window
}
#[cfg(feature = "desktop")]
pub fn run() -> std::process::ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    std::process::ExitCode::from(
        AppBuilder::configure::<App>()
            .use_platform_detect()
            .start_with_classic_desktop_lifetime(&args) as u8,
    )
}
