#!/bin/sh

./target/release/kumandra-node \
        --validator \
        --chain /tmp/chainspec.json \
        --base-path /tmp/5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH \
        --name node-0 \
        --ws-external \
        --rpc-external \
        --rpc-methods=unsafe \
        --rpc-port 9933 \
        --ws-port 9944 \
        --port 30334 \
        --bootnodes /dns4/localhost/tcp/30334/p2p/12D3KooWLJeHApMwpav6Eq5HPcYTtPeNQrWAH8JkDTUdzLn9F7zr \
        --node-key-file /tmp/5D34dL5prEUaGNQtPPZ3yN5Y6BnkfXunKXXz6fo7ZJbLwRRH/p2p_secret \
        --unit-creation-delay 500 \
        --execution Native \
        --rpc-cors=all \
        --no-mdns \
        -lkumandra-party=debug \
        -lkumandra-network=debug \
        -lkumandra-finality=debug \
        -lkumandra-justification=debug \
        -lkumandra-data-store=debug \
        -lkumandra-updater=debug \
        -lkumandra-metrics=debug
