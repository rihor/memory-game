<script lang="ts">
  import Timer from "$lib/Timer.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { PageData } from "./$types";

  export let data: PageData;

  let letters = data.letters;
  let board = data.board;
  let selection1Id: null | string = null;
  let selection2Id: null | string = null;
  let isBeginning = true;

  $: if (selection1Id && selection2Id) {
    setTimeout(() => {
      if (selection1Id && selection2Id) {
        checkGuess(selection1Id, selection2Id);
      }
    }, 250);
  }

  function selectCell(rowIndex: number, colIndex: number) {
    const id = board[rowIndex][colIndex]?.uuid;

    if (selection1Id) {
      selection2Id = id;
      return;
    }

    selection1Id = id;
  }

  async function checkGuess(id1: string, id2: string) {
    const guessResponse = await invoke("guess", { guess1: id1, guess2: id2 });
    selection1Id = null;
    selection2Id = null;
  }

  function onTimeEnd() {
    isBeginning = false;
  }
</script>

<div class="w-full h-full gap-10 flex flex-col items-center justify-center p-6">
  <header class="flex gap-4 items-center">
    <a
      href="/"
      class="text-2xl flex hover:opacity-70 transition-all font-medium text-blue-600"
      >&#8592</a
    >

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

  <Timer on:timeend={onTimeEnd} />

  <div class="flex flex-col gap-4 w-full h-full m-auto items-center">
    {#each board as row, rowIndex}
      <div class="flex items-center justify-center gap-4">
        {#each row as col, colIndex}
          {@const isSelected =
            col.uuid === selection1Id || col.uuid === selection2Id}
          <div
            class="flex place-content-center w-12 text-2xl py-4 px-8 bg-blue-400 font-bold text-white rounded-md hover:bg-blue-500 cursor-pointer transition-all duration-150"
            on:click={() => selectCell(rowIndex, colIndex)}
            on:keypress={() => selectCell(rowIndex, colIndex)}
            style:outline={isSelected
              ? "2px solid red"
              : "2px solid transparent"}
          >
            {#if isBeginning || col.marked}
              {col.value}
            {:else}
              ?
            {/if}
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>
