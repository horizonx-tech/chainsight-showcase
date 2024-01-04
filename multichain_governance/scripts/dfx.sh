# !bin/sh

cd $(dirname $0)

./1_launch_dfx_local.sh
./2_update_addresses.sh
./3_deploy_canisters.sh
./4_deploy_management_canisters.sh
./5_init_canisters.sh
