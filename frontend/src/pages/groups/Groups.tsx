import { type Group } from '../../models/group'
import { CardGrid, type CardProps } from '../../components'
import React, { useEffect } from 'react'
import { useAuth } from 'react-oidc-context'

export const Groups = (): JSX.Element => {
  const auth = useAuth()
  const token = auth.user?.access_token ?? ''
  useEffect(() => {
    void fetch('http://localhost:3001/api/v1/groups', { headers: { Authorization: 'Bearer ' + token } })
      .then(async (response) => await response.json())
      .then((data) => {
        setGroups(data)
      })
  }, [])
  const [groups, setGroups] = React.useState<Group[]>([])

  function getGroupsAsCardProps (): CardProps[] {
    const cardProps: CardProps[] = []
    groups.forEach((group) => {
      cardProps.push({
        title: group.name,
        description: group.description ?? ''
      })
    })

    return cardProps
  }

  return (
        <div>
            <h1>Groups</h1>
            <CardGrid cards={getGroupsAsCardProps()}/>
        </div>
  )
}
