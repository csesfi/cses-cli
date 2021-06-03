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
