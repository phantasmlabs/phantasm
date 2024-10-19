type Params = {
  id: string
}

export function load({ params }: { params: Params }) {
  return {
    id: params.id
  }
}
