export type Connection = {
  id: string
  name: string
  address: string
}

export type Alert = {
  id: string
  type: "success" | "error"
  message: string
}
