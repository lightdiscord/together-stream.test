/* eslint-disable no-param-reassign */

export const updateEndpoint = (state, endpoint) => {
    state.endpoint = endpoint;
};

export const initialize = (state, settings) => {
    state = settings;
};
