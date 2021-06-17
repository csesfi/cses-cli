from submission import SubmissionScenario, submission_progress, \
                       SubmissionInfo, test_result, test_progress
import constants

SCENARIOS = [
    SubmissionScenario(
        SubmissionInfo(course_id="kurssi", task_id=2,
                       submission_json=constants.RUST_CODE),
        submission_progress([
            {
                "task": constants.SUMMA_TASK,
                "sender": constants.UOLEVI,
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "Rust",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
            },
            {
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "tests": [test_result()]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="alon", task_id=4,
                       submission_json=constants.CPP_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "TESTING",
            },
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "tests": [
                    test_result(),
                    test_result(2, "WRONG ANSWER", 800)
                ]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=13,
                       submission_json=constants.RS_13_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "COMPILE ERROR",
                "pending": False,
                "compiler": constants.COMPILER_ERROR
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=42,
                       submission_json=constants.CPP_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [test_result()]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=111,
                       submission_json=constants.UNKNOWN_CODE_NO_DETAILS),
        submission_progress([
            {
                "task": constants.SUMMA_TASK,
                "sender": constants.UOLEVI,
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": None,
                    "option": None
                },
                "status": "READY",
                "result": "INVALID LANGUAGE",
                "pending": False,
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=444,
                       submission_json=constants.CPP_CODE_NO_DETAILS),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [test_result()]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=555,
                       submission_json=constants.CPP_CODE_NO_LANGUAGE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [test_result()]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="progress", task_id=7,
                       submission_json=constants.CPP_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "TESTING",
                "compiler": constants.COMPILER_WARNING_OTHER,
                **test_progress(2, 71)
            },
            test_progress(18, 71),
            test_progress(35, 71),
            test_progress(53, 71),
            {
                "status": "READY",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    test_result(),
                    test_result(2, "OUTPUT LIMIT EXCEEDED", 800),
                    test_result(3, "WRONG ANSWER", 314),
                    test_result(4, "TIME LIMIT EXCEEDED", None)
                ]}
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="progress", task_id=8,
                       submission_json=constants.CPP_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {},
            {},
            {
                "status": "TESTING",
                **test_progress(0, 10)
            },
            test_progress(6, 10),
            test_progress(9, 10),
            {},
            {},
            test_progress(10, 10),
            {
                "status": "READY",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    test_result(),
                    test_result(2, "OUTPUT LIMIT EXCEEDED", 800),
                    test_result(3, "WRONG ANSWER", 314),
                    test_result(4, "TIME LIMIT EXCEEDED", None)
                ]}
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=constants.DEFAULT_TASK,
                       submission_json=constants.CPP_CODE),
        submission_progress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "tests": [test_result()]
            }
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="tira21k", task_id=23,
                       submission_json=constants.PY_CODE),
        submission_progress([
            {
                "task": constants.SUMMA_TASK,
                "sender": constants.UOLEVI,
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "CPython",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
            },
            {
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "test_report": "All tests accepted"
            },
        ])
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="tira21k", task_id=23,
                       submission_json=constants.PY_TODO_CODE),
        submission_progress([
            {
                "task": constants.SUMMA_TASK,
                "sender": constants.UOLEVI,
                "time": "2017-07-21T17:32:28Z",
                "language": {
                    "name": "CPython",
                    "option": None
                },
                "status": "PENDING",
                "pending": True,
            },
            {
                "status": "READY",
                "pending": False,
                "result": "TEST FAILED",
                "test_report": """Test failed when given the following input:
1
Error Message:
SyntaxError: unexpeted EOF while parsing (todo.py, line 3)"""
            },
        ])
    )
]
