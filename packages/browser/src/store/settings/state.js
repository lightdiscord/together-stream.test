const fallback = {
    endpoint: 'ws://localhost:8000/ws',
};

export default {
    ...fallback,
    ...JSON.parse(localStorage.getItem('together.settings') || '{}'),
};
