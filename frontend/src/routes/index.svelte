<script lang="ts">
    import Step from "./components/Step.svelte";
    import { onMount } from "svelte";
    import { stepName, stepCode } from '../store';

    let steps = []
    const fetchSteps = async () => {
        fetch("http://localhost:8080/api/steps")
        .then(response => response.json())
        .then(data => {
            steps = data["result"]            
        })
        .catch(error => console.log(error));
    }

    onMount(async () => {
        fetchSteps();
    });

    let addCode : boolean = false;
    const createStep = async () => {
        fetch("http://localhost:8080/api/steps", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({hash:"", name: $stepName, code: $stepCode})
        })
        .then(response => response.json())
        .then(data => {
            if (data.result) {
                alert("Added step: "+data.result+". Please refresh!");
            }
        })
        .catch(error => console.log(error));

        addCode = false;

        location.reload();
    }

    function showAddCode() {
        addCode = true;
    }

</script>

<div class="ml-4 mr-4 mt-12 mb-12 p-1">
    {#if !addCode}
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2">Available scripts</h2>
    <div class="flex flex-wrap gap-8">
        {#each steps as {hash, name, code}}
        <Step {name} {hash} on:stepDeleted={fetchSteps}></Step>
        {/each}
    </div>
    <button class="w-12 mt-8 rounded-lg text-xs" on:click={showAddCode}>Add</button>

    {:else}
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2">Add code</h2>
    <div class="flex flex-col mt-2 gap-4">
        <input bind:value={$stepName} placeholder="Name...">
        <textarea rows="10" cols="50" placeholder="Code..." bind:value={$stepCode}></textarea>
        <button class="text-sm" disabled={$stepName.length === 0 || $stepCode.length === 0} on:click={createStep}>Submit</button>
    </div>

    {/if}
</div>

<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }

    input, textarea {
        background: theme('colors.darkestblue');
    }

    button:disabled {
        background: rgb(87, 159, 255, 0.4);
    }

</style>