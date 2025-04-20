#!/bin/bash

if [ "$1" = 'local' ]; then
    cd frontend || exit
    npm run build

    cd ..

    cargo run same-student local-mail
else
    cd frontend || exit
    npm run build

    cd ..

    cargo run --release
fi