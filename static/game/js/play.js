class GameUI {
  constructor(playerNum) {
    this.playerNum = playerNum;
  }

  refreshGameBoard(boardData) {
    document
      .querySelector(".game-card")
      .replaceWith(this.createGameCard(boardData));

    this.canPlayNext = this.checkTurn(boardData) == this.playerNum;
  }

  checkTurn(boardData) {
    // which player's turn is it?
    let res = [0, 0, 0];
    boardData.forEach((row) => {
      row.forEach((cellData) => {
        if (res[cellData] !== 0) {
          res[cellData] += 1;
        }
      });
    });

    return res[1] > res[2] ? 2 : 1;
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
        cardElt.appendChild(this.createGameCell(i, j, cellData, boardData));
      });
    });

    return cardElt;
  }

  createGameCell(i, j, cellData, boardData) {
    let cellElt = document.createElement("div");
    cellElt.classList.add("cell");
    cellElt.setAttribute("data-row", i);
    cellElt.setAttribute("data-col", j);

    if (cellData == 1) {
      cellElt.classList.add("black");
    } else if (cellData == 2) {
      cellElt.classList.add("white");
    } else {
      // prevent from selecting cells that have no selected
      // neighbours on either side of them and aren't on the
      // left or right ends of the board
      if (
        0 < j < boardData[0].length - 1 &&
        boardData[i][j - 1] === 0 &&
        boardData[i][j + 1] === 0
      ) {
        cellElt.classList.add("disabled");
      }
    }

    return cellElt;
  }

  attachClickListener(websocket) {
    let playerNum = this.playerNum;

    // attach on game card's parent because we'll
    // replace the game card on each refresh
    document.querySelector(".game-card").addEventListener("click", (event) => {
      if (
        !event.target.classList.contains("cell") ||
        event.target.classList.contains("disabled")
      ) {
        return;
      }

      // check whether it is this player's move
      if (!this.canPlayNext) {
        return;
      }
      this.canPlayNext = false;

      if (playerNum == 1) {
        event.target.classList.add("black");
      } else if (playerNum == 2) {
        event.target.classList.add("white");
      }

      websocket.send(
        `Selection ${event.target.dataset.row} ${event.target.dataset.col}`
      );
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

  let gameUI = new GameUI(playerNum);
  gameUI.refreshGameBoard(gameBoardData);

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
