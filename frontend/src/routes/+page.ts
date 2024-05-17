import { request } from '$lib/request';
import type { Board } from '$lib/types';
import type { PageLoad } from './$types';

export const ssr = false;

export const load: PageLoad = async ({ params }) => {
  return {
    boards: (await request('GET', `/`)).boards as Board[]
  };
};

