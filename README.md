# blockchain

export CENTERAL_NODE=127.0.0.1:2001
export BLOCKS_TREE=blocks1
export TREE_DIR=data1
export NODE_ADDR=127.0.0.1:2001
export WALLET_FILE=wallet1.dat


export CENTERAL_NODE=127.0.0.1:2002
export BLOCKS_TREE=blocks2
export TREE_DIR=data2
export NODE_ADDR=127.0.0.1:2002
export WALLET_FILE=wallet2.dat

export CENTERAL_NODE=127.0.0.1:2003
export BLOCKS_TREE=blocks3
export TREE_DIR=data3
export NODE_ADDR=127.0.0.1:2003
export WALLET_FILE=wallet3.dat

export CENTERAL_NODE=127.0.0.1:2004
export BLOCKS_TREE=blocks4
export TREE_DIR=data4
export NODE_ADDR=127.0.0.1:2004
export WALLET_FILE=wallet4.dat

export CENTERAL_NODE=127.0.0.1:2005
export BLOCKS_TREE=blocks5
export TREE_DIR=data5
export NODE_ADDR=127.0.0.1:2005
export WALLET_FILE=wallet5.dat

Create Wallet. This returns WALLET_ADDR
cargo run createwallet

Create blockchain using WALLET_ADDER
cargo run createblockchain WALLET_ADDER


