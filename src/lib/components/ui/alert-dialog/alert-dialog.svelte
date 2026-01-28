<script lang="ts">
	import { AlertDialog } from "bits-ui";
	import type { AlertDialogRootProps } from "bits-ui";
	import type { Snippet } from "svelte";

	let { open = $bindable(false), children, ...restProps }: AlertDialogRootProps & { children?: Snippet } = $props();
</script>

<AlertDialog.Root bind:open {...restProps}>
	{#if children}
		{@render children()}
	{/if}
</AlertDialog.Root>
