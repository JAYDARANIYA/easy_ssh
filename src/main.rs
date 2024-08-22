use clap::{Arg, Command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    name: String,
    user_ip: String,
    key_path: String,
}

fn main() {
    let matches = Command::new("easy-ssh")
        .version("1.0")
        .about("Easy SSH management")
        .subcommand(
            Command::new("register")
                .about("Register a new server")
                .arg(
                    Arg::new("server-name")
                        .help("The name of the server")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("user@ip")
                        .help("The user and IP address of the server")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("key-path")
                        .help("The path to the SSH key")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run a command on a registered server")
                .arg(
                    Arg::new("server-name")
                        .help("The name of the server")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("ls").about("List all registered servers"))
        .subcommand(
            Command::new("remove")
                .about("Remove a registered server")
                .arg(
                    Arg::new("server-name")
                        .help("The name of the server")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("register") {
        let server = Server {
            name: matches
                .get_one::<String>("server-name")
                .unwrap()
                .to_string(),
            user_ip: matches.get_one::<String>("user@ip").unwrap().to_string(),
            key_path: matches.get_one::<String>("key-path").unwrap().to_string(),
        };
        println!("Registering server: {:?}", server);
        let manager = SledManager::default();
        match manager.save_server(&server) {
            Ok(_) => println!("Server registered successfully"),
            Err(e) => println!("Failed to register server: {}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        let server_name = matches.get_one::<String>("server-name").unwrap();
        println!("Running command on server: {}", server_name);
        let manager = SledManager::default();
        let server = match manager.get_server(server_name) {
            Ok(Some(server)) => server,
            _ => {
                println!("Server not found");
                std::process::exit(1); // Exit with a non-zero status code
            }
        };

        // Construct the SSH command
        let status = std::process::Command::new("ssh")
            .arg("-i")
            .arg(&server.key_path)
            .arg(&server.user_ip)
            .status()
            .expect("Failed to execute SSH command");

        // Exit with the status code of the SSH command
        std::process::exit(status.code().unwrap_or(1));
    }

    if let Some(_) = matches.subcommand_matches("ls") {
        let manager = SledManager::default();
        match manager.list_servers() {
            Ok(servers) => {
                for server in servers {
                    println!("{:?}", server);
                }
            }
            Err(e) => println!("Failed to list servers: {}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let server_name = matches.get_one::<String>("server-name").unwrap();
        let manager = SledManager::default();
        match manager.remove_server(server_name) {
            Ok(_) => println!("Server removed successfully"),
            Err(e) => println!("Failed to remove server: {}", e),
        }
    }
}

struct SledManager {
    db: sled::Db,
}

impl Default for SledManager {
    fn default() -> Self {
        // open the db in HOME directory
        let db = sled::open(std::env::var("HOME").unwrap() + "/.easy-ssh.db").unwrap();
        Self { db }
    }
}

// save server and get server
impl SledManager {
    fn save_server(&self, server: &Server) -> Result<(), Box<dyn std::error::Error>> {
        self.db
            .insert(server.name.as_bytes(), serde_json::to_vec(server)?)?;
        Ok(())
    }

    fn get_server(&self, name: &str) -> Result<Option<Server>, Box<dyn std::error::Error>> {
        let value = self.db.get(name.as_bytes())?;
        if let Some(value) = value {
            let server: Server = serde_json::from_slice(&value)?;
            return Ok(Some(server));
        }
        Ok(None)
    }

    fn list_servers(&self) -> Result<Vec<Server>, Box<dyn std::error::Error>> {
        let mut servers = Vec::new();
        for item in self.db.iter() {
            let (_, value) = item?;
            let server: Server = serde_json::from_slice(&value)?;
            servers.push(server);
        }
        Ok(servers)
    }

    fn remove_server(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.remove(name.as_bytes())?;
        Ok(())
    }
}
