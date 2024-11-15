import { writable } from "svelte/store"
import type { Connection, Alert, Approver } from "./types.ts"

export const approver = writable<Approver | undefined>(undefined)
export const connections = writable<Connection[]>([])
export const alerts = writable<Alert[]>([])
