from submission import SubmissionScenario, NewSubmission

RUST_CODE = {
    "language": {"name": "Rust", "option": None},
    "filename": "main.rs",
    "content": "use std::io;\n"
}
CPP_CODE = {
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
CPP_CODE_NO_LANGUAGE_NO_OPTION = {
    "language": {
        "name": None,
        "option": None
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
UNKNOWN_CODE_NO_LANGUAGE_NO_OPTION = {
    "language": {
        "name": None,
        "option": None
    },
    "filename": "main.asdf",
    "content": "#include <iostream>\n"
}
CPP_CODE_NO_LANGUAGE = {
    "language": {
        "name": None,
        "option": "C++17"
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
RS_13_CODE = {
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "filename": "13.rs",
    "content": "use std::io;\n\nfn main() {\n"
}

DEFAULT_TASK = 34

PY_TODO_CODE = {
    "language": {"name": "CPython", "option": None},
    "filename": "todo.py",
    "content": "def check(n):\n    # TODO\n"
}
PY_CODE = {
    "language": {"name": "CPython", "option": None},
    "filename": "lucky.py",
    "content": "def check(n):\n    s = 0\n"
}



scenarios = [
    SubmissionScenario(
        NewSubmission(course_id="kurssi", task_id=2, submission_json=RUST_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "Rust",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
            },

            {
                "time": "2017-07-21T17:32:28Z",
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
                }]
            }
        ],
    ),
    SubmissionScenario(
        NewSubmission(course_id="alon", task_id=4, submission_json=CPP_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "TESTING",
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
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
        NewSubmission(course_id="cses", task_id=13, submission_json=RS_13_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "compiler": """input/code.cpp:1:1: error: 'use' does not name \
a type
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
        NewSubmission(course_id="cses", task_id=42, submission_json=CPP_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer \
expressions [-Wsign-compare]
for (int i = 0; i < a.size(); i++) {
""",
                "tests": [{
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                }]
            }
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="cses", task_id=111, submission_json=UNKNOWN_CODE_NO_LANGUAGE_NO_OPTION),
        [
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": None,
                    "option": None
                },
                "status": "READY",
                "result": "INVALID LANGUAGE",
                "pending": False,
            }
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="cses", task_id=444, submission_json=CPP_CODE_NO_LANGUAGE_NO_OPTION),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer \
expressions [-Wsign-compare]
for (int i = 0; i < a.size(); i++) {
""",
                "tests": [{
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                }]
            }
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="cses", task_id=555, submission_json=CPP_CODE_NO_LANGUAGE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer \
expressions [-Wsign-compare]
for (int i = 0; i < a.size(); i++) {
""",
                "tests": [{
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                }]
            }
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="progress", task_id=7, submission_json=CPP_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "TESTING",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function [-Wuninitialized]
   while(x != 123);
         ~~^~~~~~""",
                "test_progress": {
                    "finished_tests": 2,
                    "total_tests": 71,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function [-Wuninitialized]
   while(x != 123);
         ~~^~~~~~""",
                "test_progress": {
                    "finished_tests": 18,
                    "total_tests": 71,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function [-Wuninitialized]
   while(x != 123);
         ~~^~~~~~""",
                "test_progress": {
                    "finished_tests": 35,
                    "total_tests": 71,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function [-Wuninitialized]
   while(x != 123);
         ~~^~~~~~""",
                "test_progress": {
                    "finished_tests": 53,
                    "total_tests": 71,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "READY",
                "compiler": """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function [-Wuninitialized]
   while(x != 123);
         ~~^~~~~~""",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    {
                        "number": 1,
                        "verdict": "ACCEPTED",
                        "time": 120
                    },
                    {
                        "number": 2,
                        "verdict": "OUTPUT LIMIT EXCEEDED",
                        "time": 800
                    },
                    {
                        "number": 3,
                        "verdict": "WRONG ANSWER",
                        "time": 314
                    },
                    {
                        "number": 4,
                        "verdict": "TIME LIMIT EXCEEDED",
                        "time": None
                    },
                ]}
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="progress", task_id=8, submission_json=CPP_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "PENDING",
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
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
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 0,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 6,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 9,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 9,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 9,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 10,
                    "total_tests": 10,
                },
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "C++",
                    "option": "C++17"
                },
                "status": "READY",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    {
                        "number": 1,
                        "verdict": "ACCEPTED",
                        "time": 120
                    },
                    {
                        "number": 2,
                        "verdict": "OUTPUT LIMIT EXCEEDED",
                        "time": 800
                    },
                    {
                        "number": 3,
                        "verdict": "WRONG ANSWER",
                        "time": 314
                    },
                    {
                        "number": 4,
                        "verdict": "TIME LIMIT EXCEEDED",
                        "time": None
                    },
                ]}
        ]
    ),
    SubmissionScenario(
        NewSubmission(course_id="cses", task_id=DEFAULT_TASK, submission_json=CPP_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
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
                    "option": None
                },
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "tests": [{
                    "number": 1,
                    "verdict": "ACCEPTED",
                    "time": 120
                }]
            }
        ],
    ),
    SubmissionScenario(
        NewSubmission(course_id="tira21k", task_id=23, submission_json=PY_CODE),
        [
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "CPython",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
            },
            {
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "CPython",
                    "option": None
                },
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "test_report": "All tests accepted"
            },
        ]
    ),
]
