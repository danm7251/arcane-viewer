import init, { get_arcana } from "./pkg/arcane_viewer.js";

const CARD_SIZE = 0.75;

async function run() {
    await init();  // loads WASM + JS bindings

    const welcomeScreen = document.getElementById("welcome-screen");
    const arcanaScreen = document.getElementById("arcanum-screen");
    const button = document.getElementById("button");
    const cards = get_arcana();

    button.addEventListener("click", () => {
        welcomeScreen.style.display = "none";
        arcanaScreen.style.display = "flex";
        render_cards(cards);
    });

}

// REVIEW: Do I want a table or a carousel?
function render_cards(cards) {
    const arcanumScreen = document.getElementById("cards");

    for (const card of cards) {
        const element = document.createElement("div");
        element.className = "card";

        element.innerHTML = `
            <img src="${card.img_src}" width="${CARD_SIZE*684}" height="${CARD_SIZE*1024}" />
        `;

        arcanumScreen.appendChild(element);
    }
}

document.addEventListener("DOMContentLoaded", run);