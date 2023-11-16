import registerWebworker from "webworker-promise/lib/register";
import init from "../dist/wasm/rezasm_wasm.js";
import {
    wasm_load,
    wasm_step,
    wasm_stop,
    wasm_reset,
    wasm_is_completed,
    wasm_get_exit_status,
    wasm_get_memory_bounds,
    wasm_get_memory_slice,
    wasm_get_register_value,
    wasm_get_register_names,
    wasm_get_register_values,
    wasm_get_word_size,
    wasm_receive_input
} from "../dist/wasm";

registerWebworker(async (message, emit) => {
    const command = message.command;
    const data = message.argument;

    try {
        if (command === "ping") {
            await init();
            console.log("WebAssembly code loaded");
            return "pong";
        } else if (command === "load") {
            if (data === undefined) {
                throw "Call to 'load' without providing string data";
            }
            return wasm_load(data);
        } else if (command === "step") {
            return wasm_step();
        } else if (command === "stop") {
            return wasm_stop();
        } else if (command === "reset") {
            return wasm_reset();
        } else if (command === "is_completed") {
            return wasm_is_completed();
        } else if (command === "get_exit_status") {
            return wasm_get_exit_status();
        } else if (command === "get_register_value") {
            if (data === undefined) {
                throw "Call to 'get_register_value' without providing string data";
            }
            return wasm_get_register_value(data);
        } else if (command === "get_register_names") {
            return wasm_get_register_names();
        } else if (command === "get_register_values") {
            return wasm_get_register_values();
        } else if (command === "get_memory_bounds") {
            return wasm_get_memory_bounds();
        } else if (command === "get_memory_slice") {
            if (data === undefined || data.address === undefined || data.length === undefined) {
                throw "Call to 'get_register_value' without providing address or length data";
            }
            return wasm_get_memory_slice(data.address, data.length);
        } else if (command === "get_word_size") {
            return wasm_get_word_size();
        } else if (command === "receive_input") {
            if (data === undefined || data.data === undefined) {
                throw "Call to 'receive_input' without providing string to send";
            }
            return wasm_receive_input(data.data);
        } else {
            throw `Invalid command: '${command}'`;
        }
    } catch (error) {
        console.log(error);
        throw new Error(error);
    }
});
