import React from 'react'
import './App.css'
import { MenuEntry, TopMenu } from './components'
import { Groups, Users } from './pages'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { useAuth } from 'react-oidc-context'
import { Home } from './pages/users/Home'

function App (): JSX.Element {
  const auth = useAuth()

  switch (auth.activeNavigator) {
    case 'signinRedirect':
      return <div>Redirecting...</div>
    case 'signoutRedirect':
      return <div>Redirecting...</div>
  }
  if (auth.isLoading) {
    return <div>Loading...</div>
  }
  if (auth.error != null) {
    return <div>{auth.error.name}</div>
  }

  if (auth.isAuthenticated) {
    return (
            <div className="App">
                <BrowserRouter>
                    <TopMenu title="Whats up?">
                        <MenuEntry title="Groups" link="/groups"/>
                        <MenuEntry title="Users" link="/users"/>
                        <MenuEntry title="Home" link="/me"/>
                    </TopMenu>

                    <Routes>
                        <Route index element={<Home/>}/>
                        <Route path="/groups" element={<Groups/>}/>
                        <Route path="/users" element={<Users/>}/>
                        <Route path="/me" element={<Home/>}/>
                    </Routes>
                </BrowserRouter>
            </div>
    )
  } else {
    // eslint-disable-next-line @typescript-eslint/no-misused-promises
    return <button onClick={async () => {
      await auth.signinRedirect()
    }}>Log in</button>
  }
}

export default App
