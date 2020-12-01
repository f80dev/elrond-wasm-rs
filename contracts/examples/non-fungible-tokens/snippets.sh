#sed 's/\r$//' snippets.sh
#cd dev/contracts/examples/non-fungible-tokens

USERS="../PEM"
PROJECT="."
ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)
ARGUMENTS=""
PROXY=https://testnet-api.elrond.com
#PROXY=http://161.97.75.165:7950


deploy() {
    clear
    erdpy contract deploy --project=${PROJECT} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --gas-limit=80000000 --send --outfile="deploy.json"


    TRANSACTION=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address --value=${ADDRESS}
    erdpy data store --key=deployTransaction --value=${TRANSACTION}

    echo ""
    echo "Transaction https://testnet-explorer.elrond.com/transactions/${TRANSACTION}"
    echo "Smart contract address: https://testnet-explorer.elrond.com/address/${ADDRESS}"
}


build(){
  rm ./output/*
  erdpy --verbose contract build
  ls -l ./output
  deploy
}


infos(){
  erdpy account get --proxy ${PROXY} --address ${ADDRESS} --omit-fields=code
}

mint(){
  clear
  echo "Tests sur ${PROXY}"

  echo "Minage du token"
  ARGUMENTS="10 0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1 409600 1000000"
  erdpy --verbose contract call ${ADDRESS} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --arguments ${ARGUMENTS} --gas-limit=8000000 --function="mint" --send

  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="totalMinted"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="tokenOwner" --arguments 1
}

infos(){
  clear
  echo "URI du token 1"
  erdpy contract query ${ADDRESS} --proxy ${PROXY}  --arguments 1 --function="tokenURI"

  echo "Price du token 1"
  erdpy contract query ${ADDRESS} --proxy ${PROXY}  --arguments 1 --function="tokenPrice"
}

buy(){
  clear
  echo "Achat d'un token"
  erdpy --verbose contract call ${ADDRESS} --proxy ${PROXY} --recall-nonce --pem=${BOB} --arguments 1 --value 1 --gas-limit=8000000 --function="buy" --send
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


