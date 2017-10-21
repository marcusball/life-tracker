use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};

error_chain!{
    foreign_links {
        DieselError(::diesel::result::Error);
    }
}

impl<'a> Responder<'a> for Error {
    fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'a>, Status> {
        let mut result = String::new();
        result += &format!("Error: {}", self);

        self.iter()
            .skip(1)
            .map(|chained_error| {
                result += &format!(", caused by: {}", chained_error)
            })
            .collect::<Vec<_>>();

        let response = json!({
            "status": "failure",
            "message": result,
        }).to_string();

        Ok(
            Response::build()
                .status(Status::BadRequest)
                .header(ContentType::JSON)
                .sized_body(Cursor::new(response))
                .finalize(),
        )
    }
}
