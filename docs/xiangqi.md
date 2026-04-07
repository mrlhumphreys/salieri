# Xiangqi API

## API response codes

* 200 - Recomended move was able to be generated using the algorithm
* 404 - No move was able to be generated using the algorithm. Possible causes include invalid game state or no move found in lookup (e.g. openings db)

## Xiangqi Api Endpoints

### Xiangqi Default Algorithm - Openings DB with Minimax

```
  curl -X POST http://localhost:7878/api/v0/xiangqi -d "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0" 
```

### Xiangqi Algorithm - Openings DB

```
  curl -X POST http://localhost:7878/api/v0/xiangqi/openings_db -d "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0"
```

### Xiangqi Algorithm - Minimax

```
  curl -X POST http://localhost:7878/api/v0/xiangqi/minimax -d "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0"
```

### Xiangqi Algorithm - Monte-Carlo Tree Search

```
  curl -X POST http://localhost:7878/api/v0/xiangqi/mcts -d "rheakaehr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RHEAKAEHR w - - 0 0"
```

## Xiangqi State Request Body Format

The Xiangqi State follows [FEN for Xiangqi](https://www.wxf-xiangqi.org/images/computer-xiangqi/fen-for-xiangqi-chinese-chess.pdf)

## Xiangqi Move Response Format

The Xiangqi Move response format follows [Xiangqi Notation System 2](https://en.wikipedia.org/wiki/Xiangqi#Notation)
