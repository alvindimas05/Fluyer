<script lang="ts">
import { DotLottieSvelte } from "@lottiefiles/dotlottie-svelte";
import { swipeMinimumTop } from "$lib/stores";
import MobileController from "$lib/controllers/MobileController";
</script>

<div class="fixed top-0 left-0 w-full h-full bg-gray-700/80 z-10 font-semibold text-center"
     style={`padding-top: ${$swipeMinimumTop}px;`}>

    <div class="overflow-visible h-28">
        <div class="h-40">
            <DotLottieSvelte loop autoplay src="/lotties/swipe-left.json" />
        </div>
    </div>
    <div>Swipe left to open the playlist</div>

    <div class="overflow-visible h-28">
        <div class="h-40">
            <DotLottieSvelte loop autoplay src="/lotties/swipe-right.json" />
        </div>
    </div>
    <div>Swipe right to open the menu</div>

    <div class="mt-3">Swipe in the opposite direction to close</div>

    <div class="w-full grid justify-center">
        <button class="w-fit bg-gray-500/80 rounded-md px-3 py-2 mt-3"
            onclick={MobileController.hideSwipeGuide}>Got it</button>
    </div>
</div>
