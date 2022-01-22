window.addEventListener("DOMContentLoaded", (event) => {
  const gamePlaySocketUrl = JSON.parse(
    document.getElementById("gameWsUrl").textContent
  );
  const gameBoardData = JSON.parse(
    document.getElementById("gameBoardData").textContent
  );
  const playerNum = JSON.parse(
    document.getElementById("playerNum").textContent
  );

  let gameUI = new GameUI(playerNum, gameBoardData);
  gameUI.refreshGameBoard();

  const websocket = new WebSocket(gamePlaySocketUrl);
  gameUI.attachClickListener(websocket);

  websocket.onopen = function (event) {
    console.log(`Connection to ${gamePlaySocketUrl} opened!`);
  };

  websocket.onclose = function (event) {
    console.log(`Connection to ${gamePlaySocketUrl} closed!`);
  };

  websocket.onmessage = function (event) {
    console.log(`Received message: {event.data}`);
  };
});
