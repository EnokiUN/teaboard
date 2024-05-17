import { request } from '$lib/request';
import type { Board, Post } from '$lib/types';
import type { PageLoad } from './$types';

export const ssr = false;

export const load: PageLoad = async ({ params }) => {
  let board = params.id;
  return {
    boards: (await request('GET', `/`)).boards as Board[],
    board: (await request('GET', `/${board}`)) as Board,
    posts: (await request('GET', `/${board}/feed`)) as Post[]
  };
};

