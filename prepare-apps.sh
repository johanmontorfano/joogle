#!/bin/bash

cd ./apps/search_console
vite build
./prepare_server_hosting.sh
cd ../..
