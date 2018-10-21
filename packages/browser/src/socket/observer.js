export default (bus, socket, store) => {
    console.log(!!store);

    ['close', 'open', 'error', 'message'].forEach((name) => {
        socket.addEventListener(name, (event) => {
            bus.$emit(name, event);

            if (store) {
                store.dispatch(`socket/${name}`, event);
            }
        });
    });

    return { socket };
};
