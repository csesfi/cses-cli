class SubmissionInfo:
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
    def __init__(self, submission_info, submission_updates):
        self.submission_info = submission_info
        self.submission_progress = submission_updates


def test_progress(finished_tests: int, total_tests: int) -> dict:
    return {
        "test_progress": {
            "finished_tests": finished_tests,
            "total_tests": total_tests
        }
    }


def test_result(number: int = 1, verdict: str = "ACCEPTED", time: int = 120) \
        -> dict:
    return {
        "number": number,
        "verdict": verdict,
        "time": time
    }


def submission_progress(progress: list) -> dict:
    data, status = [], progress[0].copy()
    for update in progress:
        status.update(update)
        data.append(status.copy())
    return data
