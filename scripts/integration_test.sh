#!/usr/bin/env bash

assert_response () { 
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

assert_response "backgammon move" "http://localhost:7878/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121" "2-1: 19/21 21/22"

assert_response "backgammon minimax" "http://localhost:7878/api/v0/backgammon/0020000000000500300000005005000000030050000000000200121/minimax" "2-1: 19/21 21/22"

assert_response "checkers move" "http://localhost:7878/api/v0/checkers/bbbbbbbbb-bb--b-----wwwwwwwwwwwww" "24-20"

assert_response "checkers openings db" "http://localhost:7878/api/v0/checkers/bbbbbbb-bbbb--b---w-ww-wwwwwwwwww/openings_db" "22-17"

assert_response "checkers minimax" "http://localhost:7878/api/v0/checkers/bbbbbbb-bbbb--b---w-ww-wwwwwwwwww/minimax" "21-17"

assert_response "checkers mcts" "http://localhost:7878/api/v0/checkers/bbbbbbb-bbbb--b---w-ww-wwwwwwwwww/mcts" "27-23"

