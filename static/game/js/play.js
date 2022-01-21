window.addEventListener("DOMContentLoaded", (event) => {
    const gamePlaySocketUrl = JSON.parse(document.getElementById("gameWsUrl").textContent);

    const websocket = new WebSocket(gamePlaySocketUrl);

    attachCellClickListeners();

    websocket.onopen = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} opened!`);
    }

    websocket.onclose = function (event) {
        console.log(`Connection to ${gamePlaySocketUrl} closed!`);
    }

    websocket.onmessage = function (event) {
        console.log(`Received message: {event.data}`);
    }
});

function attachCellClickListeners() {
    document.querySelectorAll(".cell").forEach((cell) => {
        cell.addEventListener("click", (event) => {
            console.log(`Clicked on cell ${event.target.dataset.row}, ${event.target.dataset.col}`);
            websocket.send(`Selection ${event.target.dataset.row} ${event.target.dataset.col}`);
        })
    })
}
