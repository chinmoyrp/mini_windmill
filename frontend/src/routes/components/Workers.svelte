<script lang="ts">
    import { onMount } from "svelte";
    import { apiServer, workers } from "../../store";

    onMount(async () => {
        fetch($apiServer + "/workers")
        .then(response => response.json())
        .then(data => {
            workers.set(data["result"]);            
        })
        .catch(error => console.log(error));
    });

</script>

<div class="flex justify-around">
{#each $workers as {id, status, last_updated}}
<div class = "step-card my-8 p-4 w-64 h-30 rounded-tl-2xl rounded-2xl">
    <div class="flex flex-col justify-around h-full">
        <div>
            <p class="text-lg">Worker {id} <span class="text-xs">{status}</span> </p>
        </div>

        <div class="flex justify-end gap-2 text-sm">
            <p class="text-xs">Last updated: {last_updated}</p>
        </div>
    </div>
</div>
{/each}
</div>

<style>
    .step-card {
        background: theme('colors.brightblue');
    }

    .step-card:hover {
        box-shadow: inset 5px 5px theme('colors.razorblue'), inset -5px -5px theme('colors.razorblue');
        transition: box-shadow 0.2s linear;
    }

    .step-card:not(:hover) {
        transition: box-shadow 0.2s linear;
    }
</style>