from submission import SubmissionScenario, SubmissionProgress, \
                       SubmissionInfo, TestResult
import constants

scenarios = [
    SubmissionScenario(
        SubmissionInfo(course_id="kurssi", task_id=2, submission_json=constants.RUST_CODE),
        SubmissionProgress([
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
                "tests": [TestResult().data]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="alon", task_id=4, submission_json=constants.CPP_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "TESTING",
            },
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "tests": [
                    TestResult().data,
                    TestResult(2, "WRONG ANSWER", 800).data
                ]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=13, submission_json=constants.RS_13_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "COMPILE ERROR",
                "pending": False,
                "compiler": constants.COMPILER_ERROR
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=42, submission_json=constants.CPP_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [TestResult().data]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=111, submission_json=constants.UNKNOWN_CODE_NO_LANGUAGE_NO_OPTION),
        SubmissionProgress([
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
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=444, submission_json=constants.CPP_CODE_NO_LANGUAGE_NO_OPTION),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [TestResult().data]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=555, submission_json=constants.CPP_CODE_NO_LANGUAGE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "WRONG ANSWER",
                "compiler": constants.COMPILER_WARNING,
                "tests": [TestResult().data]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="progress", task_id=7, submission_json=constants.CPP_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "TESTING",
                "compiler": constants.COMPILER_WARNING_OTHER,
                "test_progress": {
                    "finished_tests": 2,
                    "total_tests": 71,
                },
            },
            {
                "test_progress": {
                    "finished_tests": 18,
                    "total_tests": 71,
                },
            },
            {
                "test_progress": {
                    "finished_tests": 35,
                    "total_tests": 71,
                },
            },
            {
                "test_progress": {
                    "finished_tests": 53,
                    "total_tests": 71,
                },
            },
            {
                "status": "READY",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    TestResult().data,
                    TestResult(2, "OUTPUT LIMIT EXCEEDED", 800).data,
                    TestResult(3, "WRONG ANSWER", 314).data,
                    TestResult(4, "TIME LIMIT EXCEEDED", None).data
                ]}
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="progress", task_id=8, submission_json=constants.CPP_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            { },
            { },
            {
                "status": "TESTING",
                "test_progress": {
                    "finished_tests": 0,
                    "total_tests": 10,
                },
            },
            {
                "test_progress": {
                    "finished_tests": 6,
                    "total_tests": 10,
                },
            },
            {
                "test_progress": {
                    "finished_tests": 9,
                    "total_tests": 10,
                },
            },
            { },
            { },
            {
                "test_progress": {
                    "finished_tests": 10,
                    "total_tests": 10,
                },
            },
            {
                "status": "READY",
                "pending": False,
                "result": "OUTPUT LIMIT EXCEEDED",
                "tests": [
                    TestResult().data,
                    TestResult(2, "OUTPUT LIMIT EXCEEDED", 800).data,
                    TestResult(3, "WRONG ANSWER", 314).data,
                    TestResult(4, "TIME LIMIT EXCEEDED", None).data
                ]}
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="cses", task_id=constants.DEFAULT_TASK, submission_json=constants.CPP_CODE),
        SubmissionProgress([
            constants.CPP_PROGRESS_BASE,
            {
                "status": "READY",
                "pending": False,
                "result": "ACCEPTED",
                "tests": [TestResult().data]
            }
        ]).data
    ),
    SubmissionScenario(
        SubmissionInfo(course_id="tira21k", task_id=23, submission_json=constants.PY_CODE),
        SubmissionProgress([
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
        ]).data
    ),
]
