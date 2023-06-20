import './MenuEntry.css'
import React from 'react'
import { Link } from 'react-router-dom'

export interface MenuEntryProps {
  title: string
  link: string
}

export const MenuEntry = (props: MenuEntryProps): JSX.Element => {
  return (
        <div className="menu-entry">
            <Link to={props.link}>{props.title}</Link>
        </div>
  )
}
