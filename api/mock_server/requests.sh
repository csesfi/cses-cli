#!/bin/bash

set -x

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
	--request POST \
	--data '{"username": "ahslaaks", "password": "salasana"}' \
	http://127.0.0.1:4010/login
echo

curl --header "Content-Type: application/json" \
	--request POST \
	--data '{"username": "kalle", "password": "kissa2"}' \
	http://127.0.0.1:4010/login


echo


curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request POST \
	--data '{
		"language": {
			"name": "Rust"
		},
		"filename": "main.rs",
		"content": "dXNlIHN0ZDo6aW87"
}' \
	http://localhost:4010/course/kurssi/task/2/submit

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request GET \
	http://localhost:4010/course/1/task/2/submit/4

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request GET \
	http://localhost:4010/course/1/task/2/submit/3/poll

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request GET \
	http://localhost:4010/course/1/task/2/submit/16604403594511500093/poll

echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request GET \
	http://localhost:4010/course/1/task/2/submit/16604403594511500093

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: kissa" \
	--request POST \
	http://localhost:4010/logout

echo

curl --header "Content-Type: application/json" \
	--request POST \
	http://localhost:4010/logout
echo

curl --header "Content-Type: application/json" \
	--header "X-Auth-Token: db8c3C8BE6F4eE2C" \
	--request POST \
	http://localhost:4010/logout

echo
