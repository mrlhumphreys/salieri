#!/usr/bin/env bash

assert_get_response() {
  name=$1
  url=$2
  expected=$3
  result=$(curl -s $url)

  if [[ $result == $expected ]]; then
    echo -e "\033[1m$name\033[0m - \033[1;32mok\033[0m"
  else
    echo -e "\033[1m$name\033[0m - \033[1;31mfail\033[0m"
    echo -e "  expected: \033[1;32m$expected\033[0m"
    echo -e "  got: \033[1;31m$result\033[0m"
  fi
}

assert_post_response() {
  name=$1
  url=$2
  data=$3
  expected=$4
  result=$(curl -X POST $url -d "$data")

  if [[ $result == $expected ]]; then
    echo -e "\033[1m$name\033[0m - \033[1;32mok\033[0m"
  else
    echo -e "\033[1m$name\033[0m - \033[1;31mfail\033[0m"
    echo -e "  expected: \033[1;32m$expected\033[0m"
    echo -e "  got: \033[1;31m$result\033[0m"
  fi
}

# checkers tests
assert_post_response "checkers move" "http://localhost:7878/api/v0/checkers" "bbbbbbbbb-bb--b-----wwwwwwwwwwwww" "24-20"

assert_post_response "checkers openings db" "http://localhost:7878/api/v0/checkers/openings_db" "bbbbbbb-bbbb--b---w-ww-wwwwwwwwww" "22-17"

assert_post_response "checkers minimax" "http://localhost:7878/api/v0/checkers/minimax" "bbbbbbb-bbbb--b---w-ww-wwwwwwwwww" "21-17"

assert_post_response "checkers mcts" "http://localhost:7878/api/v0/checkers/mcts" "bbbbbbb-bbbb--b---w-ww-wwwwwwwwww" "27-23"

# backgammon tests
assert_post_response "backgammon move" "http://localhost:7878/api/v0/backgammon" "0020000000000500300000005005000000030050000000000200121" "2-1: 19/21 21/22"

assert_post_response "backgammon openings db" "http://localhost:7878/api/v0/backgammon/openings_db" "0020000000000500030000005005000000300050000000000200161" "1-6: 13/7 8/7"

assert_post_response "backgammon minimax" "http://localhost:7878/api/v0/backgammon/minimax" "0020000000000500300000005005000000030050000000000200121" "2-1: 19/21 21/22"

assert_post_response "backgammon mcts" "http://localhost:7878/api/v0/backgammon/mcts" "0020000000000500300000005005000000030050000000000200121" "2-1: 19/21 21/22"

# chess tests
assert_post_response "chess move" "http://localhost:7878/api/v0/chess" "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" "d4"

assert_post_response "chess minimax" "http://localhost:7878/api/v0/chess/minimax" "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" "d4"
