import React, { useEffect } from 'react'
import { useAuth } from 'react-oidc-context'

export const Home = (): JSX.Element => {
  const auth = useAuth()
  const token = auth.user?.access_token ?? ''
  useEffect(() => {
    void fetch('http://localhost:3001/api/v1/users/me', { headers: { Authorization: 'Bearer ' + token } })
      .then(async (response) => await response.json())
      .then((data) => {
        setUsername(data.name)
      })
  }, [])
  const [username, setUsername] = React.useState<string>('')

  return (
        <div>Hallo {username}</div>
  )
}
