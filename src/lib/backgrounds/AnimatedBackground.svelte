<script lang="ts">
    import './background.scss';
    import ColorsBackground from "./ColorsBackground.svelte";
    
    export let album_path;
</script>
<div class="bg-blur"/>
<div class="bg-blur-heart">
    <ColorsBackground {album_path}/>
</div>
<div>
    <ColorsBackground {album_path}/>    
</div>