<template>
    <div>
        <h1 class="title">Spaceship</h1>
    </div>
</template>

<script>
import { createNamespacedHelpers } from 'vuex';

const { mapState } = createNamespacedHelpers('spaceship');

export default {
    name: 'spaceship',
    socket: {
        open() {
            const { id } = this.$route.params;

            this.$socket.send(JSON.stringify({ type: 'JoinCrew', id }));
        },
    },
};
</script>
