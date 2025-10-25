export function panel_base_url(): string {
  return import.meta.env.VITE_PANEL_BASE_URL;
}
export function bot_backend_addr(): string {
  return import.meta.env.VITE_BOT_BACKEND_ADDR;
}
export function history_backend_addr(): string {
  return import.meta.env.VITE_HISTORY_BACKEND_ADDR;
}
export function twitch_client_id(): string {
  return import.meta.env.VITE_TWITCH_CLIENT_ID;
}
