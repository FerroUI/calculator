# Ferro Calculator

An initial native FerroUI port of [Microsoft Calculator](https://github.com/microsoft/calculator). **The original C++ CalcManager, CEngine and Ratpack sources are compiled unchanged.** Rust supplies platform adapters and view code-behind; FerroUI loads the calculator's adapted XAML. It runs natively on the desktop and in the browser, where the original engine is compiled to WebAssembly with the rest of the application.

**Try it in the browser:** https://ferroui.github.io/calculator/

Standard, Scientific and Programmer calculators, history and multi-item memory, bit toggling, Gregorian date calculations, all twelve physical-unit categories and upstream's explicitly fictional currency rates work. This is a working core port, **not yet a claim of complete application or visual parity**. Session persistence, the full original shell/list interactions, all adaptive layouts, graph UI integration and effect fidelity still have open work.

The running calculator uses the original CalculatorIcons.ttf and original keypad grids, names, command IDs and glyphs. Light/dark colours and native Reveal/Acrylic brush recipes come from App.xaml. The shell now uses FerroUI's native NavigationView and the window requests a Mica backdrop. Key hover/press uses native Reveal with platform policy fallbacks. The source's 1-DIP Reveal borders are retained, and the light spreads across the borders of neighbouring keys (MATERIAL-01, fixed in FerroUI). Five keypad documents are mechanically adapted and checked for reproducibility; the shell, calculator wrapper, date and converter views remain manual adaptations. The complete original XAML remains in the upstream submodule.

The pinned Calculator's modern key template uses solid fills. For the requested Reveal appearance, the port follows the [older Fluent CalcButtonStyle](https://github.com/microsoft/calculator/blob/6f49b17bf099c10830c15ea93e1207068896045b/src/Calculator/App.xaml#L142): a RootGrid that switches to a Reveal background on hover/press, and a ContentPresenter with a 1-DIP Reveal border. Both use modern 4-DIP rounded corners. There is no extra keypad backplate overlay or Calculator-specific gap mask. Modern default fills and the engine/keypad revision remain pinned; this version difference is explicitly tracked. Programmer radix rows use the original separate radio-row structure and subtle hover fills.

The two FerroUI defects that kept the neighbouring borders dark, the border light's expression lifetime and a thread-local light registry that lost lights across native rendering threads, are fixed in the FerroUI that the submodule pins. Text font availability and remaining substitute shell icons are tracked in [APP-06](docs/FERROUI-GAPS.md).

## Repository layout

| Path | Content |
|---|---|
| `src/`, `build.rs`, `native/` | the calculator (`ferro-calculator`): the views and their adapted XAML, the Rust engine adapter, and the C++ bridge and platform headers through which `build.rs` compiles the original engine |
| `Calculator.Browser/` | the browser host (`ferro-calculator-browser`): the WebAssembly entry point and its host page in `wwwroot/` |
| `upstream/calculator/` | [Microsoft Calculator](https://github.com/microsoft/calculator) as a git submodule at `79e1ac53`, unmodified |
| `FerroUI/` | [FerroUI](https://github.com/wieslawsoltes/FerroUI) as a git submodule; the calculator builds against its crates |
| `tests/` | the engine, date and UI tests |
| `scripts/`, `docs/` | the preservation and regeneration scripts, the porting notes and the compatibility tracker |

The repository follows the layout of FerroUI applications described in
[FerroUI/docs/apps.md](https://github.com/wieslawsoltes/FerroUI/blob/main/docs/apps.md): the workspace
dependencies and profiles of `Cargo.toml` are kept in step with the submodule by
`FerroUI/scripts/apps/sync-workspace.py`, and the CI is FerroUI's shared application workflow.

## Build and run

Requirements: Rust 1.89 or newer, Git, Python 3 and a C++17 toolchain, with the platform prerequisites of FerroUI: the Xcode command line tools on macOS; a C/C++ compiler, pkg-config, fontconfig and FreeType development packages on Linux; Visual Studio C++ tools and the Windows SDK on Windows. The first build downloads Skia binaries and builds HarfBuzz. CMake is not required.

```sh
git clone https://github.com/FerroUI/calculator.git
cd calculator
git -c core.autocrlf=false submodule update --init --recursive
cargo run --locked --profile fast-release
cargo run --locked --profile fast-release -- --scientific
cargo run --locked --profile fast-release -- --programmer --dark
```

Check out the submodules with `core.autocrlf=false`, as above, so that the original sources keep their line endings and the byte-for-byte preservation check also holds on Windows. Calculator is pinned at `79e1ac53` and FerroUI at a commit of its main branch, or of a FerroUI pull request while the calculator needs a change that is not merged yet; to move FerroUI forward, see section 7 of FerroUI/docs/apps.md.

### Browser

With the pinned browser toolchain of FerroUI (`FerroUI/scripts/browser/setup.sh`), whose em++ also compiles the original engine:

```sh
source ~/browser-tools/env.sh
FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh ferro-calculator-browser
```

Serve `target/browser/ferro-calculator-browser` with any static web server. The query string opens another page or theme (`?scientific`, `?programmer`, `?dark`) or selects the renderer (`?RenderingMode=Software2D`).

## Verify

```sh
python3 scripts/verify_upstream.py
python3 scripts/generate_engine_resources.py --check
python3 scripts/port_xaml.py --check
python3 FerroUI/scripts/apps/sync-workspace.py --check
cargo fmt --check
cargo test --locked --no-default-features
cargo test --locked --features headless --test ui --test reveal_gap -- --test-threads=1
FERRO_CALC_CAPTURE_DIR=artifacts cargo test --locked --features headless --test ui --test reveal_gap -- --test-threads=1
FERROUI_SMOKE_EXIT_MS=1500 cargo run --locked -- --scientific --dark
```

The core tests exercise decimal precision beyond f64, standard versus scientific evaluation, functions, integer widths and signed overflow, bases, bit toggling, native history replay, memory, original conversion ratios/offsets, mock currency, paste validation and date boundaries. The UI test loads every ported view, opens a real flyout, drives keyboard/pointer routing, exercises native NavigationView and Settings selection, checks original error visual states and dates, switches themes and exports actual compositor frames. It checks that Reveal changes as the pointer moves inside one key, and becomes a stable solid fallback when material policy disables effects. `tests/reveal_gap.rs` checks that both unhovered neighbours light their borders when the pointer is in the gap between two keys. The UI tests use FerroUI headless plus Skia, not a replacement UI renderer; each test file runs in a process of its own.

CI runs all of the above on Linux, the UI tests under a virtual X server, then builds the browser site, opens it in headless Chrome and publishes it to GitHub Pages from main. Local verification is on macOS; Windows is not claimed as tested.

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
