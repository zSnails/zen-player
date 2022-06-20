<template>
    <section class="theme-section">
        <ul class="theme-list">
            <li class="theme-info" v-for="theme in themes" ref="li" :key="theme.name"> <!--on:click"selectTheme(theme)"-->
                <h2 class="theme-name" @click="selectTheme(theme)">{{ theme.name }}</h2>
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

.theme-section {
    display: flex;
    flex-direction: column;
    width: 50vw;
    margin: 40px;
    min-width: 50vh;
}

.theme-list {
    list-style-type: none;
    max-height: 75vh;
    overflow-x: hidden;
    padding: 0;
}

.theme-list::-webkit-scrollbar-track {
    background: rgba(0 0 0 0);
}

.theme-list::-webkit-scrollbar {
    width: 1vw;
}

.theme-list::-webkit-scrollbar-thumb {
    background: #FBFBFB;
    border-radius: 50px;
}

.theme-info {
    margin: 20px;
}

.theme-info:hover .theme-name {
    text-decoration: underline;
    text-decoration-color: var(--selected-color);
    text-decoration-thickness: 3px;
    cursor: default;
}

.theme-name {
    color: var(--foreground);
    margin-bottom: 5px;
    font-weight: 600;
    font-size: 25px;
    text-decoration: none;
}
</style>
