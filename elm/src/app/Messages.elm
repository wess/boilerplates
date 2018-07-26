module Messages exposing (..)

import Navigation exposing (Location)
import Http

type Msg 
  = UrlChange Location
