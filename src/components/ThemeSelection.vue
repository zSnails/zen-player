<template>
    <section class="theme-selection">
        <ul class="theme-list">
            <li class="theme-info" v-for"theme in themes" ref="li" :key="theme.name" v-on:click"selectTheme(theme)">
                <h2 class="theme-name">{{ theme.name }}</h2>
            </li>
        </ul>
    </section>
    <section class="covert-art">
        <div ref="coverImage" id="coverImage" :style"{ 'background': currentCover } class="cover-image"></div>
        <div class="color1"><p></p></div>
        <div class="color2"><p></p></div>
        <div class="color3"><p></p></div>
    </section>
</template>

<script>
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

export default {
    name: 'ThemeSelection',
    data() {
        return {
            currentCover: '',
            themes: [
                {name: 'dark', covert: '/home/johan/Projects/tauri/zen-player/src/assets/zen-player-dark-logo.png'},
            ]
        }
    },
    async beforeMount() {
        console.debug('loading themes');
        this.themes = await invoke('get_themes');
        console.log(this.themes)
    },
    methods: {
        selectTheme(theme) {
            document.getElementById('coverImage').style['background-image'] = 'linear-gradient(90deg, #141419 0%, rgba(255, 255, 255, 0) 100%), url(${convertFileSrc(theme.cover)})'
        }
    }
}
</script>
<style>
@media screen and (max-width: 800px) {

}
</style>
