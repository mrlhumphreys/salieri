# Shogi API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Shogi Api Endpoints

### Shogi Default Algorithm - Openings DB with Minimax

```
  curl -X POST http://localhost:7878/api/v0/shogi -d "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -"
```

### Shogi Algorithm - Openings DB

```
  curl -X POST http://localhost:7878/api/v0/shogi/openings_db -d "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -"
```

### Shogi Algorithm - Minimax

```
  curl -X POST http://localhost:7878/api/v0/shogi/minimax -d "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -"
```

### Shogi Algorithm - Monte-Carlo Tree Search

```
  curl -X POST http://localhost:7878/api/v0/shogi/mcts -d "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b -"
```

## Shogi State Request Body Format

The Shogi State follows SFEN: [SFEN](https://en.wikipedia.org/wiki/Shogi_notation#SFEN)

## Shogi Move Response Format

The Shogi Move response format follows [Shgoi Notation](https://en.wikipedia.org/wiki/Shogi_notation)
