#!/bin/bash

PERSEUS_BIN_VERSION=$(perseus --version)
echo $PERSEUS_BIN_VERSION

if [ $PERSEUS_VERSION ]; then
    if [ ! -f ~/.cargo/bin/perseus ]; then
        echo "perseus doesn't exist... installing $(echo $PERSEUS_VERSION)"
        ## Should go to /opt/buildhome/.cargo/bin/perseus
        #  https://github.com/framesurge/perseus/releases/download/v0.3.6/perseus-linux-amd64
        # curl -OL https://github.com/framesurge/perseus/releases/download/v$PERSEUS_VERSION/perseus-linux-amd64
        # tar xvzf /perseus-linux-amd64 -C ~/.cargo/bin/
        # rm -rf /perseus-linux-amd64
        cargo install perseus-cli --version="${PERSEUS_VERSION}"
    else
        if [ "${PERSEUS_BIN_VERSION}" = "perseus-cli ${PERSEUS_VERSION}" ]; then
            echo "$(perseus --version) already exists"
        else
            # Forces an install of the new version if it was different from installed version
            cargo install perseus-cli --version="${PERSEUS_VERSION}" --force
        fi
    fi
else
    # Call the command for the package silently
    ls -f ~/.cargo/bin/perseus > /dev/null

    # Get the exit code of the last command
    command_exit_code="$(echo $?)"

    # Run installation if exit code is not equal to 0
    if [ "$command_exit_code" -ne "0" ]; then
        # Package does not exist: Do the package installation
        cargo install perseus
    else
    echo "Skipping 'perseus-cli' installation: Crate ($PERSEUS_BIN_VERSION) already exists"
    fi;
fi