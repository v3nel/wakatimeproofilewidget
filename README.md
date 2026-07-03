# Discord WakaTime Widget

A small Rust app that reads coding stats from WakaTime and pushes them to a Discord application profile widget.

It currently sends these dynamic values to Discord:

- Total coding time
- Coding time for the last 7 days
- Total lines value
- Favorite language
- Human changes
- AI changes
- Favorite editor

> Note: `lines_total` is currently hardcoded in `src/main.rs` as `"13k"`.

## Requirements

Before starting, install or create the following:

- [Rust](https://www.rust-lang.org/tools/install)
- A [WakaTime](https://wakatime.com/) account and API key
- A Discord application with a bot token
- Your Discord user ID

## Setup tutorial

### 1. Clone the repository

```sh
git clone https://github.com/v3nel/
cd discordwakatimewidget
```

If you already have the repo locally, just open a terminal in the project folder.

### 2. Install Rust

Check that Rust is installed:

```sh
rustc --version
cargo --version
```

If those commands fail, install Rust from <https://www.rust-lang.org/tools/install>.

### 3. Create your environment file

Copy the example environment file:

```sh
cp .env.example .env
```

On Windows PowerShell, you can use:

```powershell
Copy-Item .env.example .env
```

Then open `.env` and replace the placeholder values with your real tokens and IDs.

### 4. Get your WakaTime API key

1. Go to <https://wakatime.com/settings/api-key>.
2. Copy your API key.
3. Put it in `.env` as `WAKATIME_API_KEY`.

The code sends this value in the `Authorization: Basic ...` header. If WakaTime rejects the raw key, use the base64-encoded version of your WakaTime API key.

### 5. Create a Discord application and bot

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications).
2. Create or select an application.
3. Copy the application ID and set it as `DISCORD_APP_ID` in `.env`.
4. Open the **Bot** page.
5. Create/reset the bot token and set it as `BOT_TOKEN` in `.env`.

Keep your bot token private. Do not commit `.env`.

### 6. Get your Discord user ID

1. Open Discord.
2. Go to **User Settings > Advanced**.
3. Enable **Developer Mode**.
4. Right-click your user profile and choose **Copy User ID**.
5. Set that value as `DISCORD_USER_ID` in `.env`.

### 7. Optional GitHub token

`GITHUB_TOKEN` is required by the current config loader, but the GitHub provider is not currently used by the running code.

You can create a token at <https://github.com/settings/tokens>, or temporarily set a placeholder value if you are only using the existing WakaTime/Discord flow.

### 8. Run the app

```sh
cargo run
```

The app will:

1. Load variables from `.env`.
2. Fetch your WakaTime total coding time.
3. Fetch your WakaTime coding time for the last 7 days.
4. Fetch your WakaTime last 7 days stats.
5. Patch your Discord application profile widget data.

You should also see some values printed in the terminal before the Discord update happens.

## Environment variables

| Variable | Required | Description |
| --- | --- | --- |
| `WAKATIME_API_KEY` | Yes | WakaTime API key used for WakaTime requests. |
| `GITHUB_TOKEN` | Yes | GitHub token loaded by config. Currently not used by the main flow. |
| `DISCORD_APP_ID` | Yes | Discord application ID. |
| `BOT_TOKEN` | Yes | Discord bot token used in the `Authorization` header. |
| `DISCORD_USER_ID` | Yes | Discord user ID whose application profile is updated. |

## Development

Format the code:

```sh
cargo fmt
```

Check the project:

```sh
cargo check
```

Run the project:

```sh
cargo run
```

## Troubleshooting

### `missing environment variable: ...`

Your `.env` file is missing one of the required variables. Compare your `.env` with `.env.example`.

### WakaTime returns an authorization error

Check that `WAKATIME_API_KEY` is correct. The current code sends it as a Basic auth value. If the raw key does not work, try base64 encoding it before putting it in `.env`.

### Discord returns an authorization error

Check that:

- `BOT_TOKEN` is correct.
- `DISCORD_APP_ID` is your application ID, not your bot user ID.
- `DISCORD_USER_ID` is your personal Discord user ID.

### Nothing updates in Discord

Make sure the Discord application/profile widget data you are trying to update supports the dynamic field names used in `src/provider/discord/push_to_widget.rs`.
