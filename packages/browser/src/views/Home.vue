<template>
    <div>
        <h1 class="title">Home</h1>

        <template v-if="!spaceship || !connected">
            <button class="button is-primary" @click="becomeCaptain()" :disabled="!connected">
                Become a spaceship captain!
            </button>
        </template>
        <template v-else>
            <p>You're in the crew of the spaceship <strong>{{ spaceship }}</strong>.</p>

            <a class="button is-primary" @click="leaveCrew">
                Leave your crew
            </a>
        </template>
    </div>
</template>

<script>
import Vue from 'vue';
import { mapState, createNamespacedHelpers } from 'vuex';

const { mapState: mapSocketState } = createNamespacedHelpers('socket');

export default Vue.extend({
    computed: {
        ...mapState(['spaceship']),
        ...mapSocketState(['connected']),
    },
    socket: {
        message(message) {
            console.log('new socket message', message);
        },
    },
    methods: {
        becomeCaptain() {
            this.$socket.send(JSON.stringify({ type: 'BecomeCaptain' }));
        },
        leaveCrew() {
            this.$socket.send(JSON.stringify({ type: 'LeaveCrew' }));
        },
    },
});
</script>
