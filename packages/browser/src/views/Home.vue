<template>
    <div>
        <h1 class="title">Home</h1>

        <template v-if="!id || !connected">
            <button class="button is-primary" @click="becomeCaptain()"
                :disabled="!connected"
                :class="{ 'is-loading': !connected }">
                Become a spaceship captain!
            </button>
        </template>
        <template v-else>
            <p>You're in the crew of the spaceship <strong>{{ id }}</strong>.</p>

            <div class="field">
                <div class="control">
                    <input type="text" class="input" :value="`http://localhost:8080/#/join/${ id }`">
                </div>
            </div>

            <a class="button is-primary" @click="leaveCrew">
                Leave your crew
            </a>
        </template>
    </div>
</template>

<script>
import Vue from 'vue';
import { createNamespacedHelpers } from 'vuex';

const { mapState } = createNamespacedHelpers('socket');
const { mapState: mapSpaceshipState } = createNamespacedHelpers('spaceship');

export default Vue.extend({
    computed: {
        ...mapState(['connected']),
        ...mapSpaceshipState(['id']),
    },
    socket: {
        message(message) {
            const data = JSON.parse(message.data);

            if (data.type === 'NewCaptain') {
                this.$router.push({ name: 'spaceship', params: { id: data.id } });
            }
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
