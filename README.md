# Salieri - Board Game AI

Board Game Move Recommender written in Rust.

## Games and Algorithms

* Checkers - Common openings db
* Checkers - Minimax algorithm with Alpha Beta Pruning
* Checkers - Monte-Carlo Tree Search

## Config

Environment Variables:

* `ALLOWED_ORIGIN` - CORS allowed origins, default: `http://127.0.0.1:5000`.
* `PORT` - Port server runs on, default: `7878`.
* `MINIMAX_DEPTH` - How many layers the minimax algorithm will search, default: `5`
* `MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`

## Api Endpoints 

### Default Algorithm - Openings DB with Monte Carlo Tree Search

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww
```

### Algorithm - Openings DB 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/openings_db
```

### Algorithm - Minimax 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/minimax
```

### Algorithm - Monte-Carlo Tree Search 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/mcts
```

## API reponse codes

* 200 - Recomended move was able to be generated using the algorithm 
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Checkers State Argument Format

Checkers State is represented with 33 characters. Example:

```
  bbbbbbbbb-bb--b-----wwwwwwwwwwwww
```

The first 32 characters represent the 32 squares on the board. Each character represents the state of the square:

* `w` - White Piece
* `b` - Black Piece
* `-` - Empty Square
* `W` - White King
* `B` - Black King

The last (33rd) character represents the player who's turn it is. The recommended move returned is for that player.

* `w`
* `b`

## Checkers Move Reponse Format

The response is in standard checkers notation:

```
23-19
```

With each number representing a square on the board. The first being the origin and the subsequent ones, the destinations.

Moves are represented with dashes (-) between the numbers. There will only be two in this case

Jumps are represented with crosses (x) between the numbers. There can be more than two in this case.

## TODO

* Extract checkers component
* Add Caching of Calculated Moves
* Add Backgammon
* Add Chess

