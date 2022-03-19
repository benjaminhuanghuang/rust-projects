use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::io::prelude::*;

use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};

pub struct Router;

impl Router {
  pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
    match req.method {
      httprequest::Method::Get => match &req.resource {
        httprequest::Resource::Path(s) => {
          let route: Vec<&str> = s.split("/").collect();
          match route[1] {
            "api" => {
              let resq: HttpResponse = WebServiceHandler::handler(&req);
              let _ = resq.send_response(stream);
            }
            _ => {
              let resq: HttpResponse = StaticPageHandler::handler(&req);
              let _ = resq.send_response(stream);
            }
          }
        }
      },
      _ => {
        let resq: HttpResponse = PageNotFoundHandler::handler(&req);
        let _ = resq.send_response(stream);
      }
    }
  }
}
