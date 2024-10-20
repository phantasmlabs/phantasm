import { writable } from "svelte/store"
import type { Connection, Alert } from "./types.ts"

export const connections = writable<Connection[]>([])
export const alerts = writable<Alert[]>([])
