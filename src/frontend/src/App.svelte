<script lang="ts">
  import { canisterId, createActor } from "../../declarations/surveys";
  import { onMount } from "svelte";

  type Question = {
    title: string;
  }

  type Survey = {
    id: string;
    title: string;
    questions: Question[];
  };

  let title: string;
  let questions: string[] = [];
  let surveysActor;
  let surveys: Array<Survey> = [];
  let currentSurvey: Survey | undefined;

  onMount(async () => {
    surveysActor = createActor(canisterId, { agentOptions: { host: 'http://localhost:8000' } });
    surveys = await surveysActor.read_all();
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

  const openSurvey = (survey) => {
    currentSurvey = survey;
  };

  const closeSurvey = () => {
    currentSurvey = undefined;
  };
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
  {#if currentSurvey}
  <div>
    <h3>{currentSurvey.title}</h3>
    <div class="surveys">
      {#each currentSurvey.questions as question}
        <span>{question.title}</span>
        <input type="text" />
      {/each}
    </div>
    <span role="button" on:click={closeSurvey}>Back to questions</span>
  </div>
  {:else}
    <div>
      <h3>Select a survey</h3>
      <div class="surveys">
        {#each surveys as survey}
          <span role="button" on:click={() => openSurvey(survey)}>{survey.title}</span>
        {/each}
      </div>
    </div>
  {/if}
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

  .surveys {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
  }
</style>
