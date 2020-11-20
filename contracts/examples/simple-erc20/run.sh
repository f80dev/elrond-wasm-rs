#!/usr/bin/env bash

erdpy contract build
erdpy contract deploy --project=. --proxy http://161.97.75.165:7950 --arguments 10000 4096 --recall-nonce --pem=../PEM/alice.pem --gas-limit=50000000 --send --outfile="deploy.json"
ADDRESS=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['address']")
erdpy contract query ${ADDRESS} --function="totalSupply"