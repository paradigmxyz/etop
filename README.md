
# etop

[![Rust](https://github.com/paradigmxyz/cryo/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/paradigmxyz/cryo/actions/workflows/build_and_test.yml) [![Telegram Chat](https://img.shields.io/badge/Telegram-join_chat-blue.svg)](https://t.me/paradigm_data)

like `htop` for Ethereum

`etop` offers effortless visibility into what's happening on chain


## Contents
1. [Installation](#installation)
2. [Example Usage](#example-usage)


## Installation

#### Install From Crates.io
`cargo install etop-cli`


#### Install From Source
```bash
git clone https://github.com/paradigmxyz/etop
cd etop
cargo install --path crates/etop-cli
```

## Example Usage

1. Show summary of ERC20 Transfers
`etop erc20_transfers_by_erc20`

2. Show summary of transactions per contraction
`etop transactions_by_to_address`

3. Show 10 most recent blocks
`etop blocks -w 10`

## Keyboard Shortcuts
- `]` increment block
- `[` decrement block
- `}` increment window
- `{` decrement window
- `l` snap to live data
- `q` quit

## Parameters
- `--window`: data window size in number of blocks
- `--block`: block to start at, by default starts at latest block
- `--rpc`: rpc endpoint url

