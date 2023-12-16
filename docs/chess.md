# Chess API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm 
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Chess Api Endpoints 

### Chess Default Algorithm 

```
  curl -X POST http://localhost:7878/api/v0/chess -d "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
```

### Chess Algorithm - Minimax 

```
  curl -X POST http://localhost:7878/api/v0/chess/minimax -d "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
```

## Chess State Request Body Format

The Chess State follows FEN: [Forsyth-Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)

## Chess Move Response Format

The Chess Move response format follows [Algebraic Notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess))
