<script lang="ts">
	interface Props {
		title?: string;
		description?: string;
	}

	const { title, description } = $props<Props>();
</script>

<div class="mx-5">
	{#if title}
		<p class="font-semibold">{title}</p>
	{/if}
	{#if description}
		<p>{description}</p>
	{/if}
</div>
