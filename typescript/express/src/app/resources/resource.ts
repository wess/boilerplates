//
//  resource.ts
//  express
// 
//  Created by Wess Cope (me@wess.io) on 10/29/19
//  Copyright 2019 Wess Cope
//

import {Router} from 'express'

export default abstract class Resource {
  static get id():string {
    return this.constructor.name
  }

  abstract router():Router
}
