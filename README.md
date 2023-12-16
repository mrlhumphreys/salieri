# Salieri - Board Game AI

Board Game Move Recommender written in Rust.

## Games and Algorithms

* Backgammon - Common openings db
* Backgammon - Minimax algorithm with Alpha Beta Pruning
* Backgammon - Monte-Carlo Tree Search
* Checkers - Common openings db
* Checkers - Minimax algorithm with Alpha Beta Pruning
* Checkers - Monte-Carlo Tree Search
* Chess - Minimax algorithm with Alpha Beta Pruning

## Config

Application Environment Variables:

* `ALLOWED_ORIGIN` - CORS allowed origins, default: `http://127.0.0.1:8080`.
* `PORT` - Port server runs on, default: `7878`.

Game Environment Variables:

* `BACKGAMMON_MINIMAX_DEPTH` - How many layers the backgammon minimax algorithm will search, default: `1`
* `BACKGAMMON_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `BACKGAMMON_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`
* `CHECKERS_MINIMAX_DEPTH` - How many layers the checkers minimax algorithm will search, default: `5`
* `CHECKERS_MCTS_SIMULATION_COUNT` - How many simulations the Monte Carlo Tree Search algorithm will search, default: `120`
* `CHECKERS_MCTS_SIMULATION_DEPTH` - How many moves deep in the Monte Carlo Tree Search algorithm will search for each simulation, default: `40`
* `CHESS_MINIMAX_DEPTH` - How many layers the chess minimax algorithm will search, default: `3`

## TODO

* Chess
    * Performance - Move piece into square
    * Performance - Move to Mailbox board implmentation
* Add performance tests
