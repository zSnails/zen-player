<template>
    <section class="left-section">
        <button class="btn">+ Add song</button>
        <ul class="left-list">
            <li class="left-info" v-for="track in playlistMusic" ref="li" :key="track.id" @click="selectTrack(track)">
                <h3 class="left-name" :to="`/playlist/${track.id}/music`">{{ track.name }}</h3>
                <p class="left-description">{{ track.description }}</p>
            </li>
        </ul>
    </section>
</template>
<script>

import { store } from "../store.js";
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: 'MusicView',
    data() {
        return {
            music: []
        }
    },
    async beforeMount() {
        this.music = await invoke("get_music", { playlistId: parseInt(this.$route.params.id) });
    },
    methods: {
        selectTrack(track) {
            store.cover = track.cover;
        }
    },
    computed: {
        playlistMusic() {
            return this.music; //.filter(m => this.playlist.id);
        }
    }
}
</script>
<style>
</style>
