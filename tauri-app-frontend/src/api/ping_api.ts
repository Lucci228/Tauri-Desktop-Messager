const API_BASE = import.meta.env.VITE_API_URL ?? "http://localhost:7878";

export interface PingResponse {
  status: string;
  ping_count: number;
}

async function apiFetch<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, options);
  if (!res.ok) throw new Error(`API error: ${res.status}`);
  return res.json();
}

export async function ping_server(): Promise<PingResponse> {
  const res = await apiFetch<PingResponse>("/ping/test");
  return res
}
