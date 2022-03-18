#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Uninitialized,
}

// conver string to Httprequest
impl From<&str> for Method {
  fn from(s: &str) -> Method {
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Version {
  V1_1,
  V2_0,
  Uninitialized,
}

// conver string to Httprequest
impl From<&str> for Version {
  fn from(s: &str) -> Version {
    match s {
      "HTTP/1.1" => Version::V1_1,
      "HTTP/2.0" => Version::V2_0,
      _ => Version::Uninitialized,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_into() {
    let m: Method = "GET".into();
    assert_eq!(m, Method::Get)
  }


  #[test]
  fn test_version_into() {
    let v: Version = "HTTP/1.1".into();
    assert_eq!(v, Version::V1_1)
  }
}
