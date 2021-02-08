#sed 's/\r$//' snippets.sh
#cd contracts/examples/non-fungible-tokens
#source snippets.sh && deploy
#source snippets.sh && infos

USERS="../PEM"
PROJECT="."

ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"
DAN="${USERS}/dan.pem"
CAROL="${USERS}/carol.pem"
ADDRESS=$(erdpy data load --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction)
ARGUMENTS=""

#PROXY=https://testnet-api.elrond.com
#CHAINID="T"

PROXY=https://devnet-api.elrond.com
CHAINID="D"

#PROXY=http://161.97.75.165:7950
#CHAINID="local-testnet"


deploy() {
    build

    clear
    erdpy contract deploy --chain ${CHAINID}  --metadata-payable --project=${PROJECT} --proxy ${PROXY} --recall-nonce --pem=${ALICE} --gas-limit=120000000 --send --outfile="deploy.json"

    TRANSACTION=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address --value=${ADDRESS}
    erdpy data store --key=deployTransaction --value=${TRANSACTION}

    echo "si besoin https://testnet-wallet.elrond.com"
    echo "Transaction https://devnet-explorer.elrond.com/transactions/${TRANSACTION}"
    echo "Transaction ${PROXY}/transaction/${TRANSACTION}"
    echo "Smart contract address: https://devnet-explorer.elrond.com/address/${ADDRESS}"
    echo "Smart contract address: ${PROXY}/address/${ADDRESS}"
    echo "contract deployed ${ADDRESS}"
}


build(){
  rm ./output/*
  erdpy --verbose contract build
  ls -l ./output
}



mint(){
  clear
  echo "Minage du token"
  #mint(count: u64, new_token_owner: Address, new_token_uri: &Vec<u8>,secret: &Vec<u8>, new_token_price: BigUint)
  ARGUMENTS="1 0xaaaaaaaaaa 0xabababab 0xffffff 0xfffff0 0xffffff 0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1 50"
  erdpy contract call ${ADDRESS} --chain ${CHAINID} --proxy ${PROXY} --recall-nonce --pem=${BOB} --arguments ${ARGUMENTS} --gas-limit=120000000 --function="mint" --send
  echo "Transaction ${PROXY}/transaction/${TRANSACTION}"
}


buy(){
  clear
  echo "Achat d'un token"
  # buy(&self, #[payment] payment: BigUint, token_id: u64,seller:Address)
  ARGUMENTS="0 0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1"
  #dan achete et bob doit recevoir des fond
  erdpy contract call ${ADDRESS} --proxy ${PROXY} --chain ${CHAINID} --recall-nonce --pem=${CAROL} --arguments ${ARGUMENTS} --value 100000000 --gas-limit=80000000 --function="buy" --send
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


hex_address(){
  echo "alice="
  erdpy wallet pem-address-hex ${ALICE}

  echo "bob="
  erdpy wallet pem-address-hex ${BOB}

  echo "dan="
  erdpy wallet pem-address-hex ${DAN}

  echo "carol="
  erdpy wallet pem-address-hex ${CAROL}
}

balances(){
  echo "balances"

  echo ""
  echo "alice="
  erdpy account get --address "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th" --balance

  echo ""
  echo "bob="
  erdpy account get --address "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx" --balance

  echo ""
  echo "dan="
  erdpy account get --address "erd1kyaqzaprcdnv4luvanah0gfxzzsnpaygsy6pytrexll2urtd05ts9vegu7" --balance

  echo ""
  echo "carol="
  erdpy account get --address "erd1k2s324ww2g0yj38qn2ch2jwctdy8mnfxep94q9arncc6xecg3xaq6mjse8" --balance

}


infos(){
  clear

  echo ""
  echo "contract ${ADDRESS}"

  echo ""
  echo "Contract owner"
  erdpy contract query ${ADDRESS}  --proxy ${PROXY} --function="contractOwner"

  echo ""
  echo "total minted"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="totalMinted"

  echo ""
  echo "miner of 1"
  erdpy contract query ${ADDRESS} --proxy ${PROXY} --function="tokenMiner" --arguments 0

  ARGUMENTS="0x0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1 0x0000000000000000000000000000000000000000000000000000000000000000 0x0000000000000000000000000000000000000000000000000000000000000000"
  echo ""
  echo "recuperation des tokens sur ${ADDRESS}"
  erdpy --verbose contract query ${ADDRESS} --proxy ${PROXY}  --function="tokens" --arguments ${ARGUMENTS}
}





transfert(){
  clear
  echo "Achat d'un token"
  erdpy --verbose contract call ${ADDRESS} --chain ${CHAINID} --proxy ${PROXY} --recall-nonce --pem=${BOB} --arguments 1 --value 1 --gas-limit=8000000 --function="buy" --send
}


checkDeployment() {
    echo ""
    echo "Vérification du déploiement sur ${PROXY}"
    erdpy tx get --proxy ${PROXY} --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    erdpy account get --proxy ${PROXY} --address=$ADDRESS --omit-fields="['code']"
}


_test() {
  deploy
  mint
  infos
}


