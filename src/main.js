import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';
import App from './App.vue'
import PlaylistView from "./components/PlaylistSelection";

const routes = [
    { path: '/', component: PlaylistView },
    // { path: '/:playlistName', component: PlaylistContent },
];


const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

let app = createApp(App);
app.use(router);
app.mount('#app');
