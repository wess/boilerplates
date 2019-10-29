//
//  index.ts
//  express
// 
//  Created by Wess Cope (me@wess.io) on 10/29/19
//  Copyright 2019 Wess Cope
//

import express      from 'express'
import {Connection} from 'typeorm'

import { 
  Resource,
  Main
} from './resources'

export default class App {
  secret:string = process.env.SECRET_AUTH_KEY
  connection:Connection
  context:express.Application

  resources:Array<[string, Resource]> = [
    ["/", new Main()]
  ]

  constructor(connection:Connection) {
    this.connection = connection

    this.context = express()

    this.context.use(express.json())
    this.context.use(express.urlencoded({extended: false}))
  }

  mount() {
    this.resources.forEach(resource => {
      this.context.use(resource[0], resource[1].router())
    })
  }

  run() {
    this.mount()

    const port = process.env.SERVER_PORT || 3000

    this.context.listen(port, () => {
      console.log(`Server started at http://localhost:${port}`)
    })
  }
}
