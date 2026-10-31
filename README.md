# Ferro Calculator

An initial native FerroUI port of [Microsoft Calculator](https://github.com/microsoft/calculator). **The original C++ CalcManager, CEngine and Ratpack sources are compiled unchanged.** Rust supplies platform adapters and view code-behind; FerroUI loads the calculator's adapted XAML.

Standard, Scientific and Programmer calculators, history and multi-item memory, bit toggling, Gregorian date calculations, all twelve physical-unit categories and upstream's explicitly fictional currency rates work. This is a working core port, **not yet a claim of complete application or visual parity**. Session persistence, the full original shell/list interactions, all adaptive layouts, graph UI integration and effect fidelity still have open work.

The running calculator uses the original CalculatorIcons.ttf and original keypad grids, names, command IDs and glyphs. Light/dark colours and native Reveal/Acrylic brush recipes come from App.xaml. The shell now uses FerroUI's native NavigationView and the window requests a Mica backdrop. Key hover/press uses native Reveal with platform policy fallbacks. The source's 1-DIP Reveal borders are retained, but the neighboring-edge glow has a failing rendering reproducer on pinned FerroUI/Skia (MATERIAL-01). Five keypad documents are mechanically adapted and checked for reproducibility; the shell, calculator wrapper, date and converter views remain manual adaptations. The complete original XAML remains in the upstream submodule.

The pinned Calculator's modern key template uses solid fills. For the requested Reveal appearance, the port follows the [older Fluent CalcButtonStyle](https://github.com/microsoft/calculator/blob/6f49b17bf099c10830c15ea93e1207068896045b/src/Calculator/App.xaml#L142): a RootGrid that switches to a Reveal background on hover/press, and a ContentPresenter with a 1-DIP Reveal border. Both use modern 4-DIP rounded corners. There is no extra keypad backplate overlay or Calculator-specific gap mask. Modern default fills and the engine/keypad revision remain pinned; this version difference is explicitly tracked. Programmer radix rows use the original separate radio-row structure and subtle hover fills.

A separately exported [FerroUI Reveal patch series](patches/ferroui-reveal-lighting.patch) corrects both the border-light expression lifetime and a thread-local light registry that lost lights across native rendering threads. With both fixes, the neighbouring-border pixel reproducer and independent numeric/operator surface-lighting checks pass; the user also confirmed the spreading border light in the rebuilt native window. The application's Git dependency remains unchanged pending upstream adoption. Text font availability and remaining substitute shell icons are tracked in [APP-06](docs/FERROUI-GAPS.md).

## Build and run

Requirements: Rust 1.89 or newer, Git and a C++17 toolchain. macOS needs Xcode command-line tools; Windows needs Visual Studio C++ tools and the Windows SDK; Linux needs the desktop build dependencies documented by [FerroUI](https://github.com/wieslawsoltes/FerroUI). Skia/native backend prerequisites are FerroUI's, and Cargo builds them. CMake is not required by this application.

```sh
git -c core.autocrlf=false submodule update --init --recursive
cargo run --locked
cargo run --locked -- --scientific
cargo run --locked -- --programmer --dark
```

Both upstream projects are pinned: Calculator `79e1ac53` as a Git submodule, FerroUI main `62b48431` (audited 2026-10-31) in Cargo.toml/Cargo.lock. Normal builds have no dependency on a sibling local checkout; local patched verification builds use an external Cargo override.

When space is limited, use a build directory on another volume:

```sh
CARGO_TARGET_DIR=/tmp/ferrocalc-target cargo run --locked
```

## Verify

```sh
python3 scripts/verify_upstream.py
python3 scripts/generate_engine_resources.py --check
python3 scripts/port_xaml.py --check
cargo fmt --check
cargo test --locked --no-default-features
cargo test --locked --features headless --test ui -- --test-threads=1
FERRO_CALC_CAPTURE_DIR=artifacts cargo test --locked --features headless --test ui -- --test-threads=1
FERROUI_SMOKE_EXIT_MS=1500 cargo run --locked -- --scientific --dark
```

The core tests exercise decimal precision beyond f64, standard versus scientific evaluation, functions, integer widths and signed overflow, bases, bit toggling, native history replay, memory, original conversion ratios/offsets, mock currency, paste validation and date boundaries. The UI test loads every ported view, opens a real flyout, drives keyboard/pointer routing, exercises native NavigationView and Settings selection, checks original error visual states and dates, switches themes and exports actual compositor frames. It checks that Reveal changes as the pointer moves inside one key, and becomes a stable solid fallback when material policy disables effects. It uses FerroUI headless plus Skia, not a replacement UI renderer.

The separate gap-border pixel reproducer is explicitly ignored in the ordinary suite until MATERIAL-01 is resolved. It currently fails on pinned FerroUI/Skia and can be run with `cargo test --locked --features headless --test ui reveal_gap_lights_both_unhovered_neighbor_borders -- --ignored --test-threads=1`. Passing the hover tests does not close this rendering gap.

CI is configured to run engine checks on macOS, Linux and Windows and render the UI on macOS. Local verification is on macOS; the other platforms are not claimed as locally tested.

## Fidelity and missing framework features

[`docs/FERROUI-GAPS.md`](docs/FERROUI-GAPS.md) is the working compatibility document. It distinguishes missing APIs, partial composition semantics, ordinary platform adaptations and application work. In particular:

- Reveal brushes/states, AcrylicBrush, NavigationView, ThemeShadow and alpha masks are now available upstream; the port adopts the materials and navigation APIs.
- Light/dark resources preserve original material types and recipes. The framework's material policy controls transparency/energy-saver fallbacks.
- Existing lights, effect factories, backdrops and shadows still have documented rendering/platform differences that need native Windows visual verification.
- Original error visual-state groups are restored; remaining adaptive/storyboard groups and the history/list/swipe presentation remain open.
- Microsoft's proprietary graph engine is absent upstream. Graphing is visibly unavailable; no replacement graph engine is invented.
- Currency uses the exact static fictional feed from upstream developer builds and is labelled as mock data throughout its page.

[`docs/PORTING.md`](docs/PORTING.md) maps files and records the architecture and validation. [`docs/xaml-adaptations.json`](docs/xaml-adaptations.json) retains every removed or translated keypad member, including the complete original state groups. [`docs/upstream-manifest.json`](docs/upstream-manifest.json) checks all 775 original files byte for byte.

## License

MIT. Preserve [NOTICE.md](NOTICE.md), the upstream Calculator license/notices and FerroUI/dependency notices with distributions.
