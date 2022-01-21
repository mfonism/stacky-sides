class GameUI {
  constructor() {}

  refreshGameBoard(boardData) {
    document
      .querySelector(".game-card")
      .replaceWith(this.createGameCard(boardData));
  }

  createGameCard(boardData) {
    let cardElt = document.createElement("div");
    cardElt.classList.add(
      "game-card",
      "mx-auto",
      "d-flex",
      "justify-content-center",
      "align-content-center",
      "flex-wrap"
    );

    boardData.forEach((row, i) => {
      row.forEach((cellData, j) => {
        cardElt.appendChild(this.createGameCell(i, j, cellData));
      });
    });

    return cardElt;
  }

  createGameCell(i, j, cellData) {
    let cellElt = document.createElement("div");
    cellElt.classList.add("cell");
    cellElt.setAttribute("data-row", i);
    cellElt.setAttribute("data-col", j);

    if (cellData == 1) {
      cellElt.classList.add("black");
    } else if (cellData == 2) {
      cellElt.classList.add("white");
    }

    return cellElt;
  }

  attachClickListeners(websocket, playerNum) {
    document.querySelectorAll(".cell").forEach((cell) => {
      cell.addEventListener("click", (event) => {
        if (playerNum == 1) {
          event.target.classList.add("black");
        } else if (playerNum == 2) {
          event.target.classList.add("white");
        }

        websocket.send(
          `Selection ${event.target.dataset.row} ${event.target.dataset.col}`
        );
      });
    });
  }
}

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

  let gameUI = new GameUI();
  gameUI.refreshGameBoard(gameBoardData);

  const websocket = new WebSocket(gamePlaySocketUrl);
  gameUI.attachClickListeners(websocket, playerNum);

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
