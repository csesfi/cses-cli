# Task number
DEFAULT_TASK = 34

# Submitter
UOLEVI = {
    "id": 1,
    "username": "mooc~123",
    "displayname": "uolevi@cses.fi (mooc.fi)"
}

# Submission data values
RUST_CODE = {
    "language": {"name": "Rust", "option": None},
    "filename": "main.rs",
    "content": "use std::io;\n"
}
CPP_CODE = {
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
CPP_CODE_NO_LANGUAGE_NO_OPTION = {
    "language": {
        "name": None,
        "option": None
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
UNKNOWN_CODE_NO_LANGUAGE_NO_OPTION = {
    "language": {
        "name": None,
        "option": None
    },
    "filename": "main.asdf",
    "content": "#include <iostream>\n"
}
CPP_CODE_NO_LANGUAGE = {
    "language": {
        "name": None,
        "option": "C++17"
    },
    "filename": "main.cpp",
    "content": "#include <iostream>\n"
}
RS_13_CODE = {
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "filename": "13.rs",
    "content": "use std::io;\n\nfn main() {\n"
}
PY_TODO_CODE = {
    "language": {"name": "CPython", "option": None},
    "filename": "todo.py",
    "content": "def check(n):\n    # TODO\n"
}
PY_CODE = {
    "language": {"name": "CPython", "option": None},
    "filename": "lucky.py",
    "content": "def check(n):\n    s = 0\n"
}

# Task information
SUMMA_TASK = {
    "id": 123,
    "name": "Summa",
}

# Base progress information
CPP_PROGRESS_BASE = {
    "task": SUMMA_TASK,
    "sender": UOLEVI,
    "time": "2017-07-21T17:32:28Z",
    "language": {
        "name": "C++",
        "option": "C++17"
    },
    "status": "PENDING",
    "pending": True,
}

# Compiler messages
COMPILER_ERROR = """input/code.cpp:1:1: error: 'use' does not name \
a type
use std::io;
^~~
input/code.cpp:3:1: error: 'fn' does not name a type
fn main() {
^~
"""
COMPILER_WARNING = """input/code.cpp: In function 'int main()':
input/code.cpp:27:29: warning: comparison between signed and unsigned integer \
expressions [-Wsign-compare]
for (int i = 0; i < a.size(); i++) {
"""
COMPILER_WARNING_OTHER = """input/code.cpp: In function 'int main()':
input/code.cpp:3:11: warning: 'x' is used uninitialized in this function \
[-Wuninitialized]
   while(x != 123);
         ~~^~~~~~"""