#!/bin/bash

cd frontend || exit
npm run build

cd ..

cargo run
