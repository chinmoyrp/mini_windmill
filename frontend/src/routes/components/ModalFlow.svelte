<script lang="ts">
    export let isOpenModal: boolean = false;
    let boilerplate = `
func _g_main(x int, y int) int {
    return x+y
}`

    let saveDisabled: boolean = true;
    let btnText: string = 'Next';

    function reviewSubmission() {
        saveDisabled = !saveDisabled;
        if (saveDisabled) {
            btnText = 'Next';
        } else {
            btnText = 'Back';
        }
    }

    function closeModal() {
        isOpenModal = false;
    }

</script>

<div id="modal" style="--display: {isOpenModal ? 'block' : 'none'};" class="rounded-2xl p-1">
    <div class="codeinput divide-y-2 rounded-xl p-2">
        <div class="flex justify-between">
            <label for="cars">Choose a car:</label>

            <select name="cars" id="cars">
              <option value="volvo">Volvo</option>
              <option value="saab">Saab</option>
              <option value="mercedes">Mercedes</option>
              <option value="audi">Audi</option>
            </select>

            <button class="codenext px-2 mb-1 rounded-xl text-xs" on:click={reviewSubmission}>{btnText}</button>
        </div>
        <div class="w-full">
            <button class="codenext px-2 mt-2 rounded-xl text-xs" disabled={saveDisabled} on:click={closeModal}>Save</button>
        </div>
    </div>
</div>

<style>
    #modal {
        display: var(--display);
        position: fixed;
        z-index: 2;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: theme('colors.darkerblue');
        filter: drop-shadow(0 0 20px theme('colors.brightblue'));
    }

    .codeinput, select {
        background: theme('colors.darkerblue');
    }

    button:disabled {
        background: rgb(255, 255, 255, 0.5);
    }
</style>