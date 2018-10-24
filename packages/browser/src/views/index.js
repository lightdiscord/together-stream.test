import Vue from 'vue';
import VueRouter from 'vue-router';
import Home from './Home.vue';

Vue.use(VueRouter);

export const Router = new VueRouter({
    routes: [
        {
            path: '/',
            name: 'home',
            component: Home,
        },
        {
            path: '/settings',
            name: 'settings',
            component: () => import(/* webpackChunkName: "settings" */ './Settings.vue'),
        },
        {
            path: '/join/:id',
            name: 'spaceship',
            component: () => import(/* webpackChunkName: "spaceship" */ './Spaceship.vue'),
        },
    ],
});
