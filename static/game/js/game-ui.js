class GameUI {
  constructor(playerNum, gameBoardData) {
    this.canPlayNext = false;
    this.playerNum = playerNum;
    this.gameBoardData = gameBoardData;
  }

  replaceGameBoardData(gameBoardData) {
    this.gameBoardData = gameBoardData;
    this.refreshGameBoard();
  }

  refreshGameBoard() {
    document.querySelector(".game-card").replaceWith(this.createGameCard());

    console.log('PN ', this.playerNum);
    console.log('CH ', this.checkTurn());
    this.canPlayNext = this.checkTurn() === this.playerNum;
    console.log('CPN', this.canPlayNext);
  }

  checkTurn() {
    // which player's turn is it?
    let res = [0, 0, 0];

    for (let row of this.gameBoardData) {
      for (let cellData of row) {
        if (cellData !== 0) {
          res[cellData] += 1;
        }
      }
    }
    return res[1] > res[2] ? 2 : 1;
  }

  createGameCard() {
    let cardElt = document.createElement("div");
    cardElt.classList.add(
      "game-card",
      "mx-auto",
      "d-flex",
      "justify-content-center",
      "align-content-center",
      "flex-wrap"
    );

    this.gameBoardData.forEach((row, i) => {
      row.forEach((_cellData, j) => {
        cardElt.appendChild(this.createGameCell(i, j));
      });
    });

    return cardElt;
  }

  createGameCell(i, j) {
    let cellElt = document.createElement("div");
    cellElt.classList.add("cell");
    cellElt.setAttribute("data-row", i);
    cellElt.setAttribute("data-col", j);

    if (this.gameBoardData[i][j] == 1) {
      cellElt.classList.add("colored", "black");
    } else if (this.gameBoardData[i][j] == 2) {
      cellElt.classList.add("colored", "white");
    } else {
      // prevent from selecting cells that have no selected
      // neighbours on either side of them and aren't on the
      // left or right ends of the board
      if (
        0 < j < this.gameBoardData[0].length - 1 &&
        this.gameBoardData[i][j - 1] === 0 &&
        this.gameBoardData[i][j + 1] === 0
      ) {
        cellElt.classList.add("disabled");
      }
    }

    return cellElt;
  }

  attachClickListener(websocket) {
    // attach listener on game card's parent
    // (and not on game card itself) because we'll
    // replace the game card on each refresh
    document.querySelector(".game-card").parentElement.addEventListener("click", (event) => {
      console.log('entering')
      if (
        !event.target.classList.contains("cell") ||
        event.target.classList.contains("disabled") ||
        event.target.classList.contains("colored")
      ) {
        console.log('returing 1');
        return;
      }

      // check whether it is this player's move
      if (!this.canPlayNext) {
        console.log('returning 2')
        return;
      }
      this.canPlayNext = false;

      let row = event.target.dataset.row;
      let col = event.target.dataset.col;
      this.gameBoardData[row][col] = this.playerNum;

      this.refreshGameBoard(gameBoardData);
      console.log(`Selection ${row} ${col}`);
      websocket.send(`Selection ${row} ${col}`);
    });
  }
}
