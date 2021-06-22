import base64


class Template:
    def __init__(self, has_token, scope_id, task_id, language, file):
        self.has_token = has_token
        self.scope_id = scope_id
        self.task_id = task_id
        self.language = language
        self.file = file

    def __eq__(self, other):
        return self.has_token == other.has_token \
            and self.scope_id == other.scope_id \
            and self.task_id == other.task_id \
            and self.language == other.language \
            and self.file == other.file

    def __hash__(self):
        return hash((self.has_token, self.scope_id, self.task_id,
                     self.language, self.file))


def _to_b64(string):
    return base64.b64encode(string.encode()).decode()


TEMPLATES = {
    Template(True, "cses", None, None, None): {
        "template_source": _to_b64("code1"),
        "filename": "code1"
    },
    Template(False, "cses", "1", "Rust", None): {
        "template_source": _to_b64("rust1"),
        "filename": "rust1.rs"
    },
    Template(True, "cses", "1", "Rust", None): {
        "template_source": _to_b64("rust2"),
        "filename": "rust2.rs"
    },
    Template(False, "cses", None, None, "rust3.rs"): {
        "template_source": _to_b64("rust3"),
        "filename": "rust3.rs"
    }
}
