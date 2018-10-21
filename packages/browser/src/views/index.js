import Vue from 'vue';
import VueRouter from 'vue-router';
import Home from './Home.vue';

export const Router = () => {
    Vue.use(VueRouter);

    return new VueRouter({
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
        ],
    });
};
