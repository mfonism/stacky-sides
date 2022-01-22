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

  const websocket = new WebSocket(gamePlaySocketUrl);
  websocket.onopen = function (event) {
    gameUI.refreshGameBoard();
    gameUI.attachClickListener(websocket);
  };

  websocket.onclose = function (event) {
    console.log(`Connection to ${gamePlaySocketUrl} closed!`);
  };

  websocket.onmessage = function (event) {
    let message = event.data.toLowerCase();
    if (message.startsWith("board")) {
      let message = message.slice("board".length).trim();
      gameUI.replaceGameBoardData(JSON.parse(message));
    }
  };
});
