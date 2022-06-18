<script lang="ts">
    import Step from "./components/Step.svelte";
    import Modal from "./components/Modal.svelte";
    import { onMount } from "svelte";
    import { steps } from '../store';

    import Prism from "prettier";

    onMount(async () => {
        fetch("http://localhost:8080/api/steps")
        .then(response => response.json())
        .then(data => {
            steps.set(data["result"]);
            //console.log(names);
            
        })
        .catch(error => console.log(error));
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
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2">Available scripts</h2>
    <div class="flex gap-8">
        {#each $steps as {hash, name, code}}
        <Step {name} {hash} {code}></Step>
        {/each}
    </div>
    
    <Modal {isOpenModal}> </Modal>
</div>

<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }
</style>