/* eslint-disable no-param-reassign */

import Vue from 'vue';
import Vuex from 'vuex';

import settings from './settings';
import socket from './socket';

export const Store = () => {
    Vue.use(Vuex);

    const store = new Vuex.Store({
        state: {
            socketed: false,
            spaceship: null,
        },
        mutations: {
            socketUpdate(state, status) {
                state.socketed = status;
            },
            crewed(state, spaceship) {
                state.spaceship = spaceship;
            },
        },
        actions: {

        },
        modules: {
            settings,
            socket,
        },
    });

    // Bus.$on('open', () => (store.commit('socketUpdate', true)));

    // Bus.$on('close', () => {
    //     store.commit('socketUpdate', false);
    //     store.commit('crewed', null);
    // });

    // Bus.$on('message', (event) => {
    //     const data = JSON.parse(event.data);

    //     switch (data.type) {
    //     case 'NewCaptain':
    //         store.commit('crewed', data.id);
    //         break;
    //     case 'LeaveCrew':
    //         store.commit('crewed', null);
    //         break;
    //     }
    // });

    store.subscribe((mutation, state) => {
        console.log('setitem ?');
        localStorage.setItem('together.settings', JSON.stringify(state.settings));
    });

    return store;
};
