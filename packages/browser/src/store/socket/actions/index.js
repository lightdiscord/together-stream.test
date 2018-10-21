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
};
