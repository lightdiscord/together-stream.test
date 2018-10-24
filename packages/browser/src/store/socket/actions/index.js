export const close = ({ commit }, event) => {
    console.log('close', event);
    commit('status', false);
};

export const open = ({ commit }, event) => {
    console.log('open', event);
    commit('status', true);
};

export const error = (context, event) => {
    console.log('error', event);
};

export const message = (context, event) => {
    console.log('message', event);
    const data = JSON.parse(event.data);

    const actions = ({
        NewCaptain: ({ commit }, event, data) => {
            commit('spaceship/id', data.id, { root: true });
        },

        LeaveCrew: ({ commit }) => {
            commit('spaceship/id', null, { root: true });
        },
    });

    const action = actions[data.type];

    if (action) {
        action(context, event, data);
    }
};
