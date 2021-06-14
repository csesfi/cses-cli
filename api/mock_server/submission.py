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


class SubmissionProgress:
    """A class for representing the progress of a submission"""
    def __init__(self, progress: list):
        self.data, status = [], progress[0].copy()
        for update in progress:
            status.update(update)
            self.data.append(status.copy())


class SubmissionScenario:
    """A class for representing a single possible submission scenario"""
    def __init__(self, submission_info, submission_progress):
        self.submission_info = submission_info
        self.submission_progress = submission_progress


class TestResult:
    """A class for representing the result of a single test"""
    def __init__(self, number=1, verdict="ACCEPTED", time=120):
        self.data = {
            "number": number,
            "verdict": verdict,
            "time": time
        }
