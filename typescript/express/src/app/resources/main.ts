//
//  main.ts
//  express
// 
//  Created by Wess Cope (me@wess.io) on 10/29/19
//  Copyright 2019 Wess Cope
//

import {
  Router, 
  Request, 
  Response
} from 'express'

import Resource from './resource'

export default class Main extends Resource {
  index(req:Request, res:Response) {
    res.send({hello: 'world'})
  }

  router():Router {
    let router = Router()

    router.get("/", this.index)

    return router
  }
}
