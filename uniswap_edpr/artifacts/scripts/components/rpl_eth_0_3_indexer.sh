#!/bin/bash
# init
dfx canister --network ic call rpl_eth_0_3_indexer init_in '(variant { "Production" }, record {
                refueling_interval = 86400: nat64;
                vault_intial_supply = 1000000000000: nat;
                indexer = record { 
                    initial_supply = 0: nat;
                    refueling_amount = 1000000000000: nat;
                    refueling_threshold = 500000000000: nat;
                };
                db = record { 
                    initial_supply = 1500000000000: nat;
                    refueling_amount = 1000000000000: nat;
                    refueling_threshold = 500000000000: nat;
                };
                proxy = record { 
                    initial_supply = 300000000000: nat;
                    refueling_amount = 100000000000: nat;
                    refueling_threshold = 100000000000: nat;
                };
        })' --with-cycles 2800000000000 --wallet $(dfx identity get-wallet --network ic)
# setup
dfx canister --network ic call rpl_eth_0_3_indexer setup "(
    \"$(dfx canister --network ic id shared_algorithm_lens)\",
    vec { \"$(dfx canister --network ic id rpl_eth_0_3_pool_fees)\"; \"$(dfx canister --network ic id rpl_eth_0_3_tcumul_28x6hr)\"; \"$(dfx canister --network ic id rpl_eth_0_3_v3pool)\"; \"$(dfx canister --network ic id eth_usdc_price)\" },
)"
# set_task
dfx canister --network ic call rpl_eth_0_3_indexer set_task '(7200, 5)'
