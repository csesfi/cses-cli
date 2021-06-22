use minreq::{Error, Request, Response};

pub trait TraceSend {
    fn trace_send(self) -> Result<Response, Error>;
}

impl TraceSend for Request {
    fn trace_send(self) -> Result<Response, Error> {
        // minreq::Request unfortunately exposes none of its contents, so we
        // can't conveniently print them here
        let result = self.send();
        let trace = if cfg!(debug_assertions) {
            match std::env::var("CSES_CLI_TRACE") {
                Ok(val) => !val.is_empty(),
                Err(_) => false,
            }
        } else {
            false
        };
        if trace {
            if let Ok(response) = &result {
                eprintln!("Response: {}", response.status_code);
                if let Ok(body) = response.as_str() {
                    eprintln!("{}", body);
                }
            }
        }
        result
    }
}
