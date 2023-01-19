<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;

  let letters = data.letters;
  let board = data.board;
</script>

<div class="w-full h-full flex flex-col items-center justify-center p-6">
  <header class="flex gap-6 items-center">
    <h1 class="text-blue-600 font-bold text-2xl">Letters in game</h1>

    <div class="flex gap-2">
      {#each letters as letter}
        <span
          class="text-lg bg-slate-400 rounded-md py-2 p-4 text-blue-100 font-bold"
          >{letter}</span
        >
      {/each}
    </div>
  </header>

  <div class="flex gap-4 w-full h-full">
    {#each board as row}
      <div class="flex h-7 bg-red-100">
        {#each row as col}
          <div class="bg-blue-400 gap-2">{col.value}</div>
        {/each}
      </div>
    {/each}
  </div>
</div>
