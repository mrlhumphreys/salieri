# Checkers API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm 
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

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

## Checkers Move Response Format

The response is in standard checkers notation:

```
23-19
```

With each number representing a square on the board. The first being the origin and the subsequent ones, the destinations.

Moves are represented with dashes (-) between the numbers. There will only be two in this case

Jumps are represented with crosses (x) between the numbers. There can be more than two in this case.

