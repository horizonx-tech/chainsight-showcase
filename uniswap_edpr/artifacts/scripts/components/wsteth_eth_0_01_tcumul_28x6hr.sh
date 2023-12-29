#!/bin/bash
# init
dfx canister --network ic call wsteth_eth_0_01_tcumul_28x6hr init_in '(variant { "Production" }, record {
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
dfx canister --network ic call wsteth_eth_0_01_tcumul_28x6hr setup "(
    \"109830a1aaad605bbf02a9dfa7b0b92ec2fb7daa\",
    record {
        url = \"https://eth-mainnet.g.alchemy.com/v2/vNTwvi1wvZDU4Oq5Z854mNXhWCSm0ch-\";
        from = null;
        chain_id = 1;
        env = variant { Production };
    }
)"
# set_task
dfx canister --network ic call wsteth_eth_0_01_tcumul_28x6hr set_task '(7200, 0)'
