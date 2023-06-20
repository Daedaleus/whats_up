import { CardGrid, type CardProps } from '../../components'
import { type User } from '../../models/user'
import React, { useEffect } from 'react'
import { useAuth } from 'react-oidc-context'

export const Users = (): JSX.Element => {
  const auth = useAuth()
  const token = auth.user?.access_token ?? ''

  useEffect(() => {
    void fetch('http://localhost:3001/api/v1/users', { headers: { Authorization: 'Bearer ' + token } })
      .then(async (response) => await response.json())
      .then((data) => {
        setUsers(data)
      })
  })
  const [users, setUsers] = React.useState<User[]>([])

  function getUsersAsCardProps (): CardProps[] {
    const cardProps: CardProps[] = []

    for (let i = 0; i < users.length; i++) {
      cardProps.push({
        title: users[i].name,
        description: users[i].id ?? ''
      })
    }

    return cardProps
  }

  return (
        <div>
            <h1>Users</h1>
            <CardGrid cards={getUsersAsCardProps()}/>
        </div>
  )
}
