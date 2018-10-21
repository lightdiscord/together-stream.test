import Vue from 'vue';
import ReconnectingWebSocket from 'reconnecting-websocket';

import App from './App.vue';
import { Router } from './views';
import { Store } from './store';
import Socket from './socket';

import './assets/stylesheet.sass';

Vue.config.productionTip = false;

const router = Router();
const store = Store();

const url = () => store.state.settings.endpoint;
const socket = new ReconnectingWebSocket(url);

Vue.use(Socket, socket, store);

store.watch(state => state.settings.endpoint, () => socket.reconnect());

const register = (el, component) => new Vue({
    el,
    router,
    store,
    render: h => h(component),
});

register('#app', App);
