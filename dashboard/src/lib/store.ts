import { writable } from "svelte/store"
import type { Connection } from "./types.ts"

export const connections = writable<Connection[]>([])
