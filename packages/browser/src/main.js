import Vue from 'vue';

import App from './App.vue';
import { Router } from './views';
import { Store } from './store';
import * as Socket from './socket';

import './assets/stylesheet.sass';

Vue.config.productionTip = false;

const store = Store();
Socket.register(store);

const register = (el, component) => new Vue({
    el,
    router: Router,
    store,
    render: h => h(component),
});

console.log(store);

register('#app', App);
