# MHF IEL Auth - Server Authentication Tool

A user-friendly Rust CLI tool that fetches configuration from your MHF server and generates `config.json` for the game launcher.

## Features

- Interactive CLI interface with prompts
- Login to existing account or register new account
- Character selection menu
- Automatic character creation
- Generates ready-to-use `config.json`
- No Python dependencies required
- Cross-platform (Windows/Linux)

## Usage

### Interactive Mode (Recommended)

Simply run without arguments for an interactive experience:

```bash
mhf-iel-auth
```

You'll be prompted to:

1. Choose Login or Register
2. Enter username and password
3. Select an existing character or create a new one
4. Config.json will be generated automatically

### Command Line Mode

#### Login with existing account

```bash
mhf-iel-auth login
# Or with credentials:
mhf-iel-auth login --username player --password secret
```

#### Register new account

```bash
mhf-iel-auth register
# Or with credentials:
mhf-iel-auth register --username newplayer --password secret
```

### Custom Server

By default, it connects to `http://127.0.0.1:8080`. To use a different server:

```bash
mhf-iel-auth --server http://your-server.com:8080 login
```

## Build

```bash
# For Linux
cargo build --package mhf-iel-auth --target x86_64-unknown-linux-gnu --release

# For Windows
cargo xwin build --package mhf-iel-auth --target x86_64-pc-windows-msvc --release
```

## Server API Requirements

Your MHF server must implement these endpoints:

### POST /login or /register

**Request:**

```json
{
  "username": "player",
  "password": "secret"
}
```

**Response:**

```json
{
  "token": "16CharacterToken1",
  "characters": [
    {"id": 1, "name": "Hunter"}
  ]
}
```

### POST /character

**Request:**

```json
{
  "token": "16CharacterToken1"
}
```

**Response:**

```json
{
  "id": 2
}
```

See [SERVER_API.md](../SERVER_API.md) for full API specification.

## Output

The tool generates a `config.json` file that can be used directly with `mhf-iel-cli`:

```json
{
  "char_id": 1,
  "char_name": "Character_1",
  "user_token": "16CharacterToken1",
  "server_host": "127.0.0.1",
  "server_port": 8080,
  "version": "ZZ",
  ...
}
```

After generation, simply run:

```bash
mhf-iel-cli.exe
```

## Why This Instead of Python GUI?

- **Single executable**: No Python installation required
- **Cross-platform**: Works on Windows and Linux
- **Fast**: Compiled Rust is much faster than Python
- **User-friendly**: Interactive prompts guide you through the process
- **Scriptable**: Can be used in automated workflows
- **Native**: No GUI dependencies, works in terminals and SSH sessions
