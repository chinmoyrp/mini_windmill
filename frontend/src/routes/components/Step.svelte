<script lang="ts">
    export let content: string = 'default';
</script>


<div class = "my-8 p-8 w-64 h-40 rounded-2xl">
    <p>{content}</p>
</div>

<style>
    div {
        background: theme('colors.brightblue');
    }

    div:hover {
        background: rgb(1, 1, 255, 0.9);
        transition: background 0.2s linear;
    }
</style>