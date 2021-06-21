import sys

INTEGRATION = False
try:
    INTEGRATION = bool(sys.argv[1])
except IndexError:
    pass


def __file_details(filename: str, new_name: str = None) -> str:
    return {
        "filename": new_name if new_name is not None else filename,
        "content": open(f"../../tests/files/{filename}").read()
    }


# Task number
DEFAULT_TASK = 34

# Submitter
UOLEVI = {
    "id": 1,
    "username": "mooc~123",
    "displayname": "uolevi@cses.fi (mooc.fi)"
}


# Submission data values
RUST_CODE = {
    "language": {"name": "Rust", "option": None},
    **__file_details("main.rs")
}
CPP_CODE = {
    "language": {"name": "C++", "option": "C++17"},
    **__file_details("main.cpp")
}
CPP_CODE_NO_DETAILS = {
    "language": {"name": None, "option": None},
    **__file_details("main.cpp")
}
UNKNOWN_CODE_NO_DETAILS = {
    "language": {"name": None, "option": None},
    **__file_details("main.cpp", "main.asdf")
}
CPP_CODE_NO_LANGUAGE = {
    "language": {"name": None, "option": "C++17"},
    **__file_details("main.cpp")
}
RS_13_CODE = {
    "language": {"name": "C++", "option": "C++17"},
    **__file_details("13.rs")
}
PY_TODO_CODE = {
    "language": {"name": "CPython", "option": None},
    **__file_details("todo.py")
}
PY_CODE = {
    "language": {"name": "CPython", "option": None},
    **__file_details("lucky.py")
}

# Task information
SUMMA_TASK = {
    "id": 123,
    "name": "Summa",
}

# Non pending submission
OLD_SUBMISSION = {
    "id": 1,
    "task": SUMMA_TASK,
    "sender": UOLEVI,
    "time": "2010-07-21T17:32:28Z",
    "language": {
        "name": "C++",
        "option": None
    },
    "status": "READY",
    "pending": False,
    "result": "ACCEPTED",
    "test_report": "All tests accepted"
}

SUBMISSION_LIST = {
    "submissions": [
        {
            "id": 1234567,
            "time": "2017-07-21T17:32:28Z",
            "language": {
                "name": "CPython",
                "option": None
            },
            "code_time": 500,
            "size": 1000,
            "result": "pass"
        },
        {
            "id": 7654321,
            "time": "2020-07-21T17:32:28Z",
            "language": {
                "name": "C++",
                 "option": "C++17"
            },
            "code_time": None,
            "size": 200,
            "result": "fail"
        }
    ]
}

SUBMISSION_LIST_WITH_MISSING_FIELDS = {
    "submissions": [
        {
            "id": 1234567,
            "time": "2017-07-21T17:32:28Z",
            "language": {
                "name": "CPython",
                "option": None
            },
            "result": "pass"
        },
        {
            "id": 7654321,
            "time": "2020-07-21T17:32:28Z",
            "language": {
                "name": "C++",
                 "option": "C++17"
            },
            "result": "fail"
        }
    ]
}

EMPTY_SUBMISSION_LIST = {
    "submissions": []
}

# Base progress information
CPP_PROGRESS_BASE = {
    "task": SUMMA_TASK,
    "sender": UOLEVI,
    "time": "2017-07-21T17:32:28Z",
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "status": "PENDING",
    "pending": True,
}

# Compiler messages
COMPILER_ERROR = """input/code.cpp:1:1: error: 'use' does not name \
a type
use std::io;
^~~
input/code.cpp:3:1: error: 'fn' does not name a type
fn main() {
^~
"""
COMPILER_WARNING = """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer \
expressions [-Wsign-compare]
for (int i = 0; i < a.size(); i++) {
"""
COMPILER_WARNING_OTHER = """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function \
[-Wuninitialized]
   while(x != 123);
         ~~^~~~~~"""

INFO_HEADER = ""

# Course content fetch
INSTRUCTIONS_TEXT = {
    "objectType": "text",
    "name": "Instructions",
    "id": 7582,
    "link": "https://cses.fi/alon/text/2434",
}

EXTERNAL_WEBSITE_LINK = {
    "objectType": "link",
    "name": "External website",
    "link": "https://alon.mooc.fi/materiaali",
}

TASK_1 = {
    "objectType": "task",
    "name": "Wierd algorithm",
    "id": 1068,
    "link": "https://cses.fi/alon/task/1068",
    "status": "none"
}

TASK_2 = {
    "objectType": "task",
    "name": "Increasing array",
    "id": 1094,
    "link": "https://cses.fi/alon/task/1094",
    "status": "none"
}

TASK_1_WITH_STATUS = {
    "objectType": "task",
    "name": "Wierd algorithm",
    "id": 1068,
    "link": "https://cses.fi/alon/task/1068",
    "status": "pass"
}

TASK_2_WITH_STATUS = {
    "objectType": "task",
    "name": "Increasing array",
    "id": 1094,
    "link": "https://cses.fi/alon/task/1094",
    "status": "fail"
}
# Courses
VISIBLE_COURSES = [
    {
        "id": "teku",
        "name": "Test course",
        "description": "This is a test course used by the Python test server."
    },
    {
        "id": "problemset",
        "name": "CSES Problem Set",
        "description": "The CSES Problem Set contains a collection of " +
                       "competitive programming practice problems."
    }
]

ALL_COURSES = VISIBLE_COURSES + [
    {
        "id": "hidden",
        "name": "Hidden course",
        "description": "If you can see this, you're logged in."
    }
]
