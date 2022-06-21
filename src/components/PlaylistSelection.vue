<template>
    <section ref="modal" class="modal">
        <div class="modal-content">
            <div class="form-element">
                <label class="form-label normal-heading">Name</label>
                <input class="form-control" name="new-playlist-name" v-model="newPlaylistName" required>
            </div>
            <div class="form-element">
                <label class="form-label normal-heading">Description</label>
                <textarea rows="3" class="form-control" name="new-playlist-name" v-model="newPlaylistDescription" required></textarea>
            </div>
            <div class="form-element">
                <label class="form-label normal-heading">Cover art</label>
                <input type="file" class="form-control" name="cover-art-location" @change="updatePlaylistCover" required/>
            </div>
            <div class="button-row">
                <button class="btn btn-submit" @click="submitNewPlaylist()">Done</button>
                <button class="btn btn-cancel" @click="cancelPlaylistCreation()">Cancel</button>
            </div>
        </div>
    </section>
    <section class="left-section">
        <button class="btn" @click="createPlaylistModal()">+ New playlist</button>
        <ul class="left-list">
            <li class="left-info" v-for="playlist in playlists" ref="li" :key="playlist.id" @click="selectPlaylist(playlist)">
                <router-link class="left-name" :to="`/playlist/${playlist.id}/music`">{{ playlist.name }}</router-link>
                <p class="left-description">{{ playlist.description }}</p>
            </li>
        </ul>
    </section>
</template>

<script>
import { /*convertFileSrc,*/ invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { store } from '../store.js';


export default {
    name: 'PlaylistSelection',
    data() {
        return {
            newPlaylistName: "",
            newPlaylistDescription: "",
            newPlaylistCover: "",
            playlists: []
        }
    },
    async beforeMount() {
        this.playlists = await invoke('get_playlists');
        listen('backend:playlist-create', event => {
            this.playlists.push(event.payload);
        });
    },
    methods: {
        updatePlaylistCover(event) {
            console.log(event);
            this.newPlaylistCover = event.target.files[0];
        },
        selectPlaylist(playlist) {
            store.cover = playlist.cover;
        },
        cancelPlaylistCreation() {
            this.$refs["modal"].style.display = "none";
            this.newPlaylistDescription = "";
            this.newPlaylistName = "";
            this.newPlaylistCover = "";
        },
        createPlaylistModal() {
            this.$refs["modal"].style.display = "block";
        },
        async submitNewPlaylist() {
            // TODO: get an actual image from the users pc
            await invoke('create_playlist', {name: this.newPlaylistName, description: this.newPlaylistDescription, cover: this.newPlaylistCover});
            this.cancelPlaylistCreation();
        }
    }
}
</script>
<style>



/* .playlist-list::-webkit-scrollbar-track { */
/*     background: rgba(0 0 0 0); */
/* } */

/* .playlist-list::-webkit-scrollbar { */
/*     width: 1vw; */
/* } */

/* .playlist-list::-webkit-scrollbar-thumb { */
/*     background: #FBFBFB; */
/*     border-radius: 50px; */
/* } */


</style>
