<script lang="ts">
interface Props {
	title: string;
	description: string;
}

const { title, description } = $props<Props>();
</script>
<p class="font-semibold">{title}</p>
<p>{description}</p>