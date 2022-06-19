<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { apiServer } from "../../store";

    const dispatch = createEventDispatcher();

    export let index: number = -1;
    export let name: string = 'hello-world';
    export let hash: string = 'xxxyyy...';

    function stepDeleted() {
		dispatch('stepDeleted');
	}

    const createJob = async (hash: string) => {
        //console.log(JSON.stringify({id:"", worker_id:0, status:{Created:""}, kind:{Step:hash}}));
        const response = await fetch($apiServer + "/jobs", {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                body: JSON.stringify({id:"", worker_id:0, status:{Created:""}, kind:{Step:hash}})
                            });
        const data = await response.json();
        
        if (data.result) {
            alert("Created job: " + data.result);
        }
    }

    const removeStep = async (hash: string) => {
        fetch($apiServer + "/steps/remove/" + hash)
            .then(response => response.json())
            .then(data => {
                let id = data.result;
               // alert('Deleted step: ' + id);
            })
            .catch(error => console.log(error));

            location.reload();
    }

    const removeFromFlow = async () => {
        fetch($apiServer + "/flow/remove/" + index)
            .then(response => response.json())
            .then(data => {
                let id = data.result;
                //alert('Removed step: ' + hash);
            })
            .catch(error => console.log(error));
            location.reload();
    }

    function truncateHash(hash: string) {
        return hash.substr(0, 6);
    }

    function copyHash(hash: string) {
        navigator.clipboard.writeText(hash);
    }

</script>


<div class = "step-card p-4 w-64 h-40 rounded-tl-2xl rounded-br-2xl">
    <div class="flex flex-col justify-around h-full">
        <div>
            <p class="text-lg">{name}</p>
            <button class="text-xs" on:click={()=>{copyHash(hash)}}>{truncateHash(hash)}...(Copy)</button> 
        </div>

        <div class="flex justify-end gap-2 text-sm">
            {#if index === -1}
            <button on:click={async () => {removeStep(hash)}}>del</button>
            <button on:click={async () => {createJob(hash)}}>run</button>
            {:else}
            <button on:click={async () => {removeFromFlow()}}>del</button>
            {/if}
        </div>
    </div>
</div>

<style>
    div {
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