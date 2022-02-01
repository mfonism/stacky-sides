# Possible Improvements

## High Priority Improvements

* __Make the AI smarter__

  At the moment the AI always selects the next available cell on the rightmost column of the board. This is more AD than AI.

  + One way I can improve upon this is to check whether the last move the human player made is leading up to one or more connections of their previous plays, and make the AI block the best of these future connections.

  + Another way is to use the minimax algorithm to create an unbeatable AI.

* __Allow player to choose whether they want to play or not__

  At the moment the creator is immediately added to the game as _player 1_ and the next person to join the game is added as _player 2_.

  This is too eager!

  I'm thinking to let the creator choose whether they want to join as a player (in the game creation view), and to ask everyone else who joins with the link whether they'd like to join is as a player or not.

* __Make the AI a little human-relatable__

  At the moment the AI responds immediately with a selection after the human player makes theirs. This doesn't make for good UX..., and hardly gives the human opponent time to breathe, LOL.

  I'm thinking to introduce a random (bounded) delay between when the human player's selection is broadcast to all listeners, and when the AI's selection is made. I think this will make the game's UX smoother.

* __Highlight the winning balls at the end of a game__

  The information required to implement this is already captured by the function that calculates whether a given move is a winning move.

* __Add the ability to play multiple rounds in a game__

  This will need restructuring of the models, so that a `Game` has many `Round`s, and a `Round` has many `Board`.

  Also, player order should be rotated on successive `Round`s in the same `Game`.

  I've sketched out how I'd restructure the models in the last section of this doc.

## Low Priority Improvements

* Make the user's _name_ light up when they join the game.

* Make the user's _name_ light up differently (or change the user's icon) when it's their turn to play.

* Introduce cues like _"Player 1 is thinking"_ to give people (especially those who join in the middle of the game) a clue as to the current state of things.


## Nice to Have's

* The ability to rewind game state and/or relive from start a game that's already in progress (or is over)


# How I'd Restructure the Models

__Note:__ I'm using an abstract representation below. This is not Rust!

```
Game
    uuid: Uuid
    created_at: Timestamp
    ended_at: Timestamp
    is_against_ai: Boolean

    # ---
    # Relationships
    # ---
    winner: ManyToOne<Player>
    players: ManyToMany<Player>
        // but only the first two players matter in a regular game
        // so we can stop adding after that.
        // in a game against the computer
        // we can stop adding after the first player.

    # ---
    # Reverse relationships
    # ---

    rounds: OneToMany<Round>


Round
    id: Integer
    created_at: Timestamp
    ended_at: Timestamp | null

    # ---
    # Relationships
    # ---

    game: ManyToOne<Game>
    player1: ManyToOne<Player>
    player2: ManyToOne<Player>
    winner: ManyToOne<Player>

    # ---
    # Reverse relationships
    # ---

    boards: OneToMany<Board>


Board
    id: Integer
    created_at: Timestamp
    state: Json

    # ---
    # Relationships
    # ---

    game_round: ManyToOne<Round>


Player
    uuid: Uuid
    created_at: Timestamp

    # ---
    # Reverse relationships
    # ---
    games_played: ManyToMany<Game>
    games_won: OneToMany<Game>
