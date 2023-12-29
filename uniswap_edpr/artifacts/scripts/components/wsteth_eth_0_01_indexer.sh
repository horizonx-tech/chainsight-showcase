#!/bin/bash
# init
dfx canister --network ic call wsteth_eth_0_01_indexer init_in '(variant { "Production" }, record {
                refueling_interval = 86400: nat64;
                vault_intial_supply = 450000000000: nat;
                indexer = record { 
                    initial_supply = 0: nat;
                    refueling_amount = 400000000000: nat;
                    refueling_threshold = 200000000000: nat;
                };
                db = record { 
                    initial_supply = 600000000000: nat;
                    refueling_amount = 400000000000: nat;
                    refueling_threshold = 200000000000: nat;
                };
                proxy = record { 
                    initial_supply = 120000000000: nat;
                    refueling_amount = 40000000000: nat;
                    refueling_threshold = 40000000000: nat;
                };
        })' --with-cycles 1200000000000 --wallet $(dfx identity get-wallet --network ic)
# setup
dfx canister --network ic call wsteth_eth_0_01_indexer setup "(
    \"$(dfx canister --network ic id shared_algorithm_lens)\",
    vec { \"$(dfx canister --network ic id wsteth_eth_0_01_pool_fees)\"; \"$(dfx canister --network ic id wsteth_eth_0_01_tcumul_28x6hr)\"; \"$(dfx canister --network ic id wsteth_eth_0_01_v3pool)\"; \"$(dfx canister --network ic id eth_usdc_price)\" },
)"
# set_task
dfx canister --network ic call wsteth_eth_0_01_indexer set_task '(7200, 5)'
