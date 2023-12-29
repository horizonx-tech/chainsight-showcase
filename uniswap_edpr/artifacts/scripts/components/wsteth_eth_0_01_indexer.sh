#!/bin/bash
# init
dfx canister  call wsteth_eth_0_01_indexer init_in '(variant { "LocalDevelopment" }, record {
                refueling_interval = 86400: nat64;
                vault_intial_supply = 400000000000: nat;
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
        })' --with-cycles 1120000000000 --wallet $(dfx identity get-wallet )
# setup
dfx canister  call wsteth_eth_0_01_indexer setup "(
    \"$(dfx canister  id shared_algorithm_lens)\",
    vec { \"$(dfx canister  id wsteth_eth_0_01_pool_fees)\"; \"$(dfx canister  id wsteth_eth_0_01_tcumul_28x6hr)\"; \"$(dfx canister  id wsteth_eth_0_01_v3pool)\"; \"$(dfx canister  id eth_usdc_price)\" },
)"
# set_task
dfx canister  call wsteth_eth_0_01_indexer set_task '(7200, 5)'
