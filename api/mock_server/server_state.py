import string
import random


random.seed(1337)


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
        self.valid_tokens.remove(token)

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
