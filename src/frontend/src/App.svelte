<script lang="ts">
  import { canisterId, createActor } from "../../declarations/surveys";
  import { onMount } from "svelte";

  let title: string = "The question is";
  let questions: string[] = ["Will this work?", "At some point?"];
  let surveysActor;

  onMount(async () => {
    surveysActor = createActor(canisterId, { agentOptions: { host: 'http://localhost:8000' } });
    const surveys = await surveysActor.read_all();
    console.log('type of count:', typeof surveys);
    console.log('The count is: ', surveys);
  })

  const createSurvey = async () => {
    try {
      console.log('creating', title, questions);
      const data = await surveysActor.create(title, questions);
      console.log(data);
      
    } catch (error) {
      console.log('In da error');
      console.log(error);
    }
  }
</script>

<main>
  <div>
    <h2>New Survey</h2>
    <form>
      <input type="text" bind:value={title} placeholder="Title of the survey" />
      <input type="text" bind:value={questions[0]} placeholder="Question 1" />
      <input type="text" bind:value={questions[1]} placeholder="Question 2" />
      <input type="text" bind:value={questions[2]} placeholder="Question 3" />
      <button type="submit" on:click|preventDefault={createSurvey}>Create</button>
    </form>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
  }

  form {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;

  }
  input {
    width: 50%;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
  }
</style>
