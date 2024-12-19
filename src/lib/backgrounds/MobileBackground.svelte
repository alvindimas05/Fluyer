<script lang="ts">
    import LoadingController from "$lib/controllers/LoadingController";
</script>

<div
    class="fixed z-[-10] w-full h-full animate__animated animate__fadeIn"
    onanimationend={() => LoadingController.setLoadingBackground(true)}
>
    <img
        class="w-full h-full"
        src="/images/mobile-background.png"
        alt="Background"
    />

    <img
        class="w-full h-full bg-blur-heart"
        src="/images/mobile-background.png"
        alt="Background"
    />
</div>
