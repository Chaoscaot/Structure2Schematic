<script lang="ts">
import Converter from "./lib/Converter.svelte";
</script>

<div>
    <nav>
        <span>Structure2Schematic</span>
    </nav>

    <Converter />

    <footer>
        <a href="https://github.com/Chaoscaot/Structure2Schematic">GitHub</a>
    </footer>
</div>

<style>
    footer {
        display: flex;
        width: 100vw;

        border-top: 1px solid #1e1f22;
        margin-top: 4rem;
        padding-top: 1rem;

        justify-content: center;
        align-items: center;
    }

    a {
        text-decoration: none;
        color: #6c6d73;
    }

    div {
        height: 100vh;
        width: 100vw;

        background: black;
        color: white;
    }

    nav {
        width: 100vw;
        height: 3rem;
        display: flex;

        >span {
            font-weight: bold;
            font-size: 1.5rem;

            margin: auto;

        }
    }
</style>