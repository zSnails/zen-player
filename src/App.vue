<template>
<main class="main-area">
    <router-view></router-view>
    <section class="cover-art">
        <div ref="coverImage" id="coverImage" :style="{ 'background': currentCover }" class="cover-image"></div>
        <h2 class="playlist-current"></h2>
    </section>
</main>
<div class="progress-bar">
</div>
</template>
<script>
export default {
  name: 'App',
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

@media screen and (max-width: 50vh) {
    .main-area {
        flex-direction: column;
        min-height: 0;
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
