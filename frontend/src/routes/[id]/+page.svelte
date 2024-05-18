<script lang="ts">
  import Markdown from '$lib/Markdown.svelte';
  import type { PageData } from './$types';
  import Post from '$lib/Post.svelte';
  import PostForm from '$lib/PostForm.svelte';

  export let data: PageData;
</script>

<a href="/" id="logo">
  <img src="/logo.svg" alt="Logo" id="logo-image" />
  Home
</a>

<div id="board-info">
  <h1 id="board-name">/{data.board.id}/</h1>
  {#if data.board.description}
    <p id="board-description"><Markdown content={data.board.description} /></p>
  {/if}
</div>

<img id="curves" src="/waves.svg" alt="waves" />

<div id="content">
  <div id="posts">
    <PostForm board={data.board.id} />
    <hr />
    {#each data.posts as post (post.id)}
      <div class="post">
        <a class="post-link" href="/posts/{post.id}">
          <Post {post} />
        </a>
        {#if post.replies.length}
          <div class="post-replies">
            {#each post.replies as reply (reply.id)}
              <a class="post-link" href="/posts/{reply.id}">
                <Post post={reply} />
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
  #logo {
    display: flex;
    margin: 10px 0 0 10px;
    align-items: center;
    gap: 10px;
    text-decoration: 0;
    font-weight: bold;
  }

  #logo-image {
    width: 40px;
  }

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

  hr {
    width: 90%;
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

  .post-replies {
    box-sizing: border-box;
    display: flex;
    flex-wrap: wrap;
    margin: 10px 20px;
    gap: 10px;
  }

  .post-link {
    text-decoration: none;
  }

  .post-replies .post-link {
    width: 100%;
  }

  @media only screen and (max-width: 1200px) {
    #content {
      flex-direction: column;
    }

    #posts {
      width: calc(100% - 40px);
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
