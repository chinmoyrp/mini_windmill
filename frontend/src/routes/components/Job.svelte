<script lang="ts">
    import { onMount } from "svelte";
    import { jobs } from "../../store";

    onMount(async () => {
        fetch("http://localhost:8080/api/jobs")
        .then(response => response.json())
        .then(data => {
           // console.log(data["result"])
            jobs.set(data["result"]);            
        })
        .catch(error => console.log(error));
    });

    function getStatus(obj: Object) {
        //console.log(Object.entries(obj))
        return 1
    }

    let viewJobResult: boolean = false;

    let jid = '';
    let out = '';

    const fetchJobDetails = async (id: string) => {
        const response = await fetch("http://localhost:8080/api/jobs/" + id);
        const data = await response.json();
        const arrOut = data['result'];
        const _output = arrOut['output'];
        out = _output.join('\n');
        jid = id;
        // console.log(out);
        // const jobKind = Object.keys(kind);
        // const isStep = jobKind[0].localeCompare('Step') === 0;
        // if (isStep) {
        //     const hash = Object.values(kind);
        //     const resp = await fetch("http://localhost:8080/api/steps/" + hash);
        //     const dt = await resp.json()
        //     const arr = Object.values(dt['result']);
        //     step = arr[2];
        // } else {
        //     step = '[Flow]';
        // }  
        viewJobResult = true;      
    }

</script>

<div class="flex flex-col">
{#if !viewJobResult}
{#each $jobs as {id, kind, status, worker_id}}
<div class = "job-card my-2 p-2 w-full h-30 rounded-2xl">
    <div class="flex flex-col justify-around h-full">
        <div>
            <span>
            <button class="text-lg" on:click={async () => {fetchJobDetails(id)}}>{id}</button>
            <p class="text-xs">Worker {worker_id}</p>
            </span>
        </div>

        <div class="flex justify-end gap-2 text-sm">
            <p class="text-xs">{Object.keys(status)}: {Object.values(status)}</p>
        </div>
    </div>
</div>
{/each}
{:else}
<div class="mb-4">
    <p class="text-md">{jid}</p>
</div>
<textarea rows="10" cols="50" readonly>{out}</textarea>
<div class="mt-4">
    <button class="btn-back" on:click={() => {viewJobResult = false}}>&#60;-Back</button>
</div>
{/if}
</div> 

<style>
    .job-card {
        background: theme('colors.razorblue');
        border-width: 1px;
        border-color: rgb(255, 255, 255, 0.5);

    }

    .job-card:hover {
        box-shadow: inset 5px 5px theme('colors.razorblue'), inset -5px -5px theme('colors.razorblue');
        transition: box-shadow 0.2s linear;
    }

    .job-card:not(:hover) {
        transition: box-shadow 0.2s linear;
    }

    button {
        background: theme('colors.razorblue');
    }

    textarea {
        background: theme('colors.darkestblue');
        border-width: 2px;
    }

</style>