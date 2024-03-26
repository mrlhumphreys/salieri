# Checkers API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Checkers Api Endpoints

### Checkers Default Algorithm - Openings DB with Monte Carlo Tree Search

```
  curl -x POST http://localhost:7878/api/v0/checkers -d "bbbbbbbbb-bb--b-----wwwwwwwwwwwww"
```

### Checkers Algorithm - Openings DB

```
  curl -x POST http://localhost:7878/api/v0/checkers/openings_db -d "W:W16,19,20,21,22,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12"
```

### Checkers Algorithm - Minimax

```
  curl -x POST http://localhost:7878/api/v0/checkers/minimax -d "W:W16,19,20,21,22,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12"
```

### Checkers Algorithm - Monte-Carlo Tree Search

```
  curl -x POST http://localhost:7878/api/v0/checkers/mcts -d "W:W16,19,20,21,22,27,28,29,30,31,32:B1,2,3,4,5,6,7,9,11,12"
```

## Checkers State Argument Format

The state is represented in FEN format. See the FEN tag under [Portable Draughts Notation](https://en.wikipedia.org/wiki/Portable_Draughts_Notation#Tag_Pairs)

## Checkers Move Response Format

The response is in standard checkers notation. See the Movetext section in [Portable Draughts Notation](https://en.wikipedia.org/wiki/Portable_Draughts_Notation#Movetext)

