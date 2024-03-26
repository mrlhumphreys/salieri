# Salieri - Board Game AI

Board Game Move Recommender written in Rust.

## Games and Algorithms

* Backgammon - Common openings db
* Backgammon - Minimax algorithm with Alpha Beta Pruning
* Backgammon - Monte-Carlo Tree Search
* Checkers - Common openings db
* Checkers - Minimax algorithm with Alpha Beta Pruning
* Checkers - Monte-Carlo Tree Search
* Chess - Common openings db 
* Chess - Minimax algorithm with Alpha Beta Pruning
* Chess - Monte-Carlo Tree Search

## Config

Application Environment Variables:

* `ALLOWED_ORIGIN` - CORS allowed origins, default: `http://127.0.0.1:5173`.
* `PORT` - Port server runs on, default: `7878`.

Game Environment Variables:

* `BACKGAMMON_MINIMAX_DEPTH` - How many layers the backgammon minimax algorithm will search, default: `1`
* `BACKGAMMON_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `BACKGAMMON_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`
* `CHECKERS_MINIMAX_DEPTH` - How many layers the checkers minimax algorithm will search, default: `5`
* `CHECKERS_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `CHECKERS_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`
* `CHESS_MINIMAX_DEPTH` - How many layers the chess minimax algorithm will search, default: `3`
* `CHESS_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `CHESS_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `60`

## TODO

* Add performance tests
