<script lang="ts">
    import { convert_structure } from 's2s_web'
    import saveAs from "file-saver"

    function convertFiles(files: FileList) {
        for (const file of files) {
            const reader = new FileReader()

            reader.readAsArrayBuffer(file)

            reader.onload = () => {
                const arrayBuffer = reader.result as ArrayBuffer
                const uint8Array = new Uint8Array(arrayBuffer)

                const schem = convert_structure(uint8Array)

                saveAs(new Blob([schem]), file.name.replace(".nbt", ".schem").replace(".mcstructure", ".schem"), {
                    autoBom: false,
                })
            }
        }

        input.value = ""
    }

    let input: HTMLInputElement
</script>

<label ondrop={(e) => {
    e.preventDefault()
    convertFiles(e.dataTransfer.files)
}} ondragover={(e) => e.preventDefault()}>
    Select .nbt or .mcstructure files to convert to .schem
    <input bind:this={input} type="file" multiple accept=".nbt,.mcstructure" onchange={(e) => convertFiles(e.currentTarget.files)}>
</label>

<style>
    label {
        display: flex;

        justify-content: center;
        align-items: center;

        width: 80%;
        height: 70%;

        background: #1e1f22;
        border-radius: 2rem;
        margin: auto;

    }

    label:hover {
        cursor: pointer;
    }

    label:focus-visible {
        background: #2e2f32;
    }

    input {
        display: none;
    }
</style>