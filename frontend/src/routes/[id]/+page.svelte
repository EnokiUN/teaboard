<script lang="ts">
  import { API_URL } from '$lib/request';
  import Markdown from '$lib/Markdown.svelte';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<div id="board-info">
  <h1 id="board-name">/{data.board.id}/</h1>
  {#if data.board.description}
    <p id="board-description"><Markdown content={data.board.description} /></p>
  {/if}
</div>

<img id="curves" src="/waves.svg" alt="waves" />

<div id="content">
  <div id="posts">
    {#each data.posts as post (post.id)}
      <div class="post">
        <a class="post-link" href="/posts/{post.id}">
          <div class="post-card">
            <span class="post-id">
              {post.id}
            </span>
            <h3>
              <Markdown content={post.title} />
            </h3>
            {#if post.content}
              <Markdown content={post.content} />
            {/if}
            {#if post.image}
              <img class="post-image" src="{API_URL}/images/{post.image}" alt="" />
            {/if}
          </div>
        </a>
        {#if post.replies.length}
          <div class="post-replies">
            {#each post.replies as reply (reply.id)}
              <a class="post-link" href="/posts/{reply.id}">
                <div class="post-card">
                  <span class="post-id">
                    {reply.id}
                  </span>
                  <h3>
                    <Markdown content={reply.title} />
                  </h3>
                  {#if reply.content}
                    <Markdown content={reply.content} />
                  {/if}
                  {#if reply.image}
                    <img class="post-image" src="{API_URL}/images/{reply.image}" alt="" />
                  {/if}
                </div>
              </a>
            {/each}
            {#if post.replies.length >= 5}
              <a href="/posts/{post.id}">See more...</a>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <div id="boards">
    <h2>Other Boards</h2>
    {#each data.boards as board}
      <a class="board-card" href="/{board.id}">
        <h3>
          /{board.id}/
        </h3>
        {#if board.description}
          <p>
            <Markdown content={board.description} />
          </p>
        {/if}
      </a>
    {/each}
  </div>
</div>

<style>
  #board-info {
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    align-items: center;
    margin: 30px;
    gap: 10px;
  }

  #board-name {
    margin: 0;
  }

  #board-description {
    margin: 0;
  }

  #curves {
    margin: 50px 0;
    width: 100%;
  }

  #content {
    display: flex;
    width: 100%;
    gap: 10px;
  }

  #boards {
    flex-grow: 1;
    display: flex;
    gap: 10px;
    flex-direction: column;
  }

  .board-card {
    display: block;
    box-sizing: border-box;
    background-color: var(--black-200);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    padding: 10px;
    text-decoration: none;
    width: calc(100% - 20px);
    margin-right: 20px;
  }
  .board-card h3 {
    margin: 5px;
  }

  #posts {
    box-sizing: border-box;
    display: flex;
    width: 70vw;
    gap: 10px;
    padding: 0 5px;
    margin: 0 20px;
    flex-direction: column;
  }

  .post-card {
    box-sizing: border-box;
    background-color: var(--black-200);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    width: 100%;
    text-decoration: none;
    padding: 10px;
  }

  .post-replies {
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    margin: 10px 20px;
    gap: 10px;
  }

  .post-card h3 {
    margin-top: 0;
  }

  .post-id {
    font-size: 16px;
  }

  .post-link {
    text-decoration: none;
  }

  .post-replies .post-link {
    width: 100%;
  }

  .post-image {
    margin-top: 20px;
  }

  @media only screen and (max-width: 1200px) {
    #content {
      flex-direction: column;
    }

    #posts {
      width: fit-content;
    }

    #boards h2 {
      margin-left: 20px;
    }

    .board-card {
      width: calc(100% - 40px);
      margin: 0 20px;
    }
  }
</style>
