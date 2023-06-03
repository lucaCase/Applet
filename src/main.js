const { invoke } = window.__TAURI__.tauri;

function fetchAndDisplay(body) {
    document.querySelector("#content").innerHTML = body;
}

window.addEventListener("DOMContentLoaded", () => {
    const closeButton = document.querySelector("#close-button");
    const maximizeButton = document.querySelector("#maximize-button");
    const minimizeButton = document.querySelector("#minimize-button");
    const bar = document.querySelector("#navigation");
    closeButton.addEventListener("click", async () => {
        await invoke('close_window');
    });
    maximizeButton.addEventListener("click", async () => {
        await invoke('maximize_window');
    });
    minimizeButton.addEventListener("click", async () => {
        await invoke('minimize_window');
    });

});

addEventListener("keydown", async (e) => {
    if (e.key === "F12") {
        //await invoke('open_devtools');
    } else if (e.key === "Enter" && document.activeElement === document.querySelector("#url-input")) {
        document.querySelector("#content").innerHTML = await invoke('get_data', { url: document.querySelector("#url-input").value });
    }
});