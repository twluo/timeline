const BASE_URL = import.meta.env.VITE_TIMELINES_ENDPOINT;

class TimelineClient {
  async get<T>(path: string): Promise<T> {
    const response = await fetch(`${BASE_URL}${path}`);
    if (!response.ok) {
      throw new Error(`GET ${path} failed: ${response.status} ${response.statusText}`);
    }

    return response.json() as Promise<T>;
  }
}

export const timelineClient = new TimelineClient();
