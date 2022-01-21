window.addEventListener("DOMContentLoaded", (event) => {
    const gamePlaySocketUrl = JSON.parse(document.getElementById("gameWsUrl").textContent);

    const websocket = new WebSocket(gamePlaySocketUrl);

    websocket.onopen = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} opened!`);
    }

    websocket.onclose = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} closed!`);
    } 
});
