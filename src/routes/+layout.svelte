<script lang="ts">
	import Dialog from "$lib/component/dialog/Dialog.svelte";
	import Toaster from "$lib/component/toast/Toaster.svelte";
</script>

<slot />

<Dialog />
<Toaster />
