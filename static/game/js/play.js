window.addEventListener('DOMContentLoaded', (event) => {
    // const gamePlaySocketUrl = JSON.parse(document.getElementById("gameWsUrl").textContent);
    const gamePlaySocketUrl = "ws://localhost:3000/websocket";
    const websocket = new WebSocket(gamePlaySocketUrl);
    websocket.onopen = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} opened!`);
    }
    websocket.onclose = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} closed!`);
    } 
});
