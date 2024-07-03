#!/bin/bash

# 固定程序ID
# Define accounts and program ID
PROGRAM_ID="3N8i3UJNMBw5eyoQEKf8Qo14xh3yLKuXpK1jWVde8tFX"
PAYER_KEYPAIR=$PAYER_KEYPAIR

# Get the latest blockhash
RECENT_BLOCKHASH=$(solana block | grep -m 1 -o 'Blockhash: [a-zA-Z0-9]*' | awk '{print $2}')

# Create the instruction
solana transfer --from $PAYER_KEYPAIR $PROGRAM_ID 0.001 --allow-unfunded-recipient --blockhash $RECENT_BLOCKHASH --sign-only --output json-compact > transaction.json

# Get the account public key from the keypair
ACCOUNT=$(solana-keygen pubkey $PAYER_KEYPAIR)

# Add the custom instruction to the transaction
solana transaction create $PROGRAM_ID --data $(xxd -p instruction_data.bin) --from $ACCOUNT --blockhash $RECENT_BLOCKHASH --output json-compact > transaction_with_instruction.json


# Send the transaction
solana transaction send transaction_with_instruction.json --from $PAYER_KEYPAIR
