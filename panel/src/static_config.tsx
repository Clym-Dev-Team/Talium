export function panel_base_url(): string {
  return getTagOrHardcoded("panel_base_addr", import.meta.env.VITE_PANEL_BASE_URL);
}
export function panel_path_prefix(): string {
  return new URL(panel_base_url()).pathname;
}
export function bot_backend_addr(): string {
  return getTagOrHardcoded("backend_base_addr", import.meta.env.VITE_BOT_BACKEND_ADDR);
}
export function history_backend_addr(): string {
  return import.meta.env.VITE_HISTORY_BACKEND_ADDR;
}
export function twitch_client_id(): string {
  return getTagOrHardcoded("twitch_client_id", import.meta.env.VITE_TWITCH_CLIENT_ID);
}

/**
 * Used to get the basic config parameters the client needs.
 * If they exist as link elements with the correct ID, the value from the href is used.
 * Otherwise, the hard_coded value is used.
 *
 * If the panel is served by the backend bot as a static site, the config values are inserted at runtime in the index.html
 * If the panel is served by the vite dev server, the values are hardcoded in via import.meta.env
 * @param id under which the backend sets the address
 * @param hard_coded value from vite via import.meta.env
 */
function getTagOrHardcoded(id: string, hard_coded: string) {
  let ele = document.getElementById(id);
  if (ele === null) {
    return hard_coded;
  } else {
    let link = ele as HTMLLinkElement;
    return link.href;
  }
}
