<template>
<main class="main-area">
    <PlaylistSelection/>
</main>
<div class="progress-bar">
</div>
</template>

<script>

import PlaylistSelection from './components/PlaylistSelection';

export default {
  name: 'App',
  components: {
    PlaylistSelection,
  }
}
</script>

<style>

:root {
    --scrollbar-color: #FBFBFB;
    --selected-color: #FBFBFB;
    --foreground: #FBFBFB;
    --foreground-secondary: #BDBDBD;
    -webkit-user-select: none; 
    -ms-user-select: none; 
    user-select: none; 
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
    background: #14141A;
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

.progress-bar {
    display: block;
    position: absolute;
    bottom: 0;
    left: 0;
    height: 10px;
    width: 20%;
    background-color: #FFF;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
}
</style>
