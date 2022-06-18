<script lang="ts">
    import Step from "./components/Step.svelte";
    import { onMount } from "svelte";
    import { stepHash } from '../store';

    let flow = [];
    const getFlow = async () => {
        Promise.all([
            fetch('http://localhost:8080/api/flow'),
            fetch('http://localhost:8080/api/steps')
        ]).then(function (responses) {
            // Get a JSON object from each of the responses
            return Promise.all(responses.map(function (response) {
                return response.json();
            }));
        }).then(function (data) {
            let _data = [];
            let flowRes = data[0]["result"][0]["steps"];
           // console.log(data[0]["result"][0]["steps"]);
            let stepRes = data[1]["result"];
            let i=0;
            for (const s in stepRes) {
                for (const h in flowRes) {
                    let step = stepRes[s];
                    let stepHash = step.hash
                    let flowHash = flowRes[h];
                    if (stepHash.localeCompare(flowHash) === 0) {
                        let obj = {index: i, hash: stepHash, name: step.name};
                        flow = [...flow, obj];
                        i = i+1;     
                    }
                }
            }
        }).catch(function (error) {
            // if there's an error, log it
            console.log(error);
        });
    }

    onMount(async () => {
        getFlow();
    });

    const addToFlow = async () => {
        fetch("http://localhost:8080/api/flow/add/"+$stepHash)
            .then(response => response.json())
            .then(data => {
                let id = data.result;
                //alert('Added to flow: ' + id +". Please refresh.");
            })
            .catch(error => console.log(error));

            addFlow = false;
            location.reload();
    }

    const createJob = async () => {
        const response = await fetch("http://localhost:8080/api/jobs", {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                body: JSON.stringify({id:"", worker_id:0, status:{Created:""}, kind:{Flow:""}})
                            });
        const data = await response.json();
        
        if (data.result) {
            alert("Created job: " + data.result);
        }
    }

    let addFlow = false;
    function showAddFlow() {
        addFlow = true;
    }
 
</script>

<div class="ml-4 mr-4 mt-12 mb-12 p-1">
    {#if !addFlow}
    <div class="flex justify-between pb-1 mb-2 border-b-2 border-brightblue">
        <h2 class="uppercase text-lg">Add steps to this flow</h2>
        <button class="rounded-lg text-sm p-1" on:click={async () => {createJob()}}>Run this flow</button>
    </div>
    <div class="flex flex-wrap gap-8">
        {#each flow as {index, hash, name}}
            <Step {index} {hash} {name}></Step>
        {/each}
    </div>
    <button class="w-12 mt-8 rounded-lg text-xs" on:click={showAddFlow}>Add</button>

    {:else}
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2">Paste a valid hash</h2>
    <div class="flex flex-col mt-2 gap-4">
        <input bind:value={$stepHash} placeholder="Hash...">
        <div class="w-16">
        <button class="w-12 mt-8 rounded-lg text-xs" disabled={$stepHash.length < 31} on:click={addToFlow}>Submit</button>
        </div>
    </div>

    {/if}
</div>



<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }

    input {
        background: theme('colors.darkestblue');
    }

    button:disabled {
        background: rgb(87, 159, 255, 0.4);
    }

</style>