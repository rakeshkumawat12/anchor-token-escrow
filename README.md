# Non-Custodial Escrow on Solana (Anchor)

A trustless escrow smart contract built using Anchor for the Solana blockchain.

## Features

- Non-custodial: Tokens are held in a PDA-owned account.
- Trustless exchange: Escrow logic ensures fair swaps between parties.
- Built with Anchor framework (Rust).

## How It Works

1. **Initialize**: Seller locks token X into a PDA-controlled account.
2. **Exchange**: Buyer sends token Y.
3. **Program validates** the conditions and completes the swap.

## Program Structure

- `initialize` – Sets up the escrow state and creates a token account.
- `exchange` and `cancel` instructions.

## Usage

```bash
anchor build
anchor test
```