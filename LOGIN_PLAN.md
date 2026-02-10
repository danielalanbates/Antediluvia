# GUI Login & Auth Plan

## Goals
- Bevy-based login screen (username + password) with token auth.
- Secure session establishment for renet transport.
- Support guest mode fallback if server unreachable.

## Flow
1) Client boots into `AppState::Login`.
2) UI:
   - TextInputs: Username, Password.
   - Buttons: Login, Quit.
   - Status text for errors/connecting.
3) On Login:
   - POST to `/auth/login` (server HTTP or UDP challenge) -> returns { player_id, token, expires }.
   - Store token in memory.
   - Transition to `AppState::Connecting`.
4) Connecting:
   - Start renet client with token as authentication data.
   - On success: transition to `AppState::InWorld`.
   - On failure: show error, return to Login.
5) InWorld:
   - Normal gameplay; periodic token refresh (optional).

## Server-side Auth Stub
- Add `/auth/login` HTTP handler (axum/warp or simple UDP challenge).
- Validate credentials (for now: accept any non-empty username/password).
- Issue token (JWT or random UUID) with player_id.
- Persist player on first login (sqlx `players` table).

## Data Models (sqlx)
- `players(id BIGINT PK, name TEXT, lineage TEXT, corruption REAL, position_x REAL, position_y REAL, position_z REAL, inventory_json TEXT)`
- `world(id BIGINT PK, corruption REAL, flood_phase TEXT, server_time_days INT)`

## Bevy UI Implementation Outline
- Add `login` module to client.
- Resources: `LoginState { username, password, status }`.
- Systems:
  - `login_ui_system` (render fields/buttons).
  - `login_submit_system` (handle button/Enter; send HTTP/UDP request).
  - `login_response_system` (handle async response; set token; change state).
- Assets: simple font, panel background; keep diegetic feel (papyrus panel).

## Security Notes
- Use HTTPS for auth if exposed externally.
- Rate-limit login attempts server-side.
- Tokens should be short-lived; refresh on world load or every 30 minutes.

## Next Steps
- Implement `/auth/login` endpoint in server crate (add minimal HTTP server).
- Add `login` Bevy state and UI to client.
- Integrate token into renet handshake once transport is wired.
