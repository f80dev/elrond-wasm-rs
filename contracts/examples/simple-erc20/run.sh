#!/usr/bin/env bash

erdpy contract build
erdpy --verbose contract deploy --project=. --proxy https://testnet-api.elrond.com --arguments 10000 4096 --recall-nonce --pem=../PEM/alice.pem --gas-limit=50000000 --send --outfile="deploy.json"