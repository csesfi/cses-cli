# Linted with 'flake8 . --exclude=venv --count --show-source --statistics'

import sys
import time

import connexion
import werkzeug

import base64

from connexion import NoContent
from connexion import RestyResolver

from connexion.exceptions import BadRequestProblem  # , ValidationError
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

from server_state import ServerState
from submission import NewSubmission
from scenarios import scenarios, DEFAULT_TASK


integration = False
try:
    integration = bool(sys.argv[1])
except(Exception):
    pass


state = ServerState(
    valid_logins=[
        {"username": "kalle", "password": "kissa2"},
        {"username": "uolevi", "password": "12345"},
        {"username": "Olaf", "password": "ILoveSummer"}
    ],
    scenarios=scenarios
)


def login_post():
    token = state.login(connexion.request.json)
    print(f"got token: {token}")
    if token is not None:
        return ({"X-Auth-Token": token}, 200)

    return ({"message": "Invalid username/password",
             "code": "invalid_credentials"}, 401)


def logout_post(token_info):
    state.logout(token_info["apikey"])
    return (NoContent, 204)


def submissions_post(token_info, course_id, task=DEFAULT_TASK):
    details = connexion.request.json
    try:
        details["content"] = base64.b64decode(details["content"]) \
                                   .decode("utf-8")
    except Exception:
        return ({"message": "Could not decode the content with base64",
                 "code": "client_error"}, 400)

    new_submission = NewSubmission(course_id, task, connexion.request.json)
    submission_id = state.add_submission(new_submission)
    if submission_id is None:
        return ({"message": f"Invalid submission: {details}",
                 "code": "client_error"}, 400)
    return ({"submission_id": submission_id, "task_id": task}, 200)


def get_submission(token_info, course_id, submission_id, poll=False):
    print(f"get submit: {token_info}")
    print(f"course_id: {course_id}")
    print(f"submission_id: {submission_id}")
    print(f"poll: {poll}")
    if not integration and poll:
        time.sleep(1.5)
    submission_info = state.get_submission_info(course_id,
                                                submission_id)
    if submission_info is None:
        return ({"message": "Submission not found",
                "code": "client_error"}, 404)
    return (submission_info, 200)


def apikey_auth(apikey, required_scopes=None):
    """Corresponds to the the apiKeyAuth in OpenAPI.

    Should return a dictionary (e.g. `return {"a": 1, "b": 2}`)
    This dictionary can be accessed with a parameter
    `token_info` in the function corresponding to the
    `operationId` in the OpenAPI path. (e.g. `def submit(token_info): ...`)
    """

    if state.is_valid(apikey):
        return {"apikey": apikey}

    # this will be overriden by the render_api_authentication_failed function
    raise werkzeug.exceptions.Unauthorized()


def render_invalid_query(exception):
    return ({"message": "Invalid query format", "code": "client_error"}, 400)


def render_api_authentication_failed(exception):
    return ({"message": "Invalid api key", "code": "invalid_api_key"}, 401)


def render_method_not_allowed(exception):
    return ({"message": "Invalid HTTP method", "code": "client_error"}, 405)


app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})
app.add_error_handler(BadRequestProblem, render_invalid_query)
app.add_error_handler(Unauthorized, render_api_authentication_failed)
app.add_error_handler(MethodNotAllowed, render_method_not_allowed)
app.add_api("openapi.yaml", validate_responses=True,
            resolver=RestyResolver('api'))
app.run(host="127.0.0.1", port=4011 if integration else 4010)
