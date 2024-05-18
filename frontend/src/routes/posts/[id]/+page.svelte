<script lang="ts">
  import Markdown from '$lib/Markdown.svelte';
  import Post from '$lib/Post.svelte';
  import PostForm from '$lib/PostForm.svelte';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<a href="/" id="logo">
  <img src="/logo.svg" alt="Logo" id="logo-image" />
  Home
</a>

<div id="main-post">
  <Post post={data.post} />
</div>
<img id="curves" src="/waves.svg" alt="waves" />
<div id="content">
  <div id="posts">
    <PostForm board={data.post.board} parentPost={data.post.id} />
    {#each data.post.replies as post (post.id)}
      <div class="post">
        <a class="post-link" href="/posts/{post.id}">
          <Post {post} />
        </a>
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

  #main-post {
    width: 80vw;
    margin: 20px auto;
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
    width: 60vw;
    gap: 10px;
    padding: 0 5px;
    margin: 0 20px;
    flex-direction: column;
  }

  .post-link {
    text-decoration: none;
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
