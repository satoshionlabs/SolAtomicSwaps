# Solana Atomic Swap

A secure and decentralized atomic swap implementation on Solana blockchain using the Anchor framework. This project enables trustless token exchanges between parties without requiring intermediaries.

## Overview

Atomic swaps allow two parties to exchange tokens securely without trusting each other or a third party. The implementation uses hash time-locked contracts (HTLCs) to ensure that either both parties receive their tokens or neither does, eliminating counterparty risk.

## Features

- **Trustless Exchange**: Complete token swaps without intermediaries
- **Atomic Execution**: Ensures that either both parties receive their tokens or neither does
- **Time-Locked Contracts**: Automatic refund mechanism if the swap isn't completed within a specified timeframe
- **On-Chain Security**: All operations are secured by Solana's blockchain

## Technical Architecture

The program consists of four main operations:

1. **Initialize**: Set up the pool with a specified fee
2. **Deposit**: Lock tokens in the contract with a secret hash and time lock
3. **Redeem**: Claim tokens by providing the secret key that matches the hash
4. **Refund**: Return tokens to the original sender if the time lock expires

### Smart Contract Structure

- **Pool**: Manages token pools and fees
- **Swap**: Represents an individual atomic swap with its parameters and state

## Prerequisites

- Node.js and npm
- Rust and Cargo
- Solana CLI tools
- Anchor framework

## Setup

1. Clone the repository

```bash
git clone <repository-url>
cd atomicswap_solana
```

2. Install dependencies

```bash
npm install
```

3. Build the program

```bash
anchor build
```

## Testing

Run the test suite to verify the implementation:

```bash
anchor test
```

The tests demonstrate the complete atomic swap workflow:

1. Initialize a new pool
2. Deposit tokens with a time lock and secret hash
3. Redeem tokens using the secret key
4. (Optional) Refund tokens after the time lock expires

## Usage Flow

### Seller (Token Provider)

1. Generate a secret key and its corresponding hash
2. Initialize the swap pool if not already done
3. Deposit tokens with the secret hash, specifying the buyer's address and a time lock

### Buyer (Token Receiver)

1. Verify the swap details on-chain
2. Obtain the secret key from the seller through an off-chain channel
3. Redeem the tokens using the secret key before the time lock expires

### Refund Process

If the buyer doesn't redeem the tokens before the time lock expires, the seller can reclaim their tokens using the refund function.

## License

[MIT](LICENSE)