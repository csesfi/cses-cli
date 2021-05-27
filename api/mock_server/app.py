import connexion
import werkzeug

from connexion import NoContent

from connexion.exceptions import ValidationError, BadRequestProblem
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

def login_get():
    if connexion.request.json == {"username": "kalle",
                                  "password": "kissa2"}:
        return {"X-Auth-Token": "asdf"}
    return ({"message": "Invalid username/password", "code": "invalid_credentials"}, 401)

def logout_post(token_info):
    print(token_info)
    return (NoContent, 204)

def apikey_auth(apikey, required_scopes=None):
    """Corresponds to the the apiKeyAuth in OpenAPI.

    Should return a dictionary (e.g. `return {"a": 1, "b": 2}`)
    This dictionary can be accessed with a parameter
    `token_info` in the function corresponding to the
    `operationId` in the OpenAPI path. (e.g. `def submit(token_info): ...`)
    """
    if apikey == "asdf":
        return {"this": "goes to logout function"}

    # this will be overriden by the render_api_authentication_failed function
    raise werkzeug.exceptions.Unauthorized()


def render_invalid_query(exception):
    return ({"message": "Invalid query format", "code": "client_error"}, 400)

def render_api_authentication_failed(exception):
    print("here")
    return ({"message": "Invalid api key", "code": "invalid_api_key"}, 401)

def render_method_not_allowed(exception):
    return ({"message": "Invalid HTTP method", "code": "client_error"}, 405)

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_error_handler(BadRequestProblem, render_invalid_query)
app.add_error_handler(Unauthorized, render_api_authentication_failed)
app.add_error_handler(MethodNotAllowed, render_method_not_allowed)
app.add_api("openapi.yaml")
app.run(host="127.0.0.1", port=4010)
