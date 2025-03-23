<script lang="ts">
import { isAndroid, isLinux, isWindows } from "$lib/platform";

let condition = isAndroid() || isLinux() || isWindows();
if (condition) {
	import("@fontsource/inter/400.css");
	import("@fontsource/inter/500.css");
	import("@fontsource/inter/600.css");
	import("@fontsource/inter/700.css");
}
</script>

{#if condition}
    <style>
        * {
            font-family: "Inter", sans-serif;
        }
    </style>
{/if}
