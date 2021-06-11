import random
import string


random.seed(1337)


class ServerState:
    def __init__(self, integration, scenarios):
        # tokens need to be saved to test the logout
        self.integration = integration
        self.pending_tokens = []
        self.valid_tokens = []
        self.submission_scenarios = scenarios

        self.submission_trackers = {}

    def login(self):
        token = self._generate_token()
        self.pending_tokens.append(token)
        return token

    def authorize_login(self, token, fail):
        assert token in self.pending_tokens
        self.pending_tokens.remove(token)
        if not fail:
            self.valid_tokens.append(token)

    def authorize_all(self):
        """Authorizes all pending tokens. Used only with integration testing"""
        for token in self.pending_tokens:
            self.valid_tokens.append(token)
        self.pending_tokens = []

    def check_login(self, token):
        if self.is_valid(token):
            return "valid"
        else:
            if token in self.pending_tokens:
                return "pending"
            else:
                return "invalid"

    def logout(self, token):
        """Logs out a valid api key"""
        assert self.is_valid(token)
        if token in self.pending_tokens:
            self.pending_tokens.remove(token)
        if token in self.valid_tokens:
            self.valid_tokens.remove(token)

    def is_valid(self, token):
        return token in self.valid_tokens or \
            (not self.integration and token in self.pending_tokens)

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

    def get_initial_submission_info(self, submission_id):
        """Returns the initial state of the submission `submission_id`"""
        if submission_id not in self.submission_trackers:
            return None
        info = self.submission_trackers[submission_id].first().copy()
        info["id"] = submission_id
        return info

    def get_submission_info(self, submission_id):
        """Returns the next state of the submission `submission_id`"""
        if submission_id not in self.submission_trackers:
            return None
        info = self.submission_trackers[submission_id].next().copy()
        info["id"] = submission_id
        return info


class SubmissionTracker:
    def __init__(self, submission_infos):
        assert len(submission_infos) > 0
        self.infos = submission_infos
        self._position = 0

    def first(self):
        return self.infos[0]

    def next(self):
        info = self.infos[self._position]
        if self._position + 1 < len(self.infos):
            self._position += 1
        return info
