<template>
    <div>
        <div class="field">
            <label class="label">Endpoint</label>
            <div class="control">
                <input type="text" class="input"
                    :class="{ 'is-danger': !validity }"
                    v-model="value">
            </div>
            <p v-if="!validity" class="help is-danger">Incorrect endpoint</p>
        </div>
        <div class="control">
            <a class="button is-primary" @click="save">Save</a>
        </div>
    </div>
</template>

<script>
import { createNamespacedHelpers } from 'vuex';

const { mapMutations } = createNamespacedHelpers('settings');

export default {
    data() {
        return {
            value: '',
        };
    },
    computed: {
        validity() {
            try {
                const url = new URL(this.value);
                return ['ws:', 'wss:'].includes(url.protocol);
            } catch (e) {
                return false;
            }
        },
    },
    mounted() {
        this.value = this.$store.state.settings.endpoint;
        this.$store.watch(state => state.settings.endpoint, endpoint => (this.value = endpoint));
    },
    methods: {
        ...mapMutations(['updateEndpoint']),

        save() {
            if (this.validity) {
                this.updateEndpoint(this.value);
            }
        },
    },
};
</script>
