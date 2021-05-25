#!/bin/bash

set -x

echo

curl -u kalle:kissa2 http://127.0.0.1:4010/login

echo

curl -u ahslaaks:kissa2 http://127.0.0.1:4010/login

echo

curl --request POST http://127.0.0.1:4010/login

echo

curl -u 10:kissa2 http://127.0.0.1:4010/login
