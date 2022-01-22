window.addEventListener("DOMContentLoaded", (event) => {
  const gamePlaySocketUrl = JSON.parse(
    document.getElementById("gameWsUrl").textContent
  );

  const websocket = new WebSocket(gamePlaySocketUrl);
  websocket.onopen = function (event) {
    const gameBoardData = JSON.parse(
      document.getElementById("gameBoardData").textContent
    );
    const playerNum = JSON.parse(
      document.getElementById("playerNum").textContent
    );
    let gameUI = new GameUI(playerNum, gameBoardData);
    gameUI.refreshGameBoard();
    gameUI.attachClickListener(websocket);
  };

  websocket.onclose = function (event) {
    console.log(`Connection to ${gamePlaySocketUrl} closed!`);
  };

  websocket.onmessage = function (event) {
    console.log(`Received message: ${event.data}`);
  };
});
