#!/bin/bash
# init
dfx canister  call shib_eth_1_relayer init_in '(variant { "LocalDevelopment" }, record {
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
dfx canister  call shib_eth_1_relayer setup "(
    \"0xB5Ef491939A6dBf17287666768C903F03602c550\",
    record {
        url = \"https://ethereum-sepolia.blockpi.network/v1/rpc/public\";
        from = null;
        chain_id = 11155111;
        env = variant { LocalDevelopment };
    },
    \"$(dfx canister  id shared_indexer_lens)\",
    vec { \"cgpjn-omaaa-aaaaa-qaakq-cai\" },
)"
# set_task
dfx canister  call shib_eth_1_relayer set_task '(144, 10)'
