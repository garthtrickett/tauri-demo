import { html, render, TemplateResult } from "lit-html";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// Define the shape of our application state.
interface AppState {
  title: string;
  count: number;
}

// Get the root element from the HTML.
const root: HTMLElement =
  document.getElementById("app") || document.createElement("div");

// Renders the UI using lit-html with Franken UI styling.
const renderApp = (): TemplateResult => {
  // Disable buttons if an API call is in progress.
  const isDisabled = state.title === "API call in progress...";
  // Disable decrement when count is 0.
  const decrementDisabled = state.count === 0;
  // Disable increment when count is 5.
  const incrementDisabled = state.count === 5;
  return html`
    <div class="uk-container mt-40">
      <div class="space-y-4">
        <h1 class="uk-hero-sm text-center font-bold">${state.title}</h1>
        <!-- Increased font-size for the counter number -->
        <p class="text-center" style="font-size: 2rem;">
          Countdown: ${state.count}
        </p>
        ${state.count > 0
          ? html`
              <div style="max-width: 600px; margin: 0 auto; padding-top: 1rem;">
                <p class="text-center">
                  Note: When the countdown hits zero, the backend will trigger
                  an external API call, displaying "API call in progress..."
                  followed by a final message. Also, when incrementing the
                  counter to five, a different trigger will activate.
                </p>
              </div>
            `
          : ""}
        <div class="flex justify-center gap-x-2">
          <button
            class="uk-btn uk-btn-default"
            ?disabled=${decrementDisabled || isDisabled}
            @click=${() => actions.decrementCounter()}
          >
            Decrement
          </button>
          <button
            class="uk-btn uk-btn-default"
            ?disabled=${incrementDisabled || isDisabled}
            @click=${() => actions.incrementCounter()}
          >
            Increment
          </button>
          <button
            class="uk-btn uk-btn-primary"
            ?disabled=${isDisabled}
            @click=${() => actions.resetCounter()}
          >
            Reset
          </button>
        </div>
      </div>
    </div>
  `;
};

// Local cache for application state.
let state: AppState = { title: "", count: 0 };

// Listen for state updates from the backend.
listen("state_updated", (event) => {
  if (event.payload) {
    state = event.payload as AppState;
    render(renderApp(), root);
  }
});

// Define actions that call the corresponding Rust commands.
const actions = {
  async decrementCounter() {
    state = await invoke<AppState>("decrement_counter");
    render(renderApp(), root);
  },
  async incrementCounter() {
    state = await invoke<AppState>("increment_counter");
    render(renderApp(), root);
  },
  async resetCounter() {
    state = await invoke<AppState>("reset_counter");
    render(renderApp(), root);
  },
};

// Fetch initial state from the backend.
async function init() {
  state = await invoke<AppState>("get_state");
  render(renderApp(), root);
  // Remove spinner overlay once the app has loaded.
  const spinner = document.getElementById("spinner");
  if (spinner) {
    spinner.remove();
  }
}

init();
