export const API_URL = 'http://0.0.0.0:8000';

export interface RequestOptions {
  apiUrl?: string;
  token?: string;
  empty?: boolean;
}

export interface RequestErr {
  message: string;
  code: number;
  err?: Object;
}

export const request = async (
  method: string,
  route: string,
  body?: Object | null,
  options?: RequestOptions
): Promise<any> => {
  let url = API_URL;
  if (!url?.endsWith('/')) {
    url = url + '/';
  }
  let headers = new Headers();
  if (options?.token) {
    headers.append('Authorization', options?.token || ''); // :/
  }
  const resp = await fetch(`${url}${route}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : null
  });
  if (resp.status >= 200 && resp.status < 300) {
    if (!options?.empty && resp.status != 204) {
      return await resp.json();
    }
    return;
  }
};

