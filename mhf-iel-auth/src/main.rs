use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input, Password, Select};
use mhf_iel::MhfConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mhf-iel-auth",
    about = "MHF Server Authentication Tool - Fetch config from server and launch game"
)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "http://127.0.0.1:8080",
        help = "Server endpoint URL"
    )]
    server: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login with existing account
    Login {
        #[arg(short, long, help = "Username (will prompt if not provided)")]
        username: Option<String>,
        #[arg(short, long, help = "Password (will prompt if not provided)")]
        password: Option<String>,
    },
    /// Register new account
    Register {
        #[arg(short, long, help = "Username (will prompt if not provided)")]
        username: Option<String>,
        #[arg(short, long, help = "Password (will prompt if not provided)")]
        password: Option<String>,
    },
}

enum CharSelection {
    Existing(u32),
    New(Character),
}

#[derive(Debug, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginResponse {
    #[serde(rename = "currentTs")]
    current_ts: u32,
    #[serde(rename = "expiryTs")]
    expiry_ts: u32,
    #[serde(rename = "entranceCount")]
    entrance_count: u32,
    notices: Vec<Notice>,
    user: User,
    characters: Vec<Character>,
    #[serde(rename = "mezFes")]
    mez_fes: MezFes,
    #[serde(rename = "patchServer")]
    patch_server: String,
}

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "tokenId")]
    token_id: u32,
    token: String,
    rights: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct Character {
    id: u32,
    name: String,
    #[serde(rename = "isFemale")]
    is_female: bool,
    weapon: u32,
    hr: u32,
    gr: u32,
    #[serde(rename = "lastLogin")]
    last_login: i32,
}

#[derive(Debug, Deserialize)]
struct Notice {
    flags: u16,
    data: String,
}

#[derive(Debug, Deserialize)]
struct MezFes {
    id: u32,
    start: u32,
    end: u32,
    #[serde(rename = "soloTickets")]
    solo_tickets: u32,
    #[serde(rename = "groupTickets")]
    group_tickets: u32,
    stalls: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CreateCharRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct CreateCharResponse {
    id: u32,
    name: String,
    #[serde(rename = "isFemale")]
    is_female: bool,
    weapon: u32,
    hr: u32,
    gr: u32,
    #[serde(rename = "lastLogin")]
    last_login: i32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or_else(|| {
        // Default to interactive mode if no command specified
        let choices = vec!["Login", "Register"];
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&choices)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => Commands::Login {
                username: None,
                password: None,
            },
            1 => Commands::Register {
                username: None,
                password: None,
            },
            _ => unreachable!(),
        }
    });

    match command {
        Commands::Login { username, password } => {
            authenticate(&cli.server, "login", username, password)?;
        }
        Commands::Register { username, password } => {
            authenticate(&cli.server, "register", username, password)?;
        }
    }

    Ok(())
}

