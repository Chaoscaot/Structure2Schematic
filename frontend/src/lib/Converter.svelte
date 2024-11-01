<script lang="ts">
    import { convert_structure } from 's2s_web'
    import saveAs from "file-saver"

    function onFileChange(event: Event & { currentTarget: HTMLInputElement }) {
        for (const file of event.currentTarget.files!) {
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

        event.currentTarget.value = ''
    }
</script>

<input type="file" multiple onchange={onFileChange}>