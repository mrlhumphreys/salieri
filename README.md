# Salieri - Board Game AI

Board Game Move Recommender written in Rust.

## Games and Algorithms

* Backgammon - Minimax algorithm with Alpha Beta Pruning
* Checkers - Common openings db
* Checkers - Minimax algorithm with Alpha Beta Pruning
* Checkers - Monte-Carlo Tree Search

## Config

Application Environment Variables:

* `ALLOWED_ORIGIN` - CORS allowed origins, default: `http://127.0.0.1:8080`.
* `PORT` - Port server runs on, default: `7878`.

Game Environment Variables:

* `BACKGAMMON_MINIMAX_DEPTH` - How many layers the backgammon minimax algorithm will search, default: `1`
* `CHECKERS_MINIMAX_DEPTH` - How many layers the checkers minimax algorithm will search, default: `5`
* `BACKGAMMON_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `BACKGAMMON_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`
* `MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`

## API reponse codes

* 200 - Recomended move was able to be generated using the algorithm 
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Backgammon Api Endpoints

### Backgammon Default Algorithm - Minimax 

```
  curl http://localhost:7878/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121
```

### Backgammon Algorithm - Minimax 

```
  curl http://localhost:7878/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121/minimax
```

## Backgammon State Argument Format

The state argument consists of pairs of hexadecimal numbers (0-f), with the first value associated with the first player, and the second value associated with the second player.

e.g. 30 - This indicates that the first player has 3 pieces here and the second player has 0 pieces

The first pair details the number of pieces on the bar for each player. 

The next 24 pairs represent each of the points (and how many pieces on each point for each player) in ascending order from the viewpoint of the first player.

The following pair represents the number of pieces off the board for each player. 

The last two numbers represent the dice roll, with each number being a value on a six-sided die. i.e. 1 to 6.

## Backgammon Move Reponse Format

This is standard backgammon notation consisting of the dice section and the movement section, separated by a colon and a space `: `.

The dice section contains two numbers representing each of the rolled dice, separated by a hyphen `-`.

The second section contains the details of the moves. Typically there will be two moves. But there can be no moves or even 4 moves, depending on the dice rolled and how limited the board is in terms of movement. 

Each move in the list are separated by a space ` `. The from point number comes first, followed by the to point number. They are separated by a slash `/`, 

```
  2-1: 19/21 21/22
```

The above example shows us the following:

  * The rolled dice are 2 and 1.
  * Two moves were made.
  * The first move was from point 19 to point 21.
  * The second move was from point 22 to point 22.

## Checkers Api Endpoints 

### Checkers Default Algorithm - Openings DB with Monte Carlo Tree Search

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww
```

### Checkers Algorithm - Openings DB 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/openings_db
```

### Checkers Algorithm - Minimax 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/minimax
```

### Checkers Algorithm - Monte-Carlo Tree Search 

```
  curl http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww/mcts
```
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
* Extract backgammon component
* Add Caching of Calculated Moves
* Add Chess

