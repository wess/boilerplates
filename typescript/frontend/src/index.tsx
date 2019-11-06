//
//  index.tsx
//  boilerplate
// 
//  Created by Wess Cope (me@wess.io) on 11/06/19
//  Copyright 2019 Wess Cope
//
import './styles/app.scss'

import React    from 'react'
import {render} from 'react-dom'

import {
  BrowserRouter as Router
} from 'react-router-dom'


import App from './app'

render(
  <Router>
    <App/>
  </Router>,
  document.getElementById('app')
)
