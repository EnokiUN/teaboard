<script lang="ts">
  import Markdown from './Markdown.svelte';
  import { API_URL } from './request';
  import type { Post } from './types';

  export let post: Post;
</script>

<div class="post-card">
  <div class="post-text">
    <span class="post-id">
      {post.id}
    </span>
    <h3>
      <Markdown content={post.title} />
    </h3>
    {#if post.content}
      <Markdown content={post.content} />
    {/if}
  </div>
  {#if post.image}
    <img class="post-image" src="{API_URL}/images/{post.image}" alt="" />
  {/if}
</div>

<style>
  .post-card {
    display: flex;
    box-sizing: border-box;
    background-color: var(--black-200);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    width: 100%;
    text-decoration: none;
    padding: 10px;
    gap: 10px;
  }

  .post-text {
    flex-grow: 1;
  }

  .post-card h3 {
    margin-top: 0;
  }

  .post-id {
    font-size: 16px;
  }

  .post-image {
    margin-top: 20px;
    max-width: 20%;
    object-fit: contain;
  }
</style>
