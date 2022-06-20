<template>
    <section class="left-section">
        <ul class="left-list">
            <li class="left-info" v-for="theme in themes" ref="li" :key="theme.name"> <!--on:click"selectTheme(theme)"-->
                <h2 class="left-name" @click="selectTheme(theme)">{{ theme.name }}</h2>
            </li>
        </ul>
    </section>
</template>
<script>
import { /*convertFileSrc,*/ invoke } from '@tauri-apps/api/tauri';
import { store } from "../store.js";
export default {
    name: 'ThemeSelection',
    data() {
        return {
            themes: []
        }
    },
    async beforeMount() {
        this.themes = await invoke('get_themes');
    },
    methods: {
        selectTheme(theme) {
            store.cover = theme.cover;
        }
    }
}
</script>
<style>
</style>
