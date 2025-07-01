#!/bin/bash


echo "Testing directly:\n"
wrk -t$(nproc) -c100 -d10s http://localhost:8001/static/sample.html

echo "Testing via  Proxy: \n"

wrk -t$(nproc) -c100 -d10s http://localhost:8080/static/sample.html

