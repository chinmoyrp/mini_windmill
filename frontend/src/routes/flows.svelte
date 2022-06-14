<script lang="ts">
    import Step from "./components/Step.svelte";
    import ModalFlow from "./components/ModalFlow.svelte";

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
    {#each scripts as content}
        <Step {content}></Step>
    {/each}
    <ModalFlow {isOpenModal}> </ModalFlow>
    <button on:click={openEditor}>Clickme!</button>
</div>

<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }
</style>