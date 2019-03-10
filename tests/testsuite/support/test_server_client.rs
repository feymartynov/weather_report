use std::sync::Mutex;

use iron::Listening;
use reqwest::Client;
use serde::de::DeserializeOwned;

use weather_report::config;

lazy_static! {
    static ref PORT_MUTEX: Mutex<u16> = Mutex::new(3000);
    pub static ref CLIENT: TestClient = TestClient::new(TestServer::new());
}

pub struct TestServer {
    listening: Listening,
}

impl TestServer {
    pub fn new() -> Self {
        let mut port = PORT_MUTEX.lock().unwrap();
        *port += 1; // Increment port number in mutex to avoid conflict in parallel tests.

        let address = format!("0.0.0.0:{}", port);
        let server_config = Some(config::Server { address });
        let listening = weather_report::start_server(server_config);
        Self { listening }
    }

    /// Base URL endpoint of the server.
    pub fn base_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.listening.socket.ip(),
            self.listening.socket.port()
        )
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.listening.close().expect("Error closing server");
    }
}

pub struct TestClient {
    server: TestServer,
    client: Client,
}

impl TestClient {
    pub fn new(server: TestServer) -> Self {
        Self {
            server,
            client: Client::new(),
        }
    }

    /// Makes a GET request, asserts the status code and returns deserialized JSON body as `T`.
    pub fn get_json<T: DeserializeOwned>(&self, relative_url: &str, expected_status: u16) -> T {
        let url = format!("{}{}", self.server.base_url(), relative_url);

        let mut response = self
            .client
            .get(&url)
            .send()
            .expect(&format!("Failed to make a GET request to {:?}", url));

        if response.status().as_u16() != expected_status {
            panic!(
                "Expected {} HTTP status code, but got {:?}.\nResponse body:\n{}\n",
                expected_status,
                response.status().as_u16(),
                response.text().unwrap()
            );
        }

        response.json::<T>().expect(&format!(
            "Failed to parse JSON response:\n{}\n",
            response.text().unwrap()
        ))
    }
}
