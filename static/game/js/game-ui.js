class GameUI {
  constructor(playerNum, gameBoardData, isAgainstAI, isGameOver) {
    this.canPlayNext = false;
    this.playerNum = playerNum;
    this.gameBoardData = gameBoardData;
    this.isAgainstAI = isAgainstAI;
    this.isGameOver = isGameOver;
    this.showPlayerStatus();
  }

  showPlayerStatus() {
    if (this.isAgainstAI) {
      // in this version, AI is always player 2
      document.querySelector(".white .description").innerText = "Computer";
    }

    if (this.playerNum === 0) {
      return;
    }

    let className =
      this.playerNum === 1 ? "black" : this.playerNum === 2 ? "white" : "oops";
    document
      .querySelector(`.${className} .description`)
      .appendChild(document.createTextNode("(You)"));
  }

  replaceGameBoardData(gameBoardData) {
    this.gameBoardData = gameBoardData;
    this.refreshGameBoard();
  }

  refreshGameBoard() {
    document.querySelector(".game-card").replaceWith(this.createGameCard());

    if (this.isGameOver) {
      this.canPlayNext = false;
      this.displayResult();
    } else {
      this.canPlayNext = this.checkTurn() === this.playerNum;
    }
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
    document
      .querySelector(".game-card")
      .parentElement.addEventListener("click", (event) => {
        if (
          !event.target.classList.contains("cell") ||
          event.target.classList.contains("disabled") ||
          event.target.classList.contains("colored")
        ) {
          return;
        }

        // check whether it is this player's move
        if (!this.canPlayNext) {
          return;
        }
        this.canPlayNext = false;

        let row = event.target.dataset.row;
        let col = event.target.dataset.col;
        this.gameBoardData[row][col] = this.playerNum;

        this.refreshGameBoard(gameBoardData);

        websocket.send(`Selection ${row} ${col}`);
      });
  }

  notifyGameEnd(winnerNum) {
    this.winnerNum = winnerNum;
    this.loserNum = this.winnerNum === 1 ? 2 : 1;
    this.isGameOver = true;
  }

  displayResult() {
    let resultElt = document.createElement("p");
    resultElt.classList.add("h6", "pt-2");

    let player1OutLoud, player2OutLoud;
    player1OutLoud = this.playerNum === 1 ? "You" : "Player 1";
    player2OutLoud = this.playerNum === 2 ? "You" : "Player 2";
    // for now, if AI is playing, they are player 2
    if (this.isAgainstAI) {
      player2OutLoud = "Computer";
    }

    let whoWon, whoLost;
    if (this.winnerNum === 1) {
      whoWon = player1OutLoud;
      whoLost = player2OutLoud;
    } else if (this.winnerNum === 2) {
      whoWon = player2OutLoud;
      whoLost = player1OutLoud;
    }

    resultElt.textContent =
      this.winnerNum === 0 ? `It's a TIE!` : `${whoWon} won, ${whoLost} lost.`;

    let resultCardElt = document.querySelector(".result-card");
    while (resultCardElt.firstChild) {
      resultCardElt.firstChild.remove();
    }

    resultCardElt.appendChild(resultElt);
  }
}
