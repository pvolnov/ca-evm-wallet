
# Chain abstraction EVM wallet

## Concept

The "Chain Signature" feature in smart contracts revolutionizes transaction signing and management. It allows smart contracts to sign transactions using a private key, which is distributed and securely held by a group of validators. The "CA EVM Wallet" leverages this technology, offering a wallet for any EVM-compatible network. Unlike traditional wallets, it permits users to give third parties full or limited rights to sign transactions on their behalf, enhancing security and flexibility. Trusted apps can verify arguments and sign transactions for users, providing a secure and versatile transaction environment.

Key use cases of this approach include:
- Account sales: Facilitating the secure transfer EVM account ownership.
- Seed phrase modification: Allowing users to change their seed phrases for enhanced security.
- Native token swaps across networks
- Delayed and limited orders on DEXs
- Subscriptions and regular payments
- NFT trading: Streamlining the process of buying, selling, and exchanging Non-Fungible Tokens at special time / conditions


## Introduction

This smart contract is designed to implement an Ethereum wallet interface on the NEAR blockchain. Its primary functionalities include the creation of Ethereum wallets within the contract with various paths (addresses), transferring access to these wallets, and signing Ethereum transactions.

## Core Functions

### 1. Create Wallet (`create_wallet`)
- **Description**: Creates a new Ethereum wallet.
- **NEAR CLI Command**:

```bash
near call [contract] create_wallet --accountId [your NEAR account]
```

- **Example**:
Creates a wallet with a unique path and grants access to the creator.

### 2. Sign Transaction (`sign`)
- **Description**: Signs a transaction with the given payload.
- **NEAR CLI Command**:

```bash
near call [contract] sign '{"payload": "[payload]", "wallet_id": "[wallet ID]"}' --accountId [your NEAR account]
```

- **Example**:
Signs a provided payload for a specified wallet.

### 3. Grant Access (`grant_assess`)
- **Description**: Grants access to the wallet to another NEAR account.
- **NEAR CLI Command**:


```bash
near call [contract] grant_assess '{"wallet_id": "[wallet ID]", "account_id": "[NEAR account]"}' --accountId [your NEAR account]
```

- **Example**:
Grants a user access to a specific wallet (other dapp) or sell account

### 4. Revoke Access (`revoke_assess`)
- **Description**: Revokes access to the wallet from a specified NEAR account.
- **NEAR CLI Command**:


```bash
near call [contract] revoke_assess '{"wallet_id": "[wallet ID]", "account_id": "[NEAR account]"}' --accountId [your NEAR account]
```

- **Example**:
Revokes a user's access to a specific wallet.

### 5. Ethereum Transfer (`eth_transfer_call`)
- **Description**: Initiates an Ethereum transaction.
- **NEAR CLI Command**:

```bash
near call [contract] eth_transfer_call '{"wallet_id": "[wallet ID]", "recipient": "[recipient]", "amount": [amount], "nonce": [nonce], "gas_price": [gas price], "gas_limit": [gas limit], "data": "[data]"}' --accountId [your NEAR account]
```


## Security and Future Perspectives

### Security
- **Authentication**: Each signature requires validation by NEAR Validators
- **Private Key**: All private keys storing distributed on the MPC node network
- **Access Management**: The `grant_assess` and `revoke_assess` functions allow for control over wallet access.


## Next steps

For a complete wallet, it lacks implementation of some key methods, which was impossible to do due to weak CA documentation. They will be added in the future during the project development

- get_public_key(path) - method to retrieve public key and eth address by path
- check the network on which the transaction will be executed
- possibility to give access to signatures of certain methods and sum limits

This smart contract on the NEAR blockchain provides a secure and efficient way to manage Ethereum wallets and transactions, paving the way for broader blockchain interoperability and enhanced user experience in the crypto ecosystem.
