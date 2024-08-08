#!/usr/bin/env bash
nohup cargo run --release --package smart_wallet_following --bin smart_wallet_following > output.log 2> error.log &