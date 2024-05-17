<script lang="ts">
  import Markdown from '$lib/Markdown.svelte';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<main>
  <div id="header">
    <img id="logo" src="/logo.svg" alt="logo" />
    <div id="header-text">
      <h1>Welcome to Teaboard</h1>
      <p>A place to get and share news anonymously, with a side of tea.</p>
    </div>
  </div>

  <img id="curves" src="/waves.svg" alt="waves" />

  <div id="boards">
    {#each data.boards as board (board.id)}
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
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
  }

  #header {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 20px;
    margin-top: 10vh;
  }

  #header-text {
    display: flex;
    justify-content: center;
    flex-direction: column;
    gap: 10px;
  }

  #header-text * {
    margin: 0 0;
  }

  #logo {
    width: 128px;
  }

  #curves {
    margin: 50px 0;
  }

  #boards {
    box-sizing: border-box;
    display: flex;
    margin: 0 auto;
    flex-wrap: wrap;
    width: 90vw;
    gap: 10px;
    padding: 0 5px;
  }

  .board-card {
    box-sizing: border-box;
    background-color: var(--black-200);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    width: calc(30vw - 10px);
    padding: 30px;
    text-decoration: none;
  }
</style>
