<script lang="ts">
  import { afterUpdate } from 'svelte';
  import { invalidateAll } from '$app/navigation';
  import { API_URL } from './request';

  export let board: string;
  export let parentPost: string | undefined = undefined;

  let input: HTMLTextAreaElement;
  let postTitle = '';
  let postContent = '';
  let postFiles: FileList;
  let postFileUrl: string;

  afterUpdate(() => {
    input.style.height = '1px'; // we do this to avoid it getting incrementally bigger with every press
    input.style.height = `${Math.min(Math.max(26, input.scrollHeight), window.innerHeight / 3)}px`;
  });

  $: {
    if (postFiles?.length > 0) {
      let reader = new FileReader();
      reader.addEventListener('load', () => {
        postFileUrl = reader.result! as string;
      });
      reader.readAsDataURL(postFiles[0]);
    }
  }

  const resetImage = () => {
    postFiles = [] as unknown as FileList;
    postFileUrl = '';
  };

  const createPost = async () => {
    let form = new FormData();
    form.append(
      'post',
      JSON.stringify({ title: postTitle, content: postContent, parent: parentPost })
    );
    if (postFiles?.length) {
      form.append('image', postFiles[0], postFiles[0].name);
    }
    await fetch(`${API_URL}/${board}/new`, { method: 'POST', body: form });
    postTitle = '';
    postContent = '';
    resetImage();
    await invalidateAll();
  };
</script>

<form on:submit|preventDefault={createPost} id="post-form">
  <h3>Create a Post</h3>
  <input
    bind:value={postTitle}
    name="post-title"
    placeholder="Some interesting title"
    id="post-title-input"
  />
  <textarea
    bind:this={input}
    bind:value={postContent}
    name="post-content"
    placeholder="Some interesting news"
    id="post-input"
  />
  <span id="image-input-container">
    <label id="image-label" for="post-image">
      {#if postFileUrl}
        <span>Change Image</span>
        <button id="image-reset" on:click={resetImage}>Reset</button>
      {:else}
        Upload an Image
      {/if}
    </label>
    <input
      bind:files={postFiles}
      id="post-image"
      name="post-image"
      type="file"
      accept="image/*,video/*"
    />
  </span>
  {#if postFileUrl}
    <img src={postFileUrl} alt="" class="image-preview" />
  {/if}
  <button id="post-button" disabled={!postTitle.length}>Post</button>
</form>

<style>
  #post-form {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  #image-input-container {
    margin: 10px 0;
  }

  #image-label {
    cursor: pointer;
    color: var(--green-200);
  }

  #image-reset {
    outline: none;
    border: unset;
    background-color: inherit;
    color: var(--red-200);
    font-size: inherit;
  }

  #post-image {
    visibility: hidden;
  }

  #post-title-input {
    width: 100%;
    outline: none;
    border: unset;
    background-color: var(--black-100);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    color: var(--white-300);
    font-size: 18px;
    margin-bottom: 10px;
  }

  #post-input {
    resize: none;
    width: 100%;
    outline: none;
    border: unset;
    background-color: var(--black-100);
    border: 2px solid var(--white-100);
    border-radius: 10px;
    color: var(--white-300);
    font-size: 18px;
    min-height: 10vh;
  }

  #post-button {
    outline: none;
    border: unset;
    background-color: var(--blue-200);
    border-radius: 10px;
    color: var(--white-300);
    padding: 10px 20px;
    margin-left: auto;
    font-size: 18px;
    transition: opacity 250ms ease-in-out;
  }

  #post-button:disabled {
    opacity: 0.8;
  }
</style>
