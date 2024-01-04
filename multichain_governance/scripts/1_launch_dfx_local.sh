# !bin/bash

cd $(dirname $0)/..

# ensure dfx is stopped
dfx stop

# Launch dfx local
dfx start --clean --background
