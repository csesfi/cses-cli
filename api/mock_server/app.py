import connexion
import werkzeug

def login_get():
    if connexion.request.json == {"username": "kalle",
                                  "password": "kissa2"}:
        return {"X-Auth-Token": "asdf"}
    return ({"message": "Invalid username/password"}, 401)

def logout_post(token_info):
    print(token_info)
    return (204)

def apikey_auth(apikey, required_scopes=None):
    """Corresponds to the the apiKeyAuth in OpenAPI.

    Should return a dictionary (e.g. `return {"a": 1, "b": 2}`)
    This dictionary can be accessed with a parameter
    `token_info` in the function corresponding to the
    `operationId` in the OpenAPI path. (e.g. `def submit(token_info): ...`)
    """
    if apikey == "asdf":
        return {"this": "goes to logout function"}

    raise werkzeug.exceptions.Unauthorized(
        response = {"message": "Invalid Authorization token"})

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_api("openapi.yaml")
app.run(debug=True, host="127.0.0.1", port=4010)
