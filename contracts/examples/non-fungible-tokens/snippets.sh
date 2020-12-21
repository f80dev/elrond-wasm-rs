#sed 's/\r$//' snippets.sh
#cd contracts/examples/non-fungible-tokens
#source snippets.sh && deploy
#source snippets.sh && infos

USERS="../PEM"
PROJECT="."

ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)
ARGUMENTS=""

#PROXY=https://testnet-api.elrond.com
#CHAINID="T"

PROXY=http://161.97.75.165:7950
CHAINID="local-testnet"


deploy() {
    build

    clear
    erdpy --verbose contract deploy --chain ${CHAINID}  --metadata-payable --project=${PROJECT} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --gas-limit=80000000 --send --outfile="deploy.json"

    TRANSACTION=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['hash']")
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
}



mint(){
  clear
  echo "Minage du token"
  ARGUMENTS="1 0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1 0xaaaaaaaaaa 0xabababab 10"
  erdpy contract call ${ADDRESS} --chain ${CHAINID} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --arguments ${ARGUMENTS} --gas-limit=80000000 --function="mint" --send
}


open(){
  clear
  echo "Ouverture d'un token"
  erdpy contract call ${ADDRESS} --chain ${CHAINID} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --arguments 0 --gas-limit=80000000 --function="open" --send
}


setstate(){
  clear
  echo "changement d'état"
  erdpy contract call ${ADDRESS} --chain ${CHAINID} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --arguments 0 1 --gas-limit=80000000 --function="setstate" --send
}



infos(){
  clear

#  echo ""
#  echo "Contract owner"
#  erdpy contract query ${ADDRESS}  --proxy ${PROXY} --function="contractOwner"
#
  echo ""
  echo "total minted"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="totalMinted"
#
  echo ""
  echo "miner of 1"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="tokenMiner" --arguments 0

#  echo ""
#  echo "TokenOwner sur 0"
#  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="tokenOwner" --arguments 0
  ARGUMENTS="170 255 0x0139472eff6886771a982f3083da5d431f24c29181e63888228dc81ca60d69e1 0x0000000000000000000000000000000000000000000000000000000000000000"
  #ARGUMENTS="170 255 0x0 0x0"
  echo ""
  echo "recuperation des tokens sur ${ADDRESS}"
  erdpy --verbose contract query ${ADDRESS} --proxy ${PROXY}  --function="tokens" --arguments ${ARGUMENTS}
}



buy(){
  clear
  echo "Achat d'un token"
  erdpy contract call ${ADDRESS} --proxy ${PROXY} --recall-nonce --pem=${BOB} --arguments 1 --value 1 --gas-limit=8000000 --function="buy" --send
}


transfert(){
  clear
  echo "Achat d'un token"
  erdpy --verbose contract call ${ADDRESS} --proxy ${PROXY} --recall-nonce --pem=${BOB} --arguments 1 --value 1 --gas-limit=8000000 --function="buy" --send
}


checkDeployment() {
    echo ""
    echo ""
    echo "Vérification du déploiement sur ${PROXY}"
    erdpy tx get --proxy ${PROXY} --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    erdpy account get --proxy ${PROXY} --address=$ADDRESS --omit-fields="['code']"
}


