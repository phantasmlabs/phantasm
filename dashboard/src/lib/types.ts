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

export type ApprovalRequest = {
  id: string
  name: string
  parameters: string
}

export type ApprovalResponse = {
  id: string
  status: "approved" | "modified" | "denied"
  parameters: string
}
