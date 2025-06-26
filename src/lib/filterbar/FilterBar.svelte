<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import { IconType } from "$lib/icon/types";
import { isMacos, isMobile } from "$lib/platform";
import { filterSearch } from "$lib/stores/filter";
import { mobileStatusBarHeight } from "$lib/stores/mobile";
</script>

<div class={`h-6 absolute top-0
    ${isMacos() ? "me-3 right-0" : "ms-3 left-0"}
    w-[calc(100%-24px)]
    sm:w-[calc(66.6667%-24px)]
    md-mdpi:w-[calc(40%-24px)]
    lg-mdpi:w-[calc(33.3334%-24px)]
    xl-mdpi:w-[calc(25%-24px)]
    md-hdpi:w-[calc(40%-24px)]
    lg-hdpi:w-[calc(33.3334%-24px)]`}
     style={`margin-top: ${isMobile() ? $mobileStatusBarHeight : 8}px;`}>
    <div class="bg-white/20 backdrop-blur-md text-white rounded-md shadow-md animate__animated animate__fadeIn animate__slow">
        <div class="w-full grid grid-cols-[auto_min-content] cursor-text px-3 py-1">
            <input
                class="w-full bg-transparent placeholder:text-white/70 text-white outline-none text-sm"
                placeholder="Search..."
                bind:value={$filterSearch}
            />
            <div class="w-4">
                <Icon type={IconType.Search} />
            </div>
        </div>
    </div>
</div>
