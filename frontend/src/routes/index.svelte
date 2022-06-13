<script lang="ts">
    import Step from "./components/Step.svelte";
    import Modal from "./components/Modal.svelte";

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
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2 border-brightblue">Available scripts</h2>
    {#each scripts as content}
        <Step {content}></Step>
    {/each}
    <Modal {isOpenModal}> </Modal>
    <button on:click={openEditor}>Clickme!</button>
</div>

<style>
    h2 {
        color: rgb(255, 255, 255, 0.7);
    }
</style>