<script>
  import { onMount } from "svelte";

  export let name = "world";
  let count = 0;
  let items = ["one", "two", "three"];

  $: doubled = count * 2;

  function increment() {
    count += 1;
  }

  onMount(() => {
    console.log("mounted");
  });
</script>

<!-- A sample. -->
<main class="page">
  <h1>Hello {name}!</h1>
  <button on:click={increment} disabled={count > 10}>
    Clicked {count} {count === 1 ? "time" : "times"}
  </button>

  {#if doubled > 4}
    <p>Doubled is {doubled}.</p>
  {:else}
    <p>Keep clicking.</p>
  {/if}

  <ul>
    {#each items as item, i}
      <li>{i}: {item}</li>
    {/each}
  </ul>
</main>

<style>
  .page {
    margin: 0 auto;
  }
</style>
