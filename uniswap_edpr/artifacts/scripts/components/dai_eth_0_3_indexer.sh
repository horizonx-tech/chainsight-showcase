#!/bin/bash
# init
dfx canister --network ic call dai_eth_0_3_indexer init_in '(variant { "Production" }, record {
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
dfx canister --network ic call dai_eth_0_3_indexer setup "(
    \"$(dfx canister --network ic id shared_algorithm_lens)\",
    vec { \"lexrj-sqaaa-aaaag-qcy5q-cai\"; \"lwrgq-6aaaa-aaaag-qcy6q-cai\"; \"bciwu-hyaaa-aaaag-qczga-cai\"; \"apgs2-iqaaa-aaaag-qczbq-cai\" },
)"
# set_task
dfx canister --network ic call dai_eth_0_3_indexer set_task '(14400, 5)'
