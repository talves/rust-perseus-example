#!/bin/bash

WASM_PACK_BIN_VERSION=$(wasm-pack --version)
echo $WASM_PACK_BIN_VERSION

if [ $WASM_PACK_VERSION ]; then
    if [ ! -f ~/.cargo/bin/wasm-pack ]; then
        echo "wasm-pack doesn't exist... installing $(echo $WASM_PACK_VERSION)"
        ## Should go to /opt/buildhome/.cargo/bin/wasm-pack
        #  https://github.com/rustwasm/wasm-pack/releases/download/v0.3.6/wasm-pack-linux-amd64
        # curl -OL https://github.com/rustwasm/wasm-pack/releases/download/v$WASM_PACK_VERSION/wasm-pack-linux-amd64
        # tar xvzf /wasm-pack-linux-amd64 -C ~/.cargo/bin/
        # rm -rf /wasm-pack-linux-amd64
        cargo install wasm-pack --version="${WASM_PACK_VERSION}"
    else
        if [ "${WASM_PACK_BIN_VERSION}" = "wasm-pack ${WASM_PACK_VERSION}" ]; then
            echo "$(wasm-pack --version) already exists"
        else
            # Forces an install of the new version if it was different from installed version
            cargo install wasm-pack --version="${WASM_PACK_VERSION}" --force
        fi
    fi
else
    # Call the command for the package silently
    ls -f ~/.cargo/bin/wasm-pack > /dev/null

    # Get the exit code of the last command
    command_exit_code="$(echo $?)"

    # Run installation if exit code is not equal to 0
    if [ "$command_exit_code" -ne "0" ]; then
        # Package does not exist: Do the package installation
        cargo install wasm-pack
    else
    echo "Skipping 'wasm-pack' installation: Crate ($WASM_PACK_BIN_VERSION) already exists"
    fi;
fi