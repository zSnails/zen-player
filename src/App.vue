<template>
<main class="main-area">
    <router-view></router-view>
    <section class="cover-art">
        <div ref="coverImage" id="coverImage" :style="currentCover" class="cover-image"></div>
        <h2 class="playlist-current"></h2>
    </section>
</main>
<ProgressBar max="120" progress="60"/>
</template>
<script>

import ProgressBar from "./components/ProgressBar";
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { store } from './store.js';

export default {
  name: 'App',
  components: {
    ProgressBar,
  },
  computed: {
    currentCover() {
        console.log("niggers")
        return {
            'background-image': `linear-gradient(90deg, #141419 0%, rgba(255, 255, 255, 0) 100%), url(${convertFileSrc(store.cover)})`,
        };
    }
  }
}
</script>

<style>

:root {
    --scrollbar-color: #FBFBFB;
    --selected-color: #FBFBFB;
    --background: #14141A;
    --foreground: #FBFBFB;
    --foreground-secondary: #BDBDBD;
    -webkit-user-select: none; 
    -ms-user-select: none; 
    user-select: none; 
}

@media screen and (max-width: 1030px) {
    .cover-image {
        display: none;
    }
}

::-webkit-scrollbar-track {
    background: rgba(0 0 0 0);
}

::-webkit-scrollbar {
    width: 1vw;
}

::-webkit-scrollbar-thumb {
    background: var(--scrollbar-color);
}

body {
    margin: 0;
    padding: 0;
    background: var(--background);
    overflow-y: hidden;
    overflow-x: hidden;
}

.main-area {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    min-height: 50vh;
}

.cover-image {
    transition: 200ms;
    background-size: cover;
    background-position: center center;
    width: 100vh;
    height: 100vh;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
}
</style>
