import connexion
import werkzeug

from connexion import NoContent
from connexion import RestyResolver

from connexion.exceptions import ValidationError, BadRequestProblem
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

def login_post():
    if connexion.request.json == {"username": "kalle",
                                  "password": "kissa2"}:
        return {"X-Auth-Token": "asdf"}
    return ({"message": "Invalid username/password", "code": "invalid_credentials"}, 401)

def logout_post(token_info):
    print(f"logout: {token_info}")
    return (NoContent, 204)

def submit_post(token_info, course_id, task_id):
    print(f"submit post: {token_info}")
    return ({"id": 1337}, 200)

def default_submission_info():
    return {
        "time": "2017-07-21T17:32:28Z",
        "language": {
            "name": "C++",
            "option": "C++17"
        },
        "status": "READY",
        "pending": False,
        "result": "ACCEPTED",
        "tests": [{
            "number": 1,
            "verdict": "ACCEPTED",
            "time": 120
            }
        ]
    }

def compile_error_submission_info():
    return {
        "time": "2017-07-21T17:32:28Z",
        "language": {
            "name": "C++",
            "option": "C++17"
        },
        "status": "COMPILE ERROR",
        "pending": False,
        "compiler": """input/code.cpp:1:1: error: 'use' does not name a type
use std::io;
^~~
input/code.cpp:3:1: error: 'fn' does not name a type
fn main() {
^~"""
    }

def get_submit(token_info, course_id, task_id, submission_id):
    print(f"get submit: {token_info}")
    if course_id == "comp":
        return (compile_error_submission_info(), 200)
    return (default_submission_info(), 200)

def get_submit_poll(token_info, course_id, task_id, submission_id):
    print(f"get submit poll: {token_info}")
    print(f"course_id: {course_id}")
    print(f"task_id: {task_id}")
    print(f"submission_id: {submission_id}")
    if course_id == "comp":
        return (compile_error_submission_info(), 200)
    return (default_submission_info(), 200)

def apikey_auth(apikey, required_scopes=None):
    """Corresponds to the the apiKeyAuth in OpenAPI.

    Should return a dictionary (e.g. `return {"a": 1, "b": 2}`)
    This dictionary can be accessed with a parameter
    `token_info` in the function corresponding to the
    `operationId` in the OpenAPI path. (e.g. `def submit(token_info): ...`)
    """
    if apikey == "asdf":
        return {"this": "goes to the function"}

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
app.add_api("openapi.yaml", validate_responses=True, resolver=RestyResolver('api'))
app.run(host="127.0.0.1", port=4010)
