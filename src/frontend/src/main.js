import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import Site from './Site.vue';
import Home from './components/Home.vue';
import NotFound from './components/NotFound.vue';

const routes = [
	{ path: '/', component: Home },
	{ path: '/:pathMatch(.*)*', component: NotFound },
]

const router = createRouter({
	// 4. Provide the history implementation to use. We are using the hash history for simplicity here.
	history: createWebHistory(),
	routes,
})

const site = createApp(Site);
site.use(router);
site.mount('#site');
