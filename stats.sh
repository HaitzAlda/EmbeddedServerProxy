#!/bin/bash


echo "Testing directly:\n"
wrk -t$(nproc) -c100 -d10s http://localhost:8001/

echo "Testing via  Proxy: \n"

wrk -t$(nproc) -c100 -d10s http://localhost:8080/

