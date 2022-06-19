import { readable, writable } from 'svelte/store';

export const stepss = writable([]);
export const flow = writable([]);
export const workers = writable([]);
export const jobs = writable([]);
export const output = writable({});

export const stepName = writable('');
export const stepCode = writable('');
export const stepHash = writable('');

export const apiServer = readable('http://localhost:8080/api');