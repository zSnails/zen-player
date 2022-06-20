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

@media screen and (max-width: 800px) {
    .cover-art {
        display: none;
    }
    .left-section {
        margin: 0;
    }
    .left-list {
        padding: 0;
    }
}

.left-section {
    display: flex;
    flex-direction: column;
    width: 50vw;
    margin: 40px;
    min-width: 50vh;
}

.left-list {
    list-style-type: none;
    max-height: 75vh;
    overflow-x: hidden;
    padding: 0;
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

.left-info {
    margin: 20px;
}

.left-info:hover .left-description {
    cursor: default;
}

.left-info:hover .left-name {
    text-decoration: underline;
    text-decoration-color: var(--selected-color);
    text-decoration-thickness: 3px;
    cursor: default;
}
.left-name {
    color: var(--foreground);
    margin-bottom: 5px;
    font-weight: 600;
    font-size: 25px;
    text-decoration: none;
}

.left-description {
    color: var(--foreground-secondary);
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
}
</style>
