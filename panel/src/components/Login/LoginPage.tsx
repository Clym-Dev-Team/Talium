import {PropsWithChildren} from "react";
import LoginPane from "./LoginPane.tsx"
import {BOT_BACKEND_ADDR} from "../../main.tsx";

export default function LoginPage(props: PropsWithChildren<Record<never, never>>) {
  if (localStorage.getItem("accessToken") == null) {
    return <LoginPane/>;
  }
  return props.children;
}

export async function fetchWithAuth(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
  const accessToken = localStorage.getItem("accessToken");

  if (accessToken == null) {
    return Promise.reject(new Error("Token invalid or Timed out. Reload the page and login again!"));
  }

  if (init === undefined) {
    init = {headers: [["token", accessToken]]};
  } else if (init.headers === undefined) {
    init.headers = [["token", accessToken]]
  } else {
    const headers: Headers = new Headers(init.headers!);
    headers.set("token", accessToken);
    init.headers = headers;
  }

  const res = await fetch(BOT_BACKEND_ADDR + input, init);
  if (res.ok) {
    return res;
  } else if (res.status == 401) {
    localStorage.removeItem("accessToken");
    return Promise.reject(new Error("Token invalid or Timed out. New Login required!"));
  } else if (res.status == 403) {
    return Promise.reject(new Error("No Permission to execute this action"));
  } else {
    return Promise.reject(new Error(`${res.status} ${res.statusText}`));
  }
}