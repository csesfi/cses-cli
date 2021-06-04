from submission import SubmissionScenario, NewSubmission


scenarios = [
    SubmissionScenario(
        NewSubmission(course_id="kurssi", task_id=2,
                      submission_json={
                          "language": {"name": "Rust", "option": None},
                          "filename": "main.rs",
                          "content": "use std::io;\n"
                      }),
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
        NewSubmission(course_id="alon", task_id=4,
                      submission_json={
                          "language": {
                              "name": "C++",
                              "option": "C++17"
                          },
                          "filename": "main.cpp",
                          "content": "#include <iostream>\n"
                      }),
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
        NewSubmission(course_id="cses", task_id=13,
                      submission_json={
                          "language": {
                              "name": "C++",
                              "option": "C++17"
                          },
                          "filename": "13.rs",
                          "content": "use std::io;\n\nfn main() {\n"
                      }),
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
        NewSubmission(course_id="cses", task_id=42,
                      submission_json={
                          "language": {
                              "name": "C++",
                              "option": "C++17"
                          },
                          "filename": "main.cpp",
                          "content": "#include <iostream>\n"
                      }),
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
    )
]
