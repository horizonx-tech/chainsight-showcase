#!/bin/bash
# init
dfx canister --network ic call shib_eth_1_indexer init_in '(variant { "Production" }, record {
                refueling_interval = 86400: nat64;
                vault_intial_supply = 500000000000: nat;
                indexer = record {
                    initial_supply = 0: nat;
                    refueling_amount = 500000000000: nat;
                    refueling_threshold = 250000000000: nat;
                };
                db = record {
                    initial_supply = 750000000000: nat;
                    refueling_amount = 500000000000: nat;
                    refueling_threshold = 250000000000: nat;
                };
                proxy = record {
                    initial_supply = 150000000000: nat;
                    refueling_amount = 50000000000: nat;
                    refueling_threshold = 50000000000: nat;
                };
        })' --with-cycles 1400000000000 --wallet $(dfx identity get-wallet --network ic)
# setup
dfx canister --network ic call shib_eth_1_indexer setup "(
    \"$(dfx canister --network ic id shared_algorithm_lens)\",
    vec { \"$(dfx canister --network ic id shib_eth_1_pool_fees)\"; \"$(dfx canister --network ic id shib_eth_1_tcumul_28x6hr)\"; \"$(dfx canister --network ic id shib_eth_1_v3pool)\"; \"$(dfx canister --network ic id eth_usdc_price)\" },
)"
# set_task
dfx canister --network ic call shib_eth_1_indexer set_task '(14400, 5)'
