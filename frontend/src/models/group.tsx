export class Group {
  id?: string
  name: string
  description?: string

  constructor (name: string, description?: string, id?: string) {
    this.name = name
    this.description = description
    this.id = id
  }
}
