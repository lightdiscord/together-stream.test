/* eslint-disable no-param-reassign */

import Vue from 'vue';

import Observer from './observer';

export default {
    install(vue, connection, store) {
        if (!connection) throw new Error("Can't locate connection");

        const bus = new Vue();
        const observer = Observer(bus, connection, store);

        vue.prototype.$socket = observer.socket;

        Vue.mixin({
            created() {
                const { socket } = this.$options;

                this.$options.socket = new Proxy({}, {
                    set: (target, key, value) => {
                        bus.$on(key, value);
                        target[key] = value;

                        return true;
                    },
                    deleteProperty: (target, key) => {
                        bus.$off(key, this.$options.socket[key]);
                        delete target.key;

                        return true;
                    },
                });

                if (socket) {
                    Object.keys(socket).forEach((key) => {
                        this.$options.socket[key] = socket[key];
                    });
                }
            },
            beforeDestroy() {
                const { socket } = this.$options;

                if (socket) {
                    Object.keys(socket).forEach((key) => {
                        delete this.$options.socket[key];
                    });
                }
            },
        });
    },
};
