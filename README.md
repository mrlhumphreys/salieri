# Salieri - Board Game AI

Board Game Move Recommender written in Rust.

Implements the Minimax algorithm with Alpha Beta Pruning.

## Usage

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww
```

Checkers State is represented with 33 characters.

The first 32 characters represent the 32 squares on the board. Each character represents the state of the square:

* w - White Piece
* b - Black Piece
* - - Empty Square
* W - White King
* B - Black King

The last (33rd) character represents the player who's turn it is. The recommended move returned is for that player.

* w
* b

The response is in standard checkers notation:

```
23-19
```

With each number representing a square on the board. The first being the origin and the subsequent ones, the destinations.

Moves are represented with dashes (-) between the numbers. There will only be two in this case

Jumps are represented with crosses (x) between the numbers. There can be more than two in this case.

## TODO

* Improve performance of move calculation
* Tweak Static Evaluation Function
* Add Caching of Calculated Moves
* Implement Monte Carlo Tree Search
* Other Games


