#!/bin/bash

# create tag 
awk '/version/ {gsub(/"/, "", $3); print $3}' Cargo.toml | xargs -I {} git tag -a v{} -m 'Version {}'

# push tags 
git push origin main
git push --tags

