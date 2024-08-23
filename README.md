## Easy SSH

### Overview

Easy SSH is a command-line tool for managing SSH connections to multiple servers. It allows you to register servers with their SSH details, list registered servers, run commands on them, and print the SSH command for a registered server.

### Installation

To install Easy SSH, you can use the provided installation script. Run the following commands in your terminal:

```sh
# Build the project
cargo build --release

chmod +x install.sh

# Strip the binary to reduce size
# Move the binary to /usr/local/bin
sudo ./install.sh
```

### Usage

#### Register a Server

To register a new server, use the `register` command:

```sh
easy-ssh register <server-name> <user@ip> <key-path>
```

- `<server-name>`: A unique name for the server.
- `<user@ip>`: The user and IP address of the server.
- `<key-path>`: The path to the SSH key.

#### Run a Command on a Registered Server

To run a command on a registered server, use the `run` command:

```sh
easy-ssh run <server-name>
```

- `<server-name>`: The name of the registered server.

#### List All Registered Servers

To list all registered servers, use the `ls` command:

```sh
easy-ssh ls
```

#### Remove a Registered Server

To remove a registered server, use the `remove` command:

```sh
easy-ssh remove <server-name>
```

- `<server-name>`: The name of the registered server to remove.

#### Print the SSH Command for a Registered Server

To print the SSH command for a registered server, use the `print` command:

```sh
easy-ssh print <server-name>
```

- `<server-name>`: The name of the registered server.

You can then pipe this output to `bash` to execute it:

```sh
easy-ssh print <server-name> | bash
```