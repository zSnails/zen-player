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
            this.changeTheme(theme);
        },
        changeTheme(theme) {
            console.log("changeTheme")
            document.body.style.setProperty("--background", theme.colors.background);
            document.body.style.setProperty("--foreground", theme.colors.foreground);
            document.body.style.setProperty("--foreground-secondary", theme.colors.secondary_foreground);
            document.body.style.setProperty("--selected-color", theme.colors.foreground);
            document.body.style.setProperty("--scrollbar-color", theme.colors.foreground);
        }
    }
}
</script>
<style>
</style>
