# Go API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Go Api Endpoints

### Go Default Algorithm - Minimax 

```
  curl -x POST http://localhost:7878/api/v0/go -d ""
```

### Go Algorithm - Minimax

```
  curl -x POST http://localhost:7878/api/v0/go/minimax -d ""
```

## Go State Argument Format

The state is represented as a Smart Game Format Node. See the documentation here: [Smart Game Format](https://www.red-bean.com/sgf/)

## Go Move Response Format

The move is represented by two letters representing the co-ordinates. See the documentation here: [Smart Game Format Properties](https://www.red-bean.com/sgf/go.html)

