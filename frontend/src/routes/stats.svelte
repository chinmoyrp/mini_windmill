<script lang="ts">
    import Job from "./components/Job.svelte";
    import Workers from "./components/Workers.svelte";   
</script>

<div class="ml-4 mr-4 mt-12 mb-12 p-1">
    <h2 class="uppercase text-lg pb-1 mb-2 border-b-2">Worker Stats</h2>
    <Workers></Workers>
    <h2 class="uppercase text-lg py-1 my-12 border-b-2">Job Stats</h2>
    <Job></Job>
</div>
