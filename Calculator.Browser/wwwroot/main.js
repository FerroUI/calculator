import { FerroExports } from "./ferroui.js";
import { loadModule } from "./ferroui-loader.js";

const host = document.getElementById("out");
try {
    const loaded = await loadModule({ script: "./ferro-calculator-browser.js", element: host });
    if (loaded) {
        const runtime = await loaded.createRuntime();
        FerroExports.attach(runtime);
        globalThis.calculator = runtime;
        runtime.runMain(globalThis.location.href);
    }
} catch (error) {
    console.error(error);
    const panel = document.createElement("section");
    panel.className = "boot-error";
    const title = document.createElement("h1");
    title.textContent = "Ferro Calculator could not start";
    const detail = document.createElement("p");
    detail.textContent = error instanceof Error ? error.message : String(error);
    const retry = document.createElement("button");
    retry.textContent = "Try again";
    retry.addEventListener("click", () => location.reload());
    panel.append(title, detail, retry);
    host.replaceChildren(panel);
}
