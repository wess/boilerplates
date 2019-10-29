//
//  index.ts
//  express
// 
//  Created by Wess Cope (me@wess.io) on 10/29/19
//  Copyright 2019 Wess Cope
//

import 'dotenv/config'
import 'reflect-metadata'

import {createConnection} from 'typeorm'
import App                from './app'

// const app = new App()
 
// app.run()

createConnection().then(async (conn) => {
  (new App()).run()
})
.catch(err => {
  console.error("Startup Error: ", err)
})
