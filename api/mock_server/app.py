# Linted with 'flake8 . --exclude=venv --count --show-source --statistics'

import sys
import time

import connexion
import base64

from connexion import NoContent
from connexion import RestyResolver

from connexion.exceptions import BadRequestProblem
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

from server_state import ServerState
from submission import NewSubmission
from scenarios import scenarios, DEFAULT_TASK, UOLEVI


integration = False
try:
    integration = bool(sys.argv[1])
except Exception:
    pass


state = ServerState(
    integration,
    scenarios=scenarios
)

app = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})


def login_post():
    token = state.login()
    print(f"got token: {token}")
    host = connexion.request.root_url
    return (
        {
            "X-Auth-Token": token,
            "authentication_url": f"{host}authorize-login?token={token}"
        },
        200
    )


@app.route('/authorize-login')
def authorize_login_post():
    token = connexion.request.args.get("token")
    fail = connexion.request.args.get("fail") is not None
    state.authorize_login(token, fail)
    return "", 204


@app.route('/authorize-all', methods=["POST"])
def authorize_all_post():
    state.authorize_all()
    return "", 204


def login_get(token_info):
    # Errors returned by security scheme
    return (UOLEVI, 200)


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
    submission_info = state.get_initial_submission_info(submission_id)
    if submission_info is None:
        return ({"message": f"Invalid submission: {details}",
                 "code": "client_error"}, 400)
    return (submission_info, 200)


def get_submission(token_info, course_id, submission_id, poll=False):
    print(f"get submit: {token_info}")
    print(f"course_id: {course_id}")
    print(f"submission_id: {submission_id}")
    print(f"poll: {poll}")
    if not integration and poll:
        time.sleep(1.5)
    submission_info = state.get_submission_info(submission_id)
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

    status = state.check_login(apikey)
    if status == "valid":
        return {"apikey": apikey}
    elif status == "pending":
        raise Unauthorized(description="pending")
    else:
        # This is overriden by the render_api_authentication_failed function
        raise Unauthorized()


def render_invalid_query(exception):
    return ({"message": "Invalid query format", "code": "client_error"}, 400)


def render_api_authentication_failed(exception):
    if exception.description == "pending":
        return ({"message": "API key pending login",
                 "code": "pending_api_key"}, 401)
    else:
        return ({"message": "Invalid api key", "code": "invalid_api_key"}, 401)


def render_method_not_allowed(exception):
    return ({"message": "Invalid HTTP method", "code": "client_error"}, 405)


app.add_error_handler(BadRequestProblem, render_invalid_query)
app.add_error_handler(Unauthorized, render_api_authentication_failed)
app.add_error_handler(MethodNotAllowed, render_method_not_allowed)
app.add_api("openapi.yaml", validate_responses=True,
            resolver=RestyResolver('api'))
app.run(host="127.0.0.1", port=4011 if integration else 4010)
