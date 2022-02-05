<script lang="ts">
  import CanisterIds from "./components/CanisterIds.svelte";
  import Links from "./components/Links.svelte";
  import { counter } from "../../declarations/counter";
  // import { counter } from "./counter";
  import { onMount } from "svelte";

  onMount(async () => {
    const count = await counter.read();
    console.log('type of count:', typeof count);
    console.log('The count is: ', count);
  })
</script>

<main>
  <a
    href="https://dfinity.org"
    target="_blank"
    rel="noopener noreferrer"
    class="logo"
  >
    <img src="images/dfinity.svg" alt="DFINITY logo" />
  </a>
  <h1>Svelte Starter dApp</h1>
  <Links />
  <CanisterIds />
</main>

<style>
  img {
    height: 22px;
  }
  .logo {
    display: inline-block;
    margin-bottom: 64px;
    margin-top: 24px;
  }

  main {
    text-align: center;
    padding: 1em;
  }

  h1 {
    text-transform: uppercase;
    font-size: 3em;
    font-weight: 400;
    line-height: 1.09;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
    h1 {
      font-size: 4em;
    }
  }
</style>
