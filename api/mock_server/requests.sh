#!/bin/bash

set -x

echo

curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username": "kalle", "password": "kissa2"}' \
	http://127.0.0.1:4010/login

echo

curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username": "ahslaaks", "password": "salasana"}' \
	http://127.0.0.1:4010/login

echo

curl --request GET \
	http://127.0.0.1:4010/login

echo

curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username": 10, "password": "salasana"}' \
	http://127.0.0.1:4010/login

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: asdf" \
	--request POST \
	http://localhost:4010/logout

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: kissa" \
	--request POST \
	http://localhost:4010/logout

echo

curl --header "Content-Type: application/json" \
	--request POST \
	http://localhost:4010/logout