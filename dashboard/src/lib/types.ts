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
  context: string
}

export type ApprovalResponse = {
  id: string
  approved: boolean
  parameters: string
  approver: Approver
}

export type Approver = {
  name: string
  email: string
}
