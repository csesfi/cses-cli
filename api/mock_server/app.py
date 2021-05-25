import connexion


def login_get(user):
    return {"X-AUTH-TOKEN": "asdf"}

def basic_auth(username, password, required_scopes=None):
    if username == "kalle":
        # will be given as a parameter to login_get
        return {"sub": username}
    return None

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_api("openapi.yaml")
app.run(host="127.0.0.1", port=4010)
