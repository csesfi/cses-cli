use minreq::{Error, Request, Response};

pub trait TraceSend {
    fn trace_send(self, trace: bool) -> Result<Response, Error>;
}

impl TraceSend for Request {
    fn trace_send(self, trace: bool) -> Result<Response, Error> {
        // minreq::Request unfortunately exposes none of its contents, so we
        // can't conveniently print them here
        let result = self.send();
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
