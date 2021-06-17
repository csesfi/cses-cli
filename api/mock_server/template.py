import base64
class Template:
    def __init__(self, has_token, course_id, task_id, language, file):
        self.has_token = has_token
        self.course_id = course_id
        self.task_id = task_id
        self.language = language
        self.file = file

    def __eq__(self, other):
        return self.has_token == other.has_token \
            and self.course_id == other.course_id \
            and self.task_id == other.task_id \
            and self.language == other.language \
            and self.file == other.file

    def __hash__(self):
        return hash((self.has_token, self.course_id, self.task_id, \
            self.language, self.file))

def _to_b64(s):
    return base64.b64encode(s.encode()).decode()

TEMPLATES = {
    Template(True, "cses", None, None, None): {"template_source": _to_b64("code1"), "file_name": "code1"},
    Template(False, "cses", 1, "Rust", None): {"template_source": _to_b64("rust1"), "file_name": "rust1.rs"},
    Template(True, "cses", 1, "Rust", None): {"template_source": _to_b64("rust2"), "file_name": "rust2.rs"},
    Template(False, "cses", None, None, "rust3.rs"): {"template_source": _to_b64("rust3"), "file_name": "rust3.rs"}
}
