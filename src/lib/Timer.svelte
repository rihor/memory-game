<script lang="ts">
  import { tweened } from "svelte/motion";
  import { createEventDispatcher } from "svelte";

  export let originalTime = 10;

  const dispatch = createEventDispatcher();
  let timer = tweened(originalTime);

  $: seconds = Math.floor($timer);
  $: timerPercent = $timer / originalTime;

  setInterval(() => {
    if ($timer > 0) $timer--;
  }, 1000);

  $: if (seconds == 0) {
    dispatch("timeend");
  }
</script>

<div class="flex flex-col gap-1 w-2/6">
  <div class="flex mb-2 items-center justify-between gap-4">
    <div>
      <span
        class="font-semibold inline-block py-1 px-2 uppercase rounded-full text-emerald-600 bg-emerald-200"
      >
        Time remaining
      </span>
    </div>
    <div class="text-right">
      <span class="text-2xl font-bold inline-block text-emerald-100">
        {seconds}
      </span>
    </div>
  </div>
  <div
    class="overflow-hidden relative h-3 text-xs flex rounded-md bg-emerald-200"
  >
    <div
      style="width: 10%"
      style:width={`${Math.floor(timerPercent * 35)}%`}
      class="h-3 z-30 shadow-none absolute top-0 left-0 rounded-r-md flex flex-col text-center whitespace-nowrap text-white justify-center bg-emerald-500"
    />
    <div
      style:width={`${Math.floor(timerPercent * 65)}%`}
      class="h-3 z-20 shadow-none absolute top-0 left-0 rounded-r-md flex flex-col text-center whitespace-nowrap text-white justify-center bg-emerald-400"
    />
    <div
      style:width={`${Math.floor(timerPercent * 100)}%`}
      class="h-3 z-10 shadow-none absolute top-0 left-0 rounded-r-md flex flex-col text-center whitespace-nowrap text-white justify-center bg-emerald-300"
    />
  </div>
</div>
