ALICE="./wallets/test-wallet2.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
ADDRESS=$(erdpy data load --key=address-devnet)
SC_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${ADDRESS})"

TOKEN_ID="STEGLD-5dfcf7"
TOKEN_ID_HEX="$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"

UNDELEGATE="undelegate"
UNDELEGATE_HEX="$(echo -n ${UNDELEGATE} | xxd -p -u | tr -d '\n')"

NEW_TOKEN_NAME="STEGLD"
NEW_TOKEN_NAME_HEX="$(echo -n ${NEW_TOKEN_NAME} | xxd -p -u | tr -d '\n')"

UNDELEGATE_TOKEN_NAME="USTEGLD"
UNDELEGATE_TOKEN_NAME_HEX="$(echo -n ${NEW_TOKEN_NAME} | xxd -p -u | tr -d '\n')"

INITIAL_SUPPLY=0 #1M * 10**18
DECIMALS=12      #18
TOKEN_CREATION_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"

CALLER_ADDRESS="erd1tuu72tqxs469uz3rd6exjvz4gkn6qlvfg0092df2ejta46c323qsky8mwx"

SEND_TOKENS=""

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --arguments 0 --send --outfile="deploy-devnet.interaction.json" --metadata-payable --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${ALICE} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
        --metadata-payable \
        --gas-limit=100000000 \
        --arguments 0
}

add() {
    read -p "Enter number: " NUMBER
    erdpy --verbose contract call ${ADDRESS} --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${ALICE} --gas-limit=50000000 --function="add" --arguments ${NUMBER} --send
}

issueToken() {
    erdpy --verbose contract call ${ADDRESS} \
        --recall-nonce --pem=${ALICE} \
        --gas-limit=600000000 \
        --value=50000000000000000 \
        --arguments 0x${NEW_TOKEN_NAME_HEX} 0x${NEW_TOKEN_NAME_HEX} --function="issueToken" \
        --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

issueMetaToken() {
    erdpy --verbose contract call ${ADDRESS} \
        --recall-nonce --pem=${ALICE} \
        --gas-limit=600000000 \
        --value=50000000000000000 \
        --arguments 0x${NEW_TOKEN_NAME_HEX} 0x${NEW_TOKEN_NAME_HEX} 0x${DECIMALS} --function="issueLockedToken" \
        --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

issueUndelegateToken() {
    erdpy --verbose contract call ${ADDRESS} \
        --recall-nonce --pem=${ALICE} \
        --gas-limit=600000000 \
        --value=50000000000000000 \
        --arguments 0x${UNDELEGATE_TOKEN_NAME_HEX} 0x${UNDELEGATE_TOKEN_NAME_HEX} 0x${DECIMALS} --function="issueUndelegatedToken" \
        --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${ALICE} \
        --gas-limit=100000000 \
        --function="setLocalRoles"
}

setLocalRolesLockedToken() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${ALICE} \
        --gas-limit=100000000 \
        --function="setLocalRolesLockedToken"
}

stake() {
    erdpy --verbose contract call ${ADDRESS} \
        --recall-nonce --pem=${ALICE} \
        --gas-limit=600000000 \
        --value=2000000000000000000 \
        --arguments 0x0000000000000000000100000000000000000000000000000000000002ffffff --function="stake" \
        --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

SEND_AMOUNT=0de0b6b3a7640000 #1 * 10**18
undelegate() {
    erdpy --verbose tx new --receiver ${CALLER_ADDRESS} --recall-nonce --pem=${ALICE} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=600000000 \
        --data="ESDTNFTTransfer@${TOKEN_ID_HEX}@01@${SEND_AMOUNT}@${SC_ADDRESS_ONLY_HEX}@${UNDELEGATE_HEX}@0000000000000000000100000000000000000000000000000000000002ffffff"
}

sendTokens() {
    erdpy --verbose tx new --receiver ${ADDRESS} \
        --recall-nonce --pem=${ALICE} \
        --gas-limit=10000000 \
        --data="ESDTTransfer@${TOKEN_ID_HEX}@64" \
        --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setToken() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${ALICE} \
        --gas-limit=100000000 \
        --function="setToken" \
        --arguments 0x${TOKEN_ID_HEX}
}

getSuccedeed() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getSuccedeed"
}

getSuccedeed() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getSuccedeed"
}

getMetaToken() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getMetaToken"
}
