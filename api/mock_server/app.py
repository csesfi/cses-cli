import connexion

def login_get():
    if connexion.request.json == {"username": "kalle",
                                  "password": "kissa2"}:
        return {"X-Auth-Token": "asdf"}
    return ({"message": "Invalid username/password"}, 401)

def logout_post():
    if connexion.request.json["X-Auth-Token"] == "asdf":
        return {"Success": True}
    return ("Invalid Authorization token", 401)

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_api("openapi.yaml")
app.run(debug=True, host="127.0.0.1", port=4010)
