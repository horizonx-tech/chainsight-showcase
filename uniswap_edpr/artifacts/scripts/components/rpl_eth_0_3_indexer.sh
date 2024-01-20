#!/bin/bash
# init
dfx canister  call rpl_eth_0_3_indexer init_in '(variant { "LocalDevelopment" }, record {
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
        })' --with-cycles 2800000000000 --wallet $(dfx identity get-wallet )
# setup
dfx canister  call rpl_eth_0_3_indexer setup "(
    \"$(dfx canister  id shared_algorithm_lens)\",
    vec { \"aax3a-h4aaa-aaaaa-qaahq-cai\"; \"c5kvi-uuaaa-aaaaa-qaaia-cai\"; \"c2lt4-zmaaa-aaaaa-qaaiq-cai\"; \"br5f7-7uaaa-aaaaa-qaaca-cai\" },
)"
# set_task
dfx canister  call rpl_eth_0_3_indexer set_task '(144, 5)'
