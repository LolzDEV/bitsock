use std::{
    fmt::{self},
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::Mutex,
};

use crate::{LogLevel, LogStage, Packet, ReadingError};

/// Error type for handling physical server errors.
///
/// ```
/// // Using ServerError in a custom handler
///
/// fn handle_server_errors(error: ServerError) {
///     eprintln!("[SERVER][ERROR]: {}", error);
///     
///     // do some other logic...
///
/// }
///
/// ```
#[derive(Clone, Debug)]
pub struct ServerError(String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Logical client data structure.
pub struct LogicalClient {
    address: String,
    stream: TcpStream,
}

impl LogicalClient {
    /// Send a [Packet] to the client.
    pub fn send(&mut self, packet: Packet) -> Result<usize, std::io::Error> {
        let size = self.stream.write(packet.encode().as_slice())?;
        Ok(size)
    }

    /// Listen to a [Packet] from the client.
    pub fn read(&mut self) -> Result<Packet, ReadingError> {
        let mut data = [0; 64];

        match self.stream.read(&mut data) {
            Ok(_) => {
                if let Ok(packet) = Packet::decode(data.to_vec()) {
                    Ok(packet)
                } else {
                    Err(ReadingError::Decode)
                }
            }
            Err(_) => Err(ReadingError::Reading),
        }
    }

    /// Get the address of the client.
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// Close the connection with the client.
    pub fn disconnect(&self) -> Result<(), std::io::Error> {
        self.stream.shutdown(Shutdown::Both)?;

        Ok(())
    }
}

/// Physical server data structure.
pub struct Server<'a> {
    pub address: &'a str,
    pub port: u16,
    listener: Option<TcpListener>,
    error_handler: Option<Box<dyn Fn(ServerError) -> () + Send + Sync>>,
    client_handler: Box<dyn Fn(LogicalClient) -> () + Send + Sync>,
    log_handler: Option<Box<dyn Fn(LogStage, LogLevel, &str) + Send + Sync>>,
}

impl<'a> Server<'a> {
    /// Creates a new physical server object. Is recommended to use [ServerBuilder] for more customization.
    pub fn new(address: &'a str, port: u16) -> Self {
        Self {
            address,
            port,
            listener: None,
            error_handler: None,
            client_handler: Box::new(|c| println!("{} connected.", c.address())),
            log_handler: None,
        }
    }

    /// Start the server execution, this will start a loop.
    pub fn run(&mut self) {
        self.log(LogLevel::INFO, "Starting server");

        self.listener =
            if let Ok(listener) = TcpListener::bind(format!("{}:{}", self.address, self.port)) {
                Some(listener)
            } else {
                self.handle_error(ServerError(format!(
                    "failed to bind listener to address {}:{}",
                    self.address, self.port,
                )));
                None
            };

        let handler = Mutex::new(&self.client_handler);

        if let Err(_) = crossbeam::thread::scope(|s| {
            s.spawn(|_| {
                if let Some(listener) = &self.listener {
                    self.log(LogLevel::INFO, "Server started, listening for connections.");

                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                let client = LogicalClient {
                                    address: stream.local_addr().unwrap().to_string(),
                                    stream,
                                };
                                handler.lock().unwrap()(client);
                            }
                            Err(e) => {
                                self.handle_error(ServerError(format!("Connection failed: {}", e)))
                            }
                        }
                    }
                }
            });
        }) {
            self.handle_error(ServerError("Failed to spawn listener thread".to_string()));
        }
    }

    /// Internal function, used to handle errors propagated by the server.
    /// You can also use a custom handler specifing it when building the physical server (see [ServerBuilder::error_handler]).
    fn handle_error(&self, error: ServerError) {
        if let Some(handler) = &self.error_handler {
            handler(error);
        } else {
            println!("{}", error);
        }
    }

    /// Log a message from the physical server.
    pub fn log(&self, level: LogLevel, message: &str) {
        if let Some(handler) = &self.log_handler {
            handler(LogStage::SERVER, level, message);
        } else {
            println!("[SERVER][{:?}]: {}", level, message);
        }
    }
}

/// Server builder object.
/// Can be used to create [Server] objects in a convenient and flexible way.
/// ```
/// //e.g.
/// fn main() {
///     let server = ServerBuilder::new().address("192.168.1.151").port(4353).build();
/// }
/// ```
pub struct ServerBuilder<'a> {
    address: &'a str,
    port: u16,
    error_handler: Option<Box<dyn Fn(ServerError) -> () + Send + Sync>>,
    client_handler: Box<dyn Fn(LogicalClient) -> () + Send + Sync>,
    log_handler: Option<Box<dyn Fn(LogStage, LogLevel, &str) + Send + Sync>>,
}

impl<'a> ServerBuilder<'a> {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            address: "0.0.0.0",
            port: 4444,
            error_handler: None,
            client_handler: Box::new(|c| println!("{} connected.", c.address())),
            log_handler: None,
        }
    }

    /// Sets the server address.
    pub fn address(mut self, address: &'a str) -> Self {
        Self {
            address: address,
            port: self.port,
            error_handler: std::mem::replace(&mut self.error_handler, None),
            client_handler: self.client_handler,
            log_handler: std::mem::replace(&mut self.log_handler, None),
        }
    }

    /// Sets the server port.
    pub fn port(mut self, port: u16) -> Self {
        Self {
            address: self.address,
            port: port,
            error_handler: std::mem::replace(&mut self.error_handler, None),
            client_handler: self.client_handler,
            log_handler: std::mem::replace(&mut self.log_handler, None),
        }
    }

    /// Sets the server `error handler`
    pub fn error_handler(mut self, handler: Box<dyn Fn(ServerError) -> () + Send + Sync>) -> Self {
        Self {
            address: self.address,
            port: self.port,
            error_handler: Some(handler),
            client_handler: self.client_handler,
            log_handler: std::mem::replace(&mut self.log_handler, None),
        }
    }

    /// Sets the server `client handler`
    pub fn client_handler(
        mut self,
        handler: Box<dyn Fn(LogicalClient) -> () + Send + Sync>,
    ) -> Self {
        Self {
            address: self.address,
            port: self.port,
            error_handler: std::mem::replace(&mut self.error_handler, None),
            client_handler: handler,
            log_handler: std::mem::replace(&mut self.log_handler, None),
        }
    }

    /// Sets the server `logger`
    pub fn log_handler(
        mut self,
        handler: Box<dyn Fn(LogStage, LogLevel, &str) + Send + Sync>,
    ) -> Self {
        Self {
            address: self.address,
            port: self.port,
            error_handler: std::mem::replace(&mut self.error_handler, None),
            client_handler: self.client_handler,
            log_handler: Some(handler),
        }
    }

    /// Build the server object.
    pub fn build(mut self) -> Server<'a> {
        Server {
            address: self.address,
            port: self.port,
            listener: None,
            error_handler: std::mem::replace(&mut self.error_handler, None),
            client_handler: self.client_handler,
            log_handler: std::mem::replace(&mut self.log_handler, None),
        }
    }
}
