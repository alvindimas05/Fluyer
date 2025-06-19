<script lang="ts">
import Icon from "$lib/icon/Icon.svelte";
import {IconType} from "$lib/icon/types";
import {filterSearch} from "$lib/stores/filter";
</script>

<div class="w-full pt-2 pb-3 pe-3 text-sm grid justify-end md:auto-cols-[50%] lg:auto-cols-[20%]
    animate__animated animate__slow animate__fadeInDown">
    <div class="bg-white/10 text-white rounded-md ms-5">
        <div class="w-full grid grid-cols-[auto_min-content] cursor-text px-2 py-1">
            <input
                class="w-full bg-transparent placeholder:text-white/80 outline-none"
                placeholder="Search..."
                bind:value={$filterSearch}
            />
            <div class="w-4">
                <Icon type={IconType.Search} />
            </div>
        </div>
    </div>
</div>