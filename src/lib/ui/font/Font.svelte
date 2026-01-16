<script lang="ts">
	import { isAndroid, isLinux, isWindows } from '$lib/platform';

	let condition = isAndroid() || isLinux() || isWindows();
	if (condition) {
		import('@fontsource/inter/latin-400.css');
		import('@fontsource/inter/latin-500.css');
		import('@fontsource/inter/latin-600.css');
		import('@fontsource/inter/latin-700.css');
	}
</script>

{#if condition}
	<style>
		* {
			font-family: 'Inter', sans-serif;
		}
	</style>
{/if}
