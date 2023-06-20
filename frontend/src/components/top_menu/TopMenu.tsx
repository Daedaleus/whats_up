import React, { type ReactNode } from 'react'
import './TopMenu.css'

export interface TopMenuProps {
  title: string
  children: ReactNode[]
}

export const TopMenu = (props: TopMenuProps): JSX.Element => {
  return (
        <div className="top-menu">
            <div className="top-menu-title">
                {props.title}
            </div>
            <div className="top-menu-buttons">
                {props.children}
            </div>
        </div>
  )
}
