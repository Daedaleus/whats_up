import { Card, type CardProps } from '../card/Card'
import React from 'react'

export interface CardGridProps {
  cards: CardProps[]
}

export const CardGrid = (props: CardGridProps): JSX.Element => {
  return (
        <div className="card-grid">
            {props.cards.map((card, index) => {
              return (
                    <div key={index} className="card-grid-item">
                        <Card title={card.title} description={card.description}/>
                    </div>
              )
            })}
        </div>
  )
}
