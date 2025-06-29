# sascp Project aka Simple and Stupid Control Panel

## Overview
The sascp project is a Rust-based web application that provides a command execution interface through a web server built using the Axum framework. Users can execute predefined system commands via a web interface.

> **Disclaimer:** Use this project at your own risk. Executing system commands via a web interface can pose security risks if not properly secured.

## Files

- **src/main.rs**: Contains the main application logic, sets up the web server, defines routes, and handles command execution.
- **Cargo.toml**: Configuration file for the Rust project, specifying package details and dependencies.
- **README.md**: Documentation for the project, including build and run instructions.
- **systemd/sascp.service**: Systemd service configuration to run the application at boot.

## Getting Started

### Prerequisites
- Rust and Cargo installed on your system.
- Access to a Unix-like operating system.

### Building the Project
To build the project, navigate to the project directory and run:

```
cargo build --release
```

### Running the Application
To run the application, use the following command:

```
cargo run
```

The application will start a web server listening on port 16661.

### Setting Up Systemd Service
To run the application as a service, copy the `sascp.service` file to the systemd directory:

```
sudo cp systemd/sascp.service /etc/systemd/system/
```

Then, enable and start the service:

```
sudo systemctl enable sascp
sudo systemctl start sascp
```

### Accessing the Web Interface
Open your web browser and navigate to `http://localhost:16661` to access the command execution interface.

## TODO

- [ ] Add authentication and authorization to secure command execution
- [ ] Implement logging for executed commands and user actions
- [ ] Provide a configuration file for customizable commands
- [ ] Add unit and integration tests
- [ ] Improve error handling and user feedback in the web interface
- [ ] Support HTTPS for secure connections

## License
This project is licensed under the MIT License. See the LICENSE file for details.