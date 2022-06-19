<template>
    <section class="playlist-section">
        <ul class="playlist-list">
            <li class="playlist-info" v-for="playlist in playlists" ref="li" :key="playlist.name" v-on:click="selectPlaylist(playlist)">
                <h2 class="playlist-name">{{ playlist.name }}</h2>
                <p class="playlist-description">{{ playlist.description }}</p>
            </li>
        </ul>
    </section>
    <section class="cover-art">
        <div ref="coverImage" id="coverImage" :style="{ 'background': currentCover }" class="cover-image"></div>
        <h2 class="playlist-current"></h2>
    </section>
</template>

<script>
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

export default {
    name: 'PlaylistSelection',
    data() {
        return {
            currentCover: '',
            playlists: [
                {name: 'Troncha', description: 'Tortillon', cover: '/home/tholly/pictures/taotao.png'},
            ]
        }
    },
    async beforeMount() {
        console.debug('loading playlists');
        this.playlists = await invoke('get_playlists');
        console.log(this.playlists)
    },
    methods: {
        selectPlaylist(playlist) {
            document.getElementById('coverImage').style['background-image'] = `linear-gradient(90deg, #141419 0%, rgba(255, 255, 255, 0) 100%), url(${convertFileSrc(playlist.cover)})`
            /* this.currentCover =     /1* background: linear-gradient(90deg, #141419 0%, rgba(255, 255, 255, 0) 100%), url('/home/tholly/pictures/taotao.png'); */
        }
    }
}
</script>
<style>
.playlist-section {
    display: flex;
    flex-direction: row;
    width: 50vw;
    margin: 40px;
    min-width: 50vh;
}

.playlist-list {
    list-style-type: none;
    max-height: 75vh;
    overflow-x: hidden;
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
    transition: 500ms;
}


.playlist-info:hover {
    width: 2rem;
    height: 1.3rem;
}

.playlist-info:hover .playlist-description {
    text-decoration: underline;
    text-decoration-color: var(--selected-color);
    text-decoration-thickness: 3px;
    cursor: default;
}

.playlist-info:hover .playlist-name {
    cursor: default;
}
.playlist-name {
    color: var(--foreground);
    margin-bottom: 5px;
}

.playlist-description {
    color: var(--foreground-secondary);
}

.cover-image {
    transition: 200ms;
    /* background: linear-gradient(90deg, #141419 0%, rgba(255, 255, 255, 0) 100%), url('/home/tholly/pictures/taotao.png'); */
    background-size: cover;
    background-position: center center;
    width: 100vh;
    height: 100vh;
}
</style>
