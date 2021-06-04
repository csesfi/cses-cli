import string
import random
import connexion
import werkzeug

import base64

from connexion import NoContent
from connexion import RestyResolver

from connexion.exceptions import ValidationError, BadRequestProblem
from connexion.exceptions import Unauthorized
from werkzeug.exceptions import MethodNotAllowed

class SubmissionTracker:
    def __init__(self, submission_infos):
        assert len(submission_infos) > 0
        self.infos = submission_infos
        self._position = 0

    def next(self):
        info = self.infos[self._position]
        if self._position + 1 < len(self.infos):
            self._position += 1
        return info

class NewSubmission:
    """A class for representing data sent to server at new submission."""
    def __init__(self, course_id, task_id, submission_json):
        self.course_id = course_id
        self.task_id = task_id
        self.submission_json = submission_json

    def __eq__(self, other):
        return self.course_id == other.course_id \
            and self.task_id == other.task_id \
            and self.submission_json == other.submission_json

class SubmissionScenario:
    """A class for representing a single possible submission scenario"""
    def __init__(self, new_submission, submission_infos):
        self.new_submission = new_submission
        self.submission_infos = submission_infos

class ServerState:
    def __init__(self, valid_logins, scenarios):
        # tokens need to be saved to test the logout
        self.valid_tokens = []
        self.valid_logins = valid_logins
        self.submission_scenarios = scenarios

        self.submission_trackers = {}

    def login(self, credentials):
        if credentials not in self.valid_logins:
            return None

        token = self._generate_token()
        self.valid_tokens.append(token)
        return token

    def logout(self, token):
        """Logs out a valid api key"""
        assert self.is_valid(token)
        state.valid_tokens.remove(token)

    def is_valid(self, token):
        return token in self.valid_tokens

    def _generate_token(self):
        token = "".join(random.choices(string.hexdigits, k=16))
        return token

    def add_submission(self, new_submission):
        """Tries to add `new_submission`.

        If `new_submission` doesn't match any of the valid
        submission, then return None.
        Otherwise returns the submission id"""

        submission = None
        for x in self.submission_scenarios:
            if x.new_submission == new_submission:
                submission = x

        if submission is None:
            return None

        submission_id = random.getrandbits(64)
        self.submission_trackers[submission_id] = SubmissionTracker(
            submission.submission_infos)

        return submission_id

    def get_submission_info(self, course_id, task_id, submission_id):
        """Returns the next state of the submission `submission_id`"""
        if submission_id not in self.submission_trackers:
            return None
        return self.submission_trackers[submission_id].next()

random.seed(1337)
state = ServerState(
    valid_logins = [
        {"username": "kalle", "password": "kissa2"},
        {"username": "uolevi", "password": "12345"},
        {"username": "Olaf", "password": "ILoveSummer"}
    ],
    scenarios = [
        SubmissionScenario(
            NewSubmission(course_id = "kurssi", task_id = 2,
                submission_json = {
                    "language": {"name": "Rust", "option": None},
                    "filename": "main.rs",
                    "content": "use std::io;\n"}
            ),
            [
                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "Rust",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
                },

                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "Rust",
                    "option": None
                },
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "tests": [{
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                    }
                ]}
            ],
        ),
        SubmissionScenario(
            NewSubmission(course_id = "alon", task_id = 4,
                submission_json = {
                    "language": {
                        "name": "C++",
                        "option": "C++17"
                        },
                    "filename": "main.cpp",
                    "content": "#include <iostream>\n"}
            ),
            [
                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "PENDING",
                "pending": True,
                },

                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "pending": True,
                },

                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "tests": [
                    {
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                    },
                    {
                    "number": 2,
                    "verdict": "WRONG ANSWER",
                    "time": 800
                    }
                ]}

            ]
        ),
        SubmissionScenario(
            NewSubmission(course_id = "cses", task_id = 13,
                submission_json = {
                    "language": {
                        "name": "C++",
                        "option": "C++17"
                        },
                    "filename": "13.rs",
                    "content": "use std::io;\n\nfn main() {\n"}
            ),
            [
                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "PENDING",
                "pending": True,
                },

                {
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
^~
"""
                },

            ]
        ),
        SubmissionScenario(
            NewSubmission(course_id = "cses", task_id = 42,
                submission_json = {
                    "language": {
                        "name": "C++",
                        "option": "C++17"
                        },
                    "filename": "main.cpp",
                    "content": "#include <iostream>\n"}
            ),
            [
                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "PENDING",
                "pending": True,
                },

                {"time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer expressions [-Wsign-compare]
  for (int i = 0; i < a.size(); i++) {
""",
                "tests": [
                    {
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                    }
                ]}

            ]
        )
    ]
)

def login_post():
    token = state.login(connexion.request.json)
    print(f"got token: {token}")
    if token is not None:
        return ({"X-Auth-Token": token}, 200)

    return ({"message": "Invalid username/password", "code": "invalid_credentials"}, 401)

def logout_post(token_info):
    state.logout(token_info["apikey"])
    return (NoContent, 204)

def submissions_post(token_info, course_id, task_id):
    details = connexion.request.json
    try:
        details["content"] = base64.b64decode(details["content"]).decode("utf-8")
    except Exception:
        return ({"message": f"Failed to decode submission content base64 encoding",
                 "code": "client_error"}, 400)

    new_submission = NewSubmission(course_id, task_id, connexion.request.json)
    submission_id = state.add_submission(new_submission)
    if submission_id is None:
        return ({"message": f"Invalid submission: {details}", "code": "client_error"}, 400)
    return ({"id": submission_id}, 200)

def get_submission(token_info, course_id, task_id, submission_id, poll=False):
    print(f"get submit: {token_info}")
    print(f"course_id: {course_id}")
    print(f"task_id: {task_id}")
    print(f"submission_id: {submission_id}")
    print(f"poll: {poll}")
    submission_info = state.get_submission_info(course_id, task_id, submission_id)
    if submission_info is None:
        return ({"message": "Submission not found", "code": "client_error"}, 404)
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
app.add_api("openapi.yaml", validate_responses=True, resolver=RestyResolver('api'))
app.run(host="127.0.0.1", port=4010)
