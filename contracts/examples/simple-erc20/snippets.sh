#sed 's/\r$//' snippets.sh >> snippets2.sh
#cd contracts/examples/simple-erc20
#source snippets.sh && deploy

USERS="../PEM"
PROJECT="."
ALICE="${USERS}/alice.pem"
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)
ARGUMENTS="1000 4096"

PROXY=https://testnet-api.elrond.com
CHAINID="T"

#PROXY=http://161.97.75.165:7950
#CHAINID="local-testnet"


deploy() {
    erdpy contract deploy --chain ${CHAINID} --proxy ${PROXY} --project=${PROJECT} --arguments ${ARGUMENTS} --recall-nonce --pem=${ALICE} --gas-limit=90000000 --send --outfile="deploy.json"

    TRANSACTION=$(erdpy data parse --file="deploy.json" --expression="data['result']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address --value=${ADDRESS}
    erdpy data store --key=deployTransaction --value=${TRANSACTION}

    echo ""
    echo "Transaction https://testnet-explorer.elrond.com/transactions/${TRANSACTION}"
    echo "Transaction ${PROXY}/transaction/${TRANSACTION}"

    echo "Smart contract address: https://testnet-explorer.elrond.com/address/${ADDRESS}"
    echo "Smart contract address: ${PROXY}/address/${ADDRESS}"
}


build(){
  rm ./output/*
  erdpy --verbose contract build
  ls -l ./output
  deploy
}


checkDeployment() {
    erdpy tx get --proxy ${PROXY} --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    erdpy account get --proxy ${PROXY}  --address=$ADDRESS --omit-fields="['code']"
}

info() {
  echo "Contrat ${ADDRESS}"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="totalSupply"
}

transfer() {
  echo "Contrat ${ADDRESS}"
  erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --arguments "erd1kyaqzaprcdnv4luvanah0gfxzzsnpaygsy6pytrexll2urtd05ts9vegu7" --gas-limit=5000000 --function="tranfer" --send
}

balance() {
  echo "Contrat ${ADDRESS}"
  erdpy --verbose contract query ${ADDRESS} --proxy ${PROXY} --arguments "0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1" --function="balanceOf"
}

