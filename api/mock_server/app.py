import connexion


def login_get():
    if connexion.request.json == {"username": "kalle",
                                  "password": "kissa2"}:
        return {"X-Auth-Token": "asdf"}
    return ("Invalid username/password", 401)

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_api("openapi.yaml")
app.run(host="127.0.0.1", port=4010)
