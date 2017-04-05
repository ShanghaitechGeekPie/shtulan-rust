use hyper::error::Error;
use hyper::client::Response;
use hyper::status::StatusCode;
use hyper::header::Headers;


const POST_URL: &'static str = "https://controller.shanghaitech.edu.cn:8445/PortalServer/Webauth/webAuthAction!login.action";

pub struct Auth<'a>{
    username: &'a str,
    password: &'a str,
    headers: Headers,
}

impl<'a> Auth<'a>{
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Auth{
            username: username,
            password: password,
            headers: Self::build_header(),
        }
    }

    fn to_url_params(&self) -> String {
        use url::form_urlencoded;
        form_urlencoded::Serializer::new(String::new())
        .append_pair("userName", self.username)
        .append_pair("password", self.password)
        .append_pair("hasValidateCode", "false")
        .append_pair("authLan", "zh_CN")
        .finish()
    }

    fn build_header() -> Headers {
        use hyper::header::{Headers, ContentType, Cookie, Accept, qitem};
        use hyper::mime::{Mime, TopLevel, SubLevel};
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![])));
        headers.set(Accept(vec![qitem(Mime(TopLevel::Star, SubLevel::Star, vec![]))]));
        headers.set(Cookie(vec![String::from("JSESSIONID=s5rhnfbl2umgz8ev01to7cdwpkyq93j4")]));
        headers
    }

    fn send_request(&self) -> Result<Response, Error>{
        use hyper::client::Client;
        use hyper::net::HttpsConnector;
        use hyper_native_tls::NativeTlsClient;
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        client.post(POST_URL)
            .body(&self.to_url_params())
            .headers(self.headers.clone())
            .send()
    }

    fn parse_response(mut response: Response){
        use std::io::Read;
        use std::str::from_utf8;
        if let StatusCode::Ok = response.status {
            let mut buf = vec![];
            match response.read_to_end(&mut buf){
                Ok(_) => {
                    if let Ok(text) = from_utf8(buf.as_slice()){
                        match Self::parse_result(text) {
                            Ok(ip) => {
                                println!("Login successfully. Your ip is {}", ip);
                            }
                            Err(e) => {
                                println!("Server error: {:?}. Text returned by server: {}", e, text);
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("Read Error: {:?}", e);
                }
            }
        } else {
            println!("Bad status: {:?}", response.status);
        }
    }

    fn parse_result(response: &str) -> Result<String, LoginError> {
        use rustc_serialize::json::Json;
        if let Ok(root) = Json::from_str(response) {
            if let Some(obj_root) = root.as_object() {
                if let Some(success) = obj_root.get("success") {
                    if success.as_boolean().unwrap_or(false) {
                        if let Some(data) = obj_root.get("data") {
                            if let Some(obj_data) = data.as_object() {
                                if let Some(ip) = obj_data.get("ip") {
                                    return Ok(ip.as_string().unwrap_or("0.0.0.0").to_string());
                                }
                            }
                        }
                    }

                }
            }
        }
        return Err(LoginError::ParseError);
    }

    pub fn login(&self) {
        let response = self.send_request();
        match response{
            Ok(response) => {
                Self::parse_response(response);
            }
            Err(e) => {
                println!("Connection Failed: {:?}",e);
            }
        }
    }
}


#[derive(Debug)]
enum LoginError {
    ParseError,
    Msg(String),
}
