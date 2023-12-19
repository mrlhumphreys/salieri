# Backgammon API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm 
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Backgammon Api Endpoints

### Backgammon Default Algorithm - Openings DB with Monte Carlo Tree Search 

```
  curl -X POST http://localhost:7878/api/v0/backgammon -d "0020000000000500300000005005000000030050000000000200121"
```

### Backgammon Algorithm - Openings DB

```
  curl -X POST http://localhost:7878/api/v0/backgammon/openings_db -d "0020000000000500300000005005000000030050000000000200121"
```

### Backgammon Algorithm - Minimax 

```
  curl -X POST http://localhost:7878/api/v0/backgammon/minimax -d 0020000000000500300000005005000000030050000000000200121
```

### Backgammon Algorithm - Monte-Carlo Tree Search

```
  curl -X POST http://localhost:7878/api/v0/backgammon/mcts -d "0020000000000500300000005005000000030050000000000200121"
```

## Backgammon State Argument Format

The state argument consists of pairs of hexadecimal numbers (0-f), with the first value associated with the first player, and the second value associated with the second player.

e.g. 30 - This indicates that the first player has 3 pieces here and the second player has 0 pieces

The first pair details the number of pieces on the bar for each player. 

The next 24 pairs represent each of the points (and how many pieces on each point for each player) in ascending order from the viewpoint of the first player.

The following pair represents the number of pieces off the board for each player. 

The last two numbers represent the dice roll, with each number being a value on a six-sided die. i.e. 1 to 6.

## Backgammon Move Response Format

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
