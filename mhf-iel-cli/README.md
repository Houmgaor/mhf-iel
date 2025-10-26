# MHF IELess Launcher CLI

Command-Line Interface for `mhf-iel`.

## Usage

1. Get a `mhf-iel-cli.exe` file by either [compiling the project](../README.md) or downloading the [latest release](https://github.com/rockisch/mhf-iel/releases/).
2. Download [`config.example.json`](config.example.json).

Go back to your MHF launcher:

1. Copy `config.examples.json` to `config.json`, edit it to match your Frontier configuration. Specifically, make sure the `char_*` keys and `user_token` have correct values.
2. Copy both `config.json` and `mhf-iel-cli.exe` to the MHF folder.
3. Run `mhf-iel-cli.exe`.

If you plan on using the CLI interface as the entrypoint of your external application, run `mhf-iel-cli.exe --help` to see some extra options available.
