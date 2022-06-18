<script lang="ts">
    import ModalEdit from "./ModalEdit.svelte";

    let viewCode = false;

    export let name: string = 'hello-world';
    export let hash: string = 'xxxyyy...';
    export let code: string = `\
package main

import (
    "fmt"
    "time"
)

func main() {
    fmt.Println("hello world!")
}
`

    const createJob = async (hash: string) => {
        console.log(JSON.stringify({id:2}));
        const response = await fetch("http://localhost:8080/api/jobs", {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json'
                                },
                                mode: 'no-cors',
                                body: JSON.stringify({id:2})
                            });
        const data = await response.json();
        
        //console.log(data);
    }

    function truncateHash(hash: string) {
        return hash.substr(0, 10);
    }

    function openModal() {
        viewCode = !viewCode;
    }

    function closeModal() {
        viewCode = false;
    }

</script>


<div class = "step-card my-8 p-4 w-64 h-40 rounded-tl-2xl rounded-br-2xl">
    <div class="flex flex-col justify-around h-full">
        <div>
            <p class="text-lg">{name}</p>
            <p class="text-xs">{truncateHash(hash)}</p> 
        </div>

        <div class="flex justify-end gap-2 text-sm">
            <button on:click={openModal}>view</button>
            <ModalEdit {code} {name} {viewCode}></ModalEdit>
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