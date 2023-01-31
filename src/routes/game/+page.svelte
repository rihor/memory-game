<script lang="ts">
  import Confetti from "$lib/Confetti.svelte";
  import Timer from "$lib/Timer.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { PageData } from "./$types";
  import type { Board } from "./+page";

  export let data: PageData;

  let selection1Id: null | string = null;
  let selection2Id: null | string = null;
  let isBeginning = true;
  let showSelected = false;

  $: board = data.board;
  $: hasWon = board.flat().every((cell) => cell.marked);

  $: if (selection1Id && selection2Id) {
    showSelected = true;

    if (selection1Id && selection2Id) {
      checkGuess(selection1Id, selection2Id).then((newBoard) => {
        setTimeout(() => {
          showSelected = false;
          selection1Id = null;
          selection2Id = null;
          board = newBoard;
        }, 1250);
      });
    }
  }

  function selectCell(rowIndex: number, colIndex: number) {
    // Block selection spam
    if (showSelected || isBeginning) {
      return;
    }

    const cell = board[rowIndex][colIndex];

    // Must not select a cell that was already solved
    if (cell.marked) {
      return;
    }

    if (selection1Id) {
      selection2Id = cell.uuid;
      return;
    }

    selection1Id = cell.uuid;
  }

  async function checkGuess(id1: string, id2: string) {
    return await invoke<Board>("guess", { id1, id2 });
  }

  function onTimeEnd() {
    isBeginning = false;
  }
</script>

<div class="w-full h-full gap-2 flex flex-col items-center justify-center p-6">
  <Confetti shouldPlay={hasWon} />

  <header class="w-full flex flex-col gap-12 items-center">
    <div class="w-full flex items-center gap-4">
      <a
        href="/"
        class="text-xl flex hover:opacity-70 transition-all font-medium text-emerald-100 gap-4"
        >&#8592 <span>Menu</span></a
      >
    </div>

    {#if isBeginning}
      <Timer on:timeend={onTimeEnd} />
    {/if}

    {#if hasWon}
      <h1 class="text-5xl text-emerald-50 font-bold">YOU WON!</h1>
    {/if}
  </header>

  <div
    class="flex flex-col gap-4 w-full h-full m-auto items-center justify-center"
  >
    {#each board as row, rowIndex}
      <div class="flex items-center justify-center gap-4">
        {#each row as col, colIndex}
          {@const isSelected =
            col.uuid === selection1Id || col.uuid === selection2Id}
          <div
            class={`flex place-content-center w-12 text-2xl py-4 px-8 ${
              isSelected ? "scale-110" : "scale-100"
            } ${
              col.marked
                ? "bg-purple-400 hover:bg-purple-500"
                : "bg-purple-200 hover:bg-purple-300 text-purple-800"
            } outline-4 font-bold text-white rounded-md  cursor-pointer transition-all duration-150`}
            on:click={() => selectCell(rowIndex, colIndex)}
            on:keypress={() => selectCell(rowIndex, colIndex)}
          >
            {#if isBeginning || col.marked || (showSelected && isSelected)}
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
