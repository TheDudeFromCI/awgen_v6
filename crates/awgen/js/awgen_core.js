export const log = (msg) => Deno.core.ops.op_log(msg);
globalThis.log = log;

export const sleep = (ms) => Deno.core.ops.op_sleep_async(ms);
globalThis.sleep = sleep;
