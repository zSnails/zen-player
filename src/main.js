import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';
import App from './App.vue'
import PlaylistView from "./components/PlaylistSelection";
import ThemeView from "./components/ThemeSelection";
import MusicView from "./components/MusicView";

const routes = [
    { path: '/', component: PlaylistView },
    { path: '/playlist/:id/music', component: MusicView },
    { path: '/themes', component: ThemeView },
];


const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

let app = createApp(App);
app.use(router);
app.mount('#app');