fn authenticate(
    server: &str,
    action: &str,
    username: Option<String>,
    password: Option<String>,
) -> Result<()> {
    // Get credentials
    let username = match username {
        Some(u) => u,
        None => Input::new()
            .with_prompt("Username")
            .interact_text()
            .context("Failed to read username")?,
    };

    let password = match password {
        Some(p) => p,
        None => Password::new()
            .with_prompt("Password")
            .interact()
            .context("Failed to read password")?,
    };

    // Send authentication request
    println!("Connecting to {}...", server);
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/{}", server, action);

    let response = client
        .post(&url)
        .json(&LoginRequest { username, password })
        .send()
        .context(format!("Failed to connect to {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Authentication failed: {} - {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let login_data: LoginResponse = response
        .json()
        .context("Failed to parse server response")?;

    println!("Authentication successful!");
    println!("Token: {}", login_data.user.token);

    // Character selection
    let (char_id, char_data) = if login_data.characters.is_empty() {
        println!("\nNo characters found. Creating new character...");
        let new_char = create_character(server, &login_data.user.token)?;
        (new_char.id, new_char)
    } else {
        let result = select_or_create_character(server, &login_data.user.token, &login_data.characters)?;
        match result {
            CharSelection::Existing(id) => {
                let char_data = login_data.characters.iter().find(|c| c.id == id).unwrap().clone();
                (id, char_data)
            }
            CharSelection::New(char_data) => (char_data.id, char_data),
        }
    };

    // Build config
    let config = build_config(server, &login_data, char_id, &char_data)?;

    // Save config to file
    let config_path = PathBuf::from("config.json");
    let config_json = serde_json::to_string_pretty(&serde_json::to_value(&config)?)?;
    std::fs::write(&config_path, config_json)
        .context("Failed to write config.json")?;

    println!("\n✓ Configuration saved to config.json");
    println!("✓ You can now run: mhf-iel-cli.exe");

    Ok(())
}

fn select_or_create_character(
    server: &str,
    token: &str,
    characters: &[Character],
) -> Result<CharSelection> {
    println!("\nAvailable characters:");

    let mut choices: Vec<String> = characters
        .iter()
        .map(|c| format!("{} (ID: {})", c.name, c.id))
        .collect();
    choices.push("Create new character".to_string());

    let selection = Select::new()
        .with_prompt("Select a character")
        .items(&choices)
        .default(0)
        .interact()
        .context("Failed to read character selection")?;

    if selection == characters.len() {
        // Create new character
        let new_char = create_character(server, token)?;
        Ok(CharSelection::New(new_char))
    } else {
        Ok(CharSelection::Existing(characters[selection].id))
    }
}

fn create_character(server: &str, token: &str) -> Result<Character> {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/character/create", server);

    let response = client
        .post(&url)
        .json(&CreateCharRequest {
            token: token.to_string(),
        })
        .send()
        .context("Failed to create character")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to create character: {} - {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    let char_data: CreateCharResponse = response
        .json()
        .context("Failed to parse character creation response")?;

    println!("Character created - ID: {}, Name: {}", char_data.id, char_data.name);

    // Convert CreateCharResponse to Character
    Ok(Character {
        id: char_data.id,
        name: char_data.name,
        is_female: char_data.is_female,
        weapon: char_data.weapon,
        hr: char_data.hr,
        gr: char_data.gr,
        last_login: char_data.last_login,
    })
}

fn build_config(
    server: &str,
    login_data: &LoginResponse,
    char_id: u32,
    char_data: &Character,
) -> Result<MhfConfig> {
    // Parse server host and port from URL
    let url = reqwest::Url::parse(server).context("Invalid server URL")?;
    let server_host = url
        .host_str()
        .context("No host in server URL")?
        .to_string();
    // Default to signserver port (53312) if not specified in API URL
    let server_port = url.port().unwrap_or(53312);

    // Convert server notices to mhf_iel format
    let notices: Vec<mhf_iel::Notice> = login_data
        .notices
        .iter()
        .map(|n| mhf_iel::Notice {
            flags: n.flags,
            data: n.data.clone(),
        })
        .collect();

    // Convert mez_fes stalls from String to MezFesStall enum
    let mez_stalls: Vec<mhf_iel::MezFesStall> = login_data
        .mez_fes
        .stalls
        .iter()
        .filter_map(|s| match s.as_str() {
            "TokotokoPartnya" => Some(mhf_iel::MezFesStall::TokotokoPartnya),
            "Pachinko" => Some(mhf_iel::MezFesStall::Pachinko),
            "VolpakkunTogether" => Some(mhf_iel::MezFesStall::VolpakkunTogether),
            "GoocooScoop" => Some(mhf_iel::MezFesStall::GoocooScoop),
            "Nyanrendo" => Some(mhf_iel::MezFesStall::Nyanrendo),
            "HoneyPanic" => Some(mhf_iel::MezFesStall::HoneyPanic),
            "DokkanBattleCats" => Some(mhf_iel::MezFesStall::DokkanBattleCats),
            "PointStall" => Some(mhf_iel::MezFesStall::PointStall),
            "StallMap" => Some(mhf_iel::MezFesStall::StallMap),
            _ => None,
        })
        .collect();

    // Get all character IDs
    let char_ids: Vec<u32> = login_data.characters.iter().map(|c| c.id).collect();

    // Build config from server response
    let config = MhfConfig {
        char_id,
        char_name: char_data.name.clone(),
        char_new: false,
        char_hr: char_data.hr,
        char_gr: char_data.gr,
        char_ids,
        user_token_id: login_data.user.token_id,
        user_token: login_data.user.token.clone(),
        user_name: String::from(""), // Not provided by API
        user_password: String::from(""), // Not provided by API
        user_rights: login_data.user.rights,
        server_host,
        server_port: server_port as u32,
        entrance_count: login_data.entrance_count,
        current_ts: login_data.current_ts,
        expiry_ts: login_data.expiry_ts,
        notices,
        mez_event_id: login_data.mez_fes.id,
        mez_start: login_data.mez_fes.start,
        mez_end: login_data.mez_fes.end,
        mez_solo_tickets: login_data.mez_fes.solo_tickets,
        mez_group_tickets: login_data.mez_fes.group_tickets,
        mez_stalls,
        version: mhf_iel::MhfVersion::ZZ,
        mhf_folder: None,
        mhf_flags: None,
    };

    Ok(config)
}
