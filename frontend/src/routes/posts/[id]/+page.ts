import { request } from '$lib/request';
import type { Board, Post } from '$lib/types';
import type { PageLoad } from './$types';

export const ssr = false;

export const load: PageLoad = async ({ params }) => {
  let postId = params.id;
  return {
    boards: (await request('GET', `/`)).boards as Board[],
    post: await request('GET', `/posts/${postId}`) as Post
  };
};

