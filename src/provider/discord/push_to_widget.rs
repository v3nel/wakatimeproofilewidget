use crate::{error::AppResult, state::AppState};

pub async fn push_to_widget(
    state: &AppState,
    code_time_total: String,
    code_time_7_days: String,
    total_lines: String,
    ai: String,
    human: String,
    fav_editor: String,
    fav_language: String,
) -> AppResult<()> {
    let app_id = &state.config.discord_app_id;
    let user_id = &state.config.discord_user_id;
    let bot_token = &state.config.discord_bot_token;

    let body = serde_json::json!({
      "username": "Venel",
      "data": {
        "dynamic": [
          {
            "type": 1,
            "name": "coding_time_total",
            "value": code_time_total
          },
          {
            "type": 1,
            "name": "coding_time_week",
            "value": code_time_7_days
          },
          {
            "type": 1,
            "name": "lines_total",
            "value": total_lines
          },
          {
            "type": 1,
            "name": "fav_language",
            "value": fav_language
          },
          {
            "type": 1,
            "name": "changes",
            "value": human
          },
          {
            "type": 1,
            "name": "ai_changes",
            "value": ai
          },
          {
            "type": 1,
            "name": "fav_editor",
            "value": fav_editor
          }
        ]
      }
    });

    state
        .http
        .patch(format!(
            "https://discord.com/api/v9/applications/{app_id}/users/{user_id}/identities/0/profile"
        ))
        .header("Authorization", format!("Bot {bot_token}"))
        .header(
            "User-Agent",
            "DiscordBot (https://github.com/discord/discord-api-docs, 1.0.0)",
        )
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
