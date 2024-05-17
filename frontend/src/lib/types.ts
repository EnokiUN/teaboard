export interface Board {
  id: string;
  description?: string;
};

export interface Post {
  id: string;
  board: string;
  title: string;
  content?: string;
  pinned?: boolean;
  moderator?: boolean;
  locked?: boolean;
  parent?: string;
  image?: string;
  replies: Post[];
  mentions?: number[];
  mentioned_posts?: number[];
};
