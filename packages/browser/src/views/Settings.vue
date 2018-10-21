<template>
    <div>
        <h1 class="title">Settings</h1>

        <div class="columns">
            <div class="column is-one-third">
                <Endpoint />
            </div>
        </div>
    </div>
</template>

<script>
import Endpoint from '@/components/settings/Endpoint.vue';

export default {
    components: {
        Endpoint,
    },
};
</script>
