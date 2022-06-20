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
            <div class="button-row">
                <button class="btn btn-submit" @click="submitNewPlaylist()">Done</button>
                <button class="btn btn-cancel" @click="cancelPlaylistCreation()">Cancel</button>
            </div>
        </div>
    </section>
    <section class="playlist-section">
        <button class="btn" @click="createPlaylistModal()">+ New playlist</button>
        <ul class="playlist-list">
            <li class="playlist-info" v-for="playlist in playlists" ref="li" :key="playlist.id" @click="selectPlaylist(playlist)">
                <router-link class="playlist-name" :to="`/${playlist.name}`">{{ playlist.name }}</router-link>
                <p class="playlist-description">{{ playlist.description }}</p>
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
        selectPlaylist(playlist) {
            store.cover = playlist.cover;
        },
        cancelPlaylistCreation() {
            this.$refs["modal"].style.display = "none";
            this.newPlaylistDescription = "";
            this.newPlaylistName = "";
        },
        createPlaylistModal() {
            this.$refs["modal"].style.display = "block";
        },
        async submitNewPlaylist() {
            // TODO: get an actual image from the users pc
            await invoke('create_playlist', {name: this.newPlaylistName, description: this.newPlaylistDescription, cover:"sexo"});
            this.cancelPlaylistCreation();
        }
    }
}
</script>
<style>
@media screen and (max-width: 800px) {
    .cover-art {
        display: none;
    }
    .playlist-section {
        margin: 0;
    }
    .playlist-list {
        padding: 0;
    }
}

.form-control {
    background-color: var(--foreground);
    color: var(--background);
    margin-left: 5px;
    padding: 4px;
    border-radius: 50px;
    outline: none;
}

.modal {
    display: none;
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: none;
    background-color: rgba(0,0,0, 0.5);
}

.modal-content {
    background-color: var(--background);
    display: flex;
    flex-direction: column;
    min-height: 3rem;
    margin: 40vh auto;
    padding: 20px;
    border-radius: 15px;
    width: 35%;
}

.form-element {
    margin: 10px;
    display: flex;
    width: 100%;
    flex-direction: row;
    flex-grow: 1;
    text-align: center;
}

.form-element .form-control {
    width: 100%;
}

.form-element .form-label {
    width: 25%;
}

.form-element textarea {
    resize: none;
    overflow: none;
    border-radius: 5px;
    max-height: 3;
}

.normal-heading {
    color: var(--foreground);
    font-size: 20px;
}

.normal-text {
    color: var(--foreground);
    font-size: 16px;
}

.btn.btn-cancel {
    color: #ff5b5b !important;
}

.btn {
    width: 10.0rem;
    background: transparent;
    border: none;
    color: var(--foreground);
}

.button-row {
    display: flex;
    flex-direction: row;
    width: 100%;
}

.button-row .btn {
    width: 100%;
}

.playlist-section {
    display: flex;
    flex-direction: column;
    width: 50vw;
    margin: 40px;
    min-width: 50vh;
}

.playlist-list {
    list-style-type: none;
    max-height: 75vh;
    overflow-x: hidden;
    padding: 0;
}


.playlist-list::-webkit-scrollbar-track {
    background: rgba(0 0 0 0);
}

.playlist-list::-webkit-scrollbar {
    width: 1vw;
}

.playlist-list::-webkit-scrollbar-thumb {
    background: #FBFBFB;
    border-radius: 50px;
}

.playlist-info {
    margin: 20px;
}

.playlist-info:hover .playlist-description {
    cursor: default;
}

.playlist-info:hover .playlist-name {
    text-decoration: underline;
    text-decoration-color: var(--selected-color);
    text-decoration-thickness: 3px;
    cursor: default;
}
.playlist-name {
    color: var(--foreground);
    margin-bottom: 5px;
    font-weight: 600;
    font-size: 25px;
    text-decoration: none;
}

.playlist-description {
    color: var(--foreground-secondary);
}

</style>
