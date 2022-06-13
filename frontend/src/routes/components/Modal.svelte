<script lang="ts">
    export let isOpenModal: boolean = false;
</script>

<div id="modal" style="--display: {isOpenModal ? 'block' : 'none'};">
<textarea/>
</div>

<style>
    #modal {
        display: var(--display);
        position: fixed;
        z-index: 2;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: #fff;
        filter: drop-shadow(0 0 20px #333);
    }
</style>