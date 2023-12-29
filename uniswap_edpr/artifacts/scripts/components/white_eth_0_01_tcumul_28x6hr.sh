#!/bin/bash
# init
dfx canister --network ic call white_eth_0_01_tcumul_28x6hr init_in '(variant { "Production" }, record {
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
        })' --with-cycles 1120000000000 --wallet $(dfx identity get-wallet --network ic)
# setup
dfx canister --network ic call white_eth_0_01_tcumul_28x6hr setup "(
    \"c5c134a1f112efa96003f8559dba6fac0ba77692\",
    record {
        url = \"https://eth-mainnet.g.alchemy.com/v2/vNTwvi1wvZDU4Oq5Z854mNXhWCSm0ch-\";
        from = null;
        chain_id = 1;
        env = variant { Production };
    }
)"
# set_task
dfx canister --network ic call white_eth_0_01_tcumul_28x6hr set_task '(7200, 0)'
