<script lang="ts">
    import LoadingController from "./LoadingController";
    import { loadingAll, loadingMusicList } from "./stores";

    let imageElement: HTMLImageElement
    let animationClass = $state("animate__fadeIn");
    
    function restartAnimation(){
        imageElement.style.animation = "none";
        imageElement.offsetHeight;
        imageElement.style.animation = "";
    }
    
    function onAnimationEnd(){
        if(animationClass == "animate__fadeIn") return;
        imageElement.className += " hidden";
        LoadingController.setLoadingShow(true);
    }
    
    function handleLoading(){
        if(!LoadingController.loadingAll()) return;
        animationClass = "animate__fadeOut";
        restartAnimation();
    }
    
    loadingAll.subscribe(handleLoading);
    $effect(handleLoading);
</script>

<div class="fixed top-0 left-0 w-full h-full flex justify-center items-center">
    <div>
        <img bind:this={imageElement} class="w-60 animate__animated animate__faster {animationClass}" src="/icons/default/loading.gif" alt="Loading"
            onanimationend={onAnimationEnd}/>
    </div>
</div>