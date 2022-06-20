import { createApp } from 'vue';
import { createRouter, createWebHashHistory } from 'vue-router';
import App from './App.vue'
import PlaylistView from "./components/PlaylistSelection";
import ThemeView from "./components/ThemeSelection"

const routes = [
    { path: '/', component: ThemeView },
    { path: '/pl', component: PlaylistView },
];


const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

let app = createApp(App);
app.use(router);
app.mount('#app');
