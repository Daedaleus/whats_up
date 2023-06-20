import './Card.css'
import React from 'react'

export interface CardProps {
  title: string
  description: string
}

export const Card = (props: CardProps): JSX.Element => {
  return (
        <div className="card">
            <div className="card-title">
                {props.title}
            </div>
            <div className="card-description">
                {props.description}
            </div>
        </div>
  )
}
