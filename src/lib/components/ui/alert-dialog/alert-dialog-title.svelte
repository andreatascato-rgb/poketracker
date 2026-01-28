<script lang="ts">
	import { AlertDialog } from "bits-ui";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: import("bits-ui").AlertDialogTitleProps & { class?: string; ref?: import("svelte").HTMLAttributes<HTMLHeadingElement>["ref"] } = $props();
</script>

<AlertDialog.Title
	bind:ref
	data-slot="alert-dialog-title"
	class={cn("text-lg leading-none font-semibold", className)}
	{...restProps}
/>
