import Vue from 'vue';

import App from './App.vue';
import { Router } from './views';
import { Store } from './store';
import * as Socket from './socket';

import './assets/stylesheet.sass';

Vue.config.productionTip = false;

const router = Router();
const store = Store();
Socket.register(store);

const register = (el, component) => new Vue({
    el,
    router,
    store,
    render: h => h(component),
});

register('#app', App);
