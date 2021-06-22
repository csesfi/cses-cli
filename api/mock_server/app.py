# Linted with 'pylint * --ignore=venv,pyproject.toml,requests.sh,poetry.lock \
# -d missing-docstring,R0903,W0613' and
# 'flake8 --exclude=venv --count --show-source --statistics'

import time

import base64
from pathlib import Path

import connexion
import prance

from connexion import NoContent
from connexion import RestyResolver

from connexion.exceptions import BadRequestProblem
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

from server_state import ServerState
from submission import SubmissionInfo
from scenarios import SCENARIOS
import constants
from template import Template, TEMPLATES


STATE = ServerState(
    scenarios=SCENARIOS
)

APP = connexion.App(__name__, specification_dir="../",
                    options={"swagger_ui": False})


def login_post():
    token = STATE.login()
    print(f"got token: {token}")
    host = connexion.request.root_url
    return (
        {
            "X-Auth-Token": token,
            "authentication_url": f"{host}authorize-login?token={token}"
        },
        200
    )


@APP.route('/authorize-login')
def authorize_login_post():
    token = connexion.request.args.get("token")
    fail = connexion.request.args.get("fail") is not None
    STATE.authorize_login(token, fail)
    return "", 204


@APP.route('/authorize-all', methods=["POST"])
def authorize_all_post():
    STATE.authorize_all()
    return "", 204


def login_get(token_info):
    # Errors returned by security scheme
    return (constants.UOLEVI, 200)


def logout_post(token_info):
    STATE.logout(token_info["apikey"])
    return (NoContent, 204)

def common_submissions_post(token_info, scope_id, task):
    details = connexion.request.json
    try:
        details["content"] = base64.b64decode(details["content"]) \
                                   .decode("utf-8")
    except TypeError:
        return ({"message": "Could not decode the content with base64",
                 "code": "client_error"}, 400)

    details["content"] = details["content"].replace("\r\n", "\n")
    new_submission = SubmissionInfo(scope_id, task, details)
    submission_id = STATE.add_submission(new_submission)
    submission_info = STATE.get_initial_submission_info(submission_id)
    if submission_info is None:
        if task == constants.DEFAULT_TASK:
            return ({"message": "Failed to deduce the task for the submission",
                     "code": "task_deduction_error"}, 400)
        if details["language"]["name"] is None:
            return ({"message": "Failed to deduce the language for the " +
                                "submission",
                     "code": "language_deduction_error"}, 400)
        return ({"message": f"Invalid submission: {details}",
                 "code": "client_error"}, 400)
    return (submission_info, 200)


def courses_submissions_post(token_info, course_id,
                             task=constants.DEFAULT_TASK):
    return common_submissions_post(token_info, course_id, task)


def contests_submissions_post(token_info, contest_id,
                              task=constants.DEFAULT_TASK):
    return common_submissions_post(token_info, contest_id, task)


def common_get_submission(token_info, scope_id, submission_id, poll):
    if submission_id == 1 and not poll:
        return (constants.OLD_SUBMISSION_COURSE, 200)
    if not constants.INTEGRATION and poll:
        time.sleep(1.5)
    submission_info = STATE.get_submission_info(submission_id)
    if submission_info is None:
        return ({"message": "Submission not found",
                 "code": "client_error"}, 404)
    return (submission_info, 200)


def courses_get_submission(token_info, course_id, submission_id, poll=False):
    print(f"get submit: {token_info}")
    print(f"course_id: {course_id}")
    print(f"submission_id: {submission_id}")
    print(f"poll: {poll}")
    return common_get_submission(token_info, course_id, submission_id, poll)


def contests_get_submission(token_info, contest_id, submission_id, poll=False):
    return common_get_submission(token_info, contest_id, submission_id, poll)


def courses_get_submission_list(token_info, course_id, task):
    print(f"token_info: {token_info}")
    print(f"course_id: {course_id}")
    print(f"task_id: {task}")
    if task == "404":
        return (constants.EMPTY_SUBMISSION_LIST, 200)
    if task == "2":
        return (constants.SUBMISSION_LIST_COURSE_WITH_MISSING_FIELDS, 200)
    return (constants.SUBMISSION_LIST_COURSE, 200)


def contests_get_submission_list(token_info, contest_id, task):
    return (constants.EMPTY_SUBMISSION_LIST, 200)


def get_courses(token_info):
    if token_info == {}:
        return ({"courses": constants.VISIBLE_COURSES}, 200)
    return ({"courses": constants.ALL_COURSES}, 200)


def get_course_content(token_info, course_id):
    if course_id != "teku":
        return ({"message": "Course not found",
                 "code": "client_error"}, 404)

    task_list = []
    if token_info == {}:
        task_list = [constants.TASK_1_COURSE, constants.TASK_2_COURSE]
    else:
        task_list = [constants.TASK_1_COURSE_WITH_STATUS,
                     constants.TASK_2_COURSE_WITH_STATUS]

    return ({"sections": [
        {
            "header": "Info",
            "text": "This is the course's general info section",
            "list": [
                constants.INSTRUCTIONS_TEXT,
                constants.EXTERNAL_WEBSITE_LINK
            ]
        },
        {
            "header": "Week 1",
            "list": task_list
        },
    ]}, 200)


def get_contest_content(token_info, contest_id):
    return ({"message": "Contest not found",
             "code": "client_error"}, 404)


def common_get_template(token_info, scope_id, task, language, filename):
    has_token = (token_info != {})
    template = Template(has_token, scope_id, task, language, filename)
    if template in TEMPLATES:
        return (TEMPLATES[template], 200)

    return ({"message": "Template not found", "code": "client_error"}, 400)


def courses_get_template(token_info, course_id, task=None,
                         language=None, filename=None):
    return common_get_template(token_info, course_id, task, language, filename)


def contests_get_template(token_info, contest_id, task=None, language=None,
                          filename=None):
    return common_get_template(token_info, contest_id, task, language, filename)


def apikey_auth(apikey, required_scopes=None):
    """Corresponds to the the apiKeyAuth in OpenAPI.

    Should return a dictionary (e.g. `return {"a": 1, "b": 2}`)
    This dictionary can be accessed with a parameter
    `token_info` in the function corresponding to the
    `operationId` in the OpenAPI path. (e.g. `def submit(token_info): ...`)
    """

    status = STATE.check_login(apikey)
    if status == "valid":
        return {"apikey": apikey}
    if status == "pending":
        raise Unauthorized(description="pending")
    # this will be overriden by the render_api_authentication_failed function
    raise Unauthorized()


def render_invalid_query(exception):
    return ({"message": "Invalid query format", "code": "client_error"}, 400)


def render_api_authentication_failed(exception):
    if exception.description == "pending":
        return ({"message": "API key pending login",
                 "code": "pending_api_key"}, 401)
    return ({"message": "Invalid api key", "code": "invalid_api_key"}, 401)


def render_method_not_allowed(exception):
    return ({"message": "Invalid HTTP method", "code": "client_error"}, 405)


def combine_specs(file):
    parser = prance.ResolvingParser(
        str(file.absolute()), lazy=True, strict=True)
    parser.parse()
    return parser.specification


APP.add_error_handler(BadRequestProblem, render_invalid_query)
APP.add_error_handler(Unauthorized, render_api_authentication_failed)
APP.add_error_handler(MethodNotAllowed, render_method_not_allowed)
APP.add_api(combine_specs(Path("../openapi.yaml")), validate_responses=True,
            resolver=RestyResolver('api'))
APP.run(host="127.0.0.1", port=4011 if constants.INTEGRATION else 4010)
