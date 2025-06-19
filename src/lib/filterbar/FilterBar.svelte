<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import {IconType} from "$lib/icon/types";
import {isMobile} from "$lib/platform";
import {mobileStatusBarHeight} from "$lib/stores/mobile";
import {filterSearch} from "$lib/stores/filter";
</script>

<div class="w-full px-3 pb-3 text-sm grid auto-cols-[50%] lg:auto-cols-[20%]
    animate__animated animate__fadeIn"
     style={`padding-top: ${isMobile() ? $mobileStatusBarHeight : 32}px`}>
    <div class="bg-white/10 text-white rounded-md me-4">
        <div class="w-full grid grid-cols-[auto_min-content] cursor-text px-2 py-1">
            <input
                class="w-full bg-transparent placeholder:text-gray-300 outline-none"
                placeholder="Search..."
                bind:value={$filterSearch}
            />
            <div class="w-4">
                <Icon type={IconType.Search} />
            </div>
        </div>
    </div>
</div>