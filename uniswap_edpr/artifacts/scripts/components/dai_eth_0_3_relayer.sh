#!/bin/bash
# init
dfx canister  call dai_eth_0_3_relayer init_in '(variant { "LocalDevelopment" }, record {
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
dfx canister  call dai_eth_0_3_relayer setup "(
    \"0xB5Ef491939A6dBf17287666768C903F03602c550\",
    record {
        url = \"https://ethereum-sepolia.blockpi.network/v1/rpc/public\";
        from = null;
        chain_id = 11155111;
        env = variant { LocalDevelopment };
    },
    \"$(dfx canister  id dai_eth_0_3_indexer_lens)\",
    vec { \"$(dfx canister  id dai_eth_0_3_pool_fees)\"; \"$(dfx canister  id dai_eth_0_3_tcumul_28x6hr)\"; \"$(dfx canister  id dai_eth_0_3_v3pool)\"; \"$(dfx canister  id eth_usdc_price)\" },
)"
# set_task
dfx canister  call dai_eth_0_3_relayer set_task '(7200, 10)'