<script lang="ts">
    import Step from "./components/Step.svelte";
    import ModalFlow from "./components/ModalFlow.svelte";
    import { onMount } from "svelte";
    import { flow } from '../store';

    onMount(async () => {
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
            let stepRes = data[1]["result"];
            for (const s in stepRes) {
                for (const h in flowRes) {
                    let step = stepRes[s];
                    let stepHash = step.hash
                    let flowHash = flowRes[h];
                    if (stepHash.localeCompare(flowHash) === 0) {
                        _data.push(step);
                    }

                }
            }
            flow.set(_data);
        }).catch(function (error) {
            // if there's an error, log it
            console.log(error);
        });
    });
 
    let scripts: string[] = ['hello-world'];
    let isOpenModal: boolean = false;

    function addScript() {
        scripts = [...scripts, 'by-world'];
    }

    function openEditor() {
        isOpenModal = !isOpenModal;
    }

</script>

<div class="ml-4 mr-4 mt-12 mb-12 p-1">
    <div class="flex justify-between pb-1 mb-2 border-b-2 border-brightblue">
    <h2 class="uppercase text-lg">Add steps to this flow</h2>
    <button class="rounded-lg text-sm p-1">Run this flow</button>
    </div>
    {#each $flow as {hash, name, code}}
        <Step {hash} {name} {code}></Step>
    {/each}
    <ModalFlow {isOpenModal}> </ModalFlow>
</div>

<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }
</style>