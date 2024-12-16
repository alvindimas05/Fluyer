<script lang="ts">
    interface Props {
        name: string;
        onclick: (
            e: MouseEvent & {
                currentTarget: EventTarget & HTMLButtonElement;
            },
        ) => any;
    }

    let { name, onclick }: Props = $props();
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<button class="hover:bg-gray-300/25 p-2 rounded" {onclick}>
    <img
        class="invert w-4"
        src={`/icons/default/${name}.png`}
        alt={name.toUpperCase()}
    />
</button>
