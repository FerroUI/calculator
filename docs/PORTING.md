# Calculator port report

## Architecture and preservation

Calculator is pinned as an unmodified Git submodule. The source manifest covers every tracked original file, including the C++ math engine, XAML, resources, icons and tests. Builds read the original C++ translation units directly from that tree. No generated or patched copy of the arithmetic engine is substituted.

`build.rs` compiles CalcManager, all CEngine and Ratpack translation units, NumberFormattingUtils and UnitConverter with the `cc` crate. New `native/compat` headers replace platform-only includes on non-MSVC toolchains: standard headers, the unused intsafe include, HRESULT definitions already provided by upstream, and completed offline task continuations. MSVC uses its original SDK headers/tasks. This is not an implementation of general PPL scheduling or a network provider.

`native/bridge.cpp` implements ICalcDisplay/IResourceProvider/IConverterDataLoader outside the original tree and owns one CalculatorManager/UnitConverter context. All exceptions terminate at the C ABI. Context-owned UTF-8 snapshot buffers are copied while locked; Rust owns and releases the opaque context exactly once. `src/engine.rs` is the only Rust FFI boundary, is not Send/Sync, and serializes calls because the upstream engine has process-global constants. UI methods release mutable borrows before invoking view updates. UI callbacks use weak references to avoid control/event ownership cycles.

`scripts/generate_engine_resources.py` extracts en-US CEngine resources, native command numbers, physical-unit names/IDs, ratios, temperature offsets and the fictional currency feed from the pinned original files. SI/PT defaults are selected. Native unit IDs are retained; converter category IDs are local routing identifiers. The upstream loader's stray trailing space in the millimetres-of-mercury resource lookup is trimmed during extraction. No Rust floating-point replacement computes calculator arithmetic. UnitConverter's double ratios are retained because that is the original native converter's algorithm.

## File inventory

| Port | Original source | State |
|---|---|---|
| `upstream/calculator/src/CalcManager/**` | Same paths | Unmodified; compiled directly, except the MSVC-only pch.cpp translation unit. |
| `native/bridge.cpp`, `src/engine.rs` | `CalcManager.Interop/CalcManagerImpl.cpp`, display/resource/loader contracts | Platform adapter; no WinRT ABI dependency. History replay follows StandardCalculatorViewModel, including negative-operand command placement and suppressed display callbacks. |
| `src/engine_commands.rs` | `Calculator.ViewModels/Common/NumbersAndOperatorsEnum.cs` | Generated numeric mappings; Rust memory-store action explicitly uses manager API. |
| `native/engine_resources.generated.h` | `Resources/en-US/*.resw`, `DataLoaders/UnitConverterDataLoader.cs`, `CurrencyHttpClient.cs` | Generated en-US/SI data adapter; original ratios, offsets and mock feed. Regional/localized settings remain open. |
| `src/app.rs` | `Calculator/App.xaml.cs` | Native FerroUI application/lifetime/assets/theme. Windows packaging and session state remain open. |
| `src/views/main_page.rs`, `MainPage.xaml`, generated `navigation.json` | `Views/MainPage.xaml(.cs)`, `Calculator.ViewModels/Common/NavCategory.cs` | Native NavigationView in source LeftMinimal mode, original menu order/glyphs, settings footer and selection events. Custom titlebar/history presentation remain adapted. |
| `src/views/calculator.rs`, `Calculator.xaml` | `Views/Calculator.xaml(.cs)`, `OperatorsPanel.xaml(.cs)`, angle/programmer display code | Calculator composition and states; wrapper manually adapted. Original result/expression-control behavior is partial. |
| `src/views/NumberPad.xaml` | `Views/NumberPad.xaml` | Mechanically adapted grid, coordinates, names, IDs and glyphs; code-behind wires native commands. |
| `src/views/CalculatorStandardOperators.xaml` | Matching original | Mechanically adapted; original keypad grid and nested NumberPad retained. VSM/error and layout exceptions inventoried. |
| `src/views/CalculatorScientificOperators.xaml` | Matching original | Mechanically adapted including full trigonometric/function flyout trees and alternate function columns. Shift/hyperbolic/inverse visibility uses Rust. |
| `src/views/CalculatorProgrammerRadixOperators.xaml` | Matching original | Mechanically adapted including bitwise/shift flyouts, A–F keypad and rotate/carry variants. |
| `src/views/CalculatorScientificAngleButtons.xaml` | Matching original | Mechanically adapted; angle cycle/F-E wired in Rust. |
| `src/views/history.rs` | `Views/HistoryList.xaml.cs`, `Memory.xaml.cs`, memory view models | Native history recall/removal and multi-item memory actions; list visuals/swipe/announcements are partial adaptations. |
| `src/views/input.rs`, generated shortcut metadata | `Common/KeyboardShortcutManager.cs`, keypad shortcut resource declarations, `Controls/CalculatorButton.cs` | Basic keyboard routing, extracted keypad shortcuts and clipboard adapters. Tunnel routing preserves Enter-to-evaluate semantics. Full platform shortcut/localization audit remains. |
| `src/views/unit_converter.rs`, `UnitConverter.xaml` | `Views/UnitConverter.xaml(.cs)` and its view model | Manually adapted, original native converter performs calculations. Selection, reverse conversion and mock-currency notice work; original responsive layouts/animations remain. |
| `src/date_calculator.rs`, `src/views/date_calculator.rs`, `DateCalculator.xaml` | `ViewModels/Common/DateCalculator.cs`, `Views/DateCalculator.xaml(.cs)` | Gregorian adaptation of the Windows calendar adapter. Preserves date floor and larger-first addition/smaller-first subtraction, including month-end clamping. Other calendar systems remain open. |
| `src/styles/CalculatorResources.xaml` | App.xaml theme colour/material resources | Generated original light/dark colours and native Reveal/Acrylic types and recipe values; framework material policy controls fallbacks. |
| `src/styles/CalculatorStyles.xaml` | App.xaml CalcButtonStyle family, older Fluent Reveal template and ProgrammerOperators ProgModeRadioButtonStyle | Source-derived RootGrid and bordered ContentPresenter, pointer/press state setters and original 1-DIP margin/border. Modern 4-DIP rounded corners clip both layers. Hover/press replaces the root background; no added keypad backplate layer. Radix rows use a separate radio template with aligned labels/values, subtle hover fill, accent selection indicator and the source top/bottom RevealBackplate. Neighbor-border glow works in the separately patched build. |
| Original graph files retained upstream | `Views/GraphingCalculator/**`, GraphingInterfaces/GraphingImpl | Not adapted to a working graph UI. Proprietary engine is absent upstream. |

Several related code-behind classes share an adapter module where their WinRT types are replaced by native FerroUI controls; the table states these mappings. Generated XAML is not described as a verbatim XAML port. Every deferred keypad subtree/attribute is listed in the generated inventory. Manual wrapper differences are explicitly listed here and in FERROUI-GAPS.md.

## Verification and limits

### Main upgrade, 2026-10-31 (macOS)

FerroUI is now pinned to main `62b4843106ac69d3769b2f8e3dd771837534b960`. This adopts upstream features without a framework fork. The 2026-10-23 results below remain a historical baseline, not the current dependency audit.

- Replaced the overlay shell with native NavigationView, original category order and CalculatorIcons glyphs, LeftMinimal mode and Settings footer. Keyboard routing now checks IsPaneOpen rather than the removed NavPane.
- Preserved original Reveal/Acrylic brush declarations and recipe values, including StaticResource text-brush aliases that the initial extractor omitted. The aliases fix dark-theme key text. Omitted Reveal TargetTheme is made explicit per dictionary. Native MicaBackdrop is requested on the application window; operator flyouts consume the original theme-specific background resource.
- Adopted native Reveal hover/press/normal/disabled states. The subsequent source-history audit below replaces the initial extra backplate layer with the older Fluent template's RootGrid and bordered ContentPresenter. Source 1-DIP key margins/borders, minimum sizes and normal numeric font weight are restored; normal-state fill resources remain modern.
- Restored all five source ErrorVisualStates groups and GoToState calls. Remaining custom/adaptive/storyboard groups stay in the inventory; no claim that these are all restored.
- Preservation and generation checks pass: 775 original files unchanged, 111 engine resources, 176 commands, nine generated outputs and 807 inventoried adaptations/restorations.
- Full headless suite passes: seven engine, two date and one UI integration test. UI assertions cover native navigation/Settings, actual pointer-following Reveal in both themes, connected native brushes/lights, policy fallback without pointer lighting, theme text colours, error-state entry/recovery, flyouts and the existing mode/converter/date/history paths.
- Captures use the headless window's actual Skia compositor frame rather than RenderTargetBitmap, which bypasses the composition material/light render walk. Reviewed Standard light/dark and Reveal, navigation, Scientific and Programmer output; converter/date/history captures are exported too. These are local verification artifacts, not Windows reference goldens.
- Native Scientific/dark and Programmer/dark desktop smoke runs open and exit successfully. Clippy with all targets/headless and `-D warnings` passes. Debug linking still emits the existing macOS large-unwind-table warning.

Build output for this upgrade is in `/Volumes/SSD/ferrocalc-target`. Remaining framework and application differences are tracked in [FERROUI-GAPS.md](FERROUI-GAPS.md); native Windows, Vello, accessibility, non-English and exact WinUI visual comparisons remain unverified.

### Reveal source-history audit, 2026-10-31

The user correctly identified the older Calculator's illumination along neighboring button edges. The [original 2019 App.xaml](https://github.com/microsoft/calculator/blob/6f49b17bf099c10830c15ea93e1207068896045b/src/Calculator/App.xaml#L142) sets Margin=1 and BorderThickness=1, switches RootGrid.Background to a RevealBackgroundBrush only for PointerOver/Pressed, and gives ContentPresenter an always-present transparent RevealBorderBrush (removed when disabled). This yields illuminated edge outlines around the gaps; the empty margin itself has no brush. There is no Calculator-specific alpha/opacity mask in this template. WinUI's Reveal material builds a scene-lighting effect and the border paint geometry limits its coverage. Background Reveal separately uses a noise alpha mask internally, not a keypad gap mask.

The application keypad template now follows that structure with modern ControlCornerRadius (4 DIP) on both background and border, and no added RevealBackplate. A Border replaces the source root Grid so its background also clips to the corner radius. The pinned modern Calculator default-fill resources remain in use, so this is a documented combination of source versions. The original submodule remains unchanged.

A focused pixel reproducer moves the pointer into the margin between Plus and Minus, checks the keys are unhovered and the native border brush is connected, and compares each neighboring 1-DIP edge strip against policy fallback while keeping an interior patch unchanged. **It currently fails on the audited FerroUI/Skia revision: neighboring edge strips do not change.** Material/lights API availability and the passing hover tests therefore did not establish the requested border effect. The reproducible test is retained as an explicitly ignored known rendering failure, runnable with `cargo test --locked --features headless --test ui reveal_gap_lights_both_unhovered_neighbor_borders -- --ignored --test-threads=1`. The normal suite has 10 passing tests and this separate ignored reproducer. See MATERIAL-01; no framework code or substitute gap glow was added.

The separate upstream patch described in MATERIAL-01 fixes the expression lifetime. With the external dependency override, the edge reproducer passes in both themes; normal numeric/operator hover checks also compare interior pixels separately from borders and verify original hover/press recipe colours. Run the ignored edge reproducer and the normal UI test in separate processes because FerroUI's global Dispatcher belongs to the first test thread. The unpatched application pin still needs upstream adoption. Updated captures are in `artifacts/radix-fidelity`.

Native-window follow-up exposed a second FerroUI issue that capture-only checks did not cover: a thread-local server light registry lost lights when rendering moved between synchronous UI-thread frames and the render thread. The separate FerroUI worktree now keeps that registry on ServerCompositor. A regression that creates a XAML light on one thread and samples its scene-lighting map on another fails before the registry fix. All 97 composition tests pass with the fix, including a second regression for directly targeted lights. The Calculator edge test now exercises both 1x and 2x display scaling. See the combined patch series linked from MATERIAL-01; the rounded-key template does not implement a substitute lighting effect.

The programmer radix selector now follows the original full-row RadioButton structure instead of individual content-sized keypad buttons. All four rows have the same width and value alignment, the selected base has its accent marker, and the original subtle hover/press fills replace the incorrectly reused keypad Reveal surface. Its top/bottom RevealBackplate uses the existing native system brushes in place of missing ListViewItemReveal theme aliases. The original adaptive row sizing and custom copy/keyboard contracts remain open. Font/icon parity is tracked as APP-06: the original symbol font is embedded, but this Mac lacks Segoe text fonts and some shell icons are still substitutes.

The build/test commands are in the README. Core tests cover semantic differences, precision and adapter failure paths. Headless UI checks include keyboard focus, pointer hit testing, real flyout opening, scientific commands, programmer conversions and radio selection, all converter pages, dates, themes and wide history layout. Captured PNGs are review artifacts, not golden WinUI comparisons.

The independent source check verifies all 775 files against their recorded SHA-256 digests and checks the submodule revision when Git metadata is available. Regeneration checks compare the extracted data and adapted markup with the checked-in files. The application never edits the FerroUI checkout.

Native Windows/WinUI captures, renderer-specific lighting/effects, accessibility clients, non-Gregorian and non-English settings, the full tiny/large adaptive states and Windows integration have not been certified. See the compatibility tracker for each incomplete feature. A type existing in FerroUI is not proof that it reproduces WinUI's rendering semantics.

### Local verification, 2026-10-23 (macOS)

| Check | Result |
|---|---|
| `cargo test --locked --features headless -- --test-threads=1` | 7 engine tests + 2 date tests + 1 UI integration test passed; 0 failures. |
| `cargo clippy --locked --all-targets --features headless -- -D warnings` | Passed. |
| `cargo fmt --check` | Passed. |
| Upstream verification | 775 unchanged original files at the pinned Calculator revision. |
| Engine/resource and XAML regeneration checks | Passed; 111 engine resource entries, 176 command entries, 8 generated XAML/metadata/inventory files, 802 explicit keypad adaptations. |
| Native desktop smoke | Scientific/dark window opened and automatically closed; exit code 0. Programmer/dark smoke also checked. |
| Rendering review | Inspected light/dark Standard, Scientific, Programmer, temperature, mock currency and wide history PNGs. Captures for all twelve converter categories and dates were generated too. |
| Pinned framework audit | Rechecked Reveal/Acrylic and composition-light/effect limitations against the actual `934b9ce1` dependency source and its guides. FerroUI remains unchanged. |

Builds used `CARGO_TARGET_DIR=/tmp/ferrocalc-target` after the workspace volume ran out of space. The only files moved were this project's generated build files. The debug linker emits a large-unwind-table warning from the linked UI build; it did not prevent tests or native startup. Windows/Linux jobs are configured but were not executed locally. Use LF checkout for the upstream submodule as shown in the README so the byte-preservation check also remains meaningful on Windows.
