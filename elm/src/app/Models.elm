module Models exposing (..)

import Routing    exposing (Route)

type alias Model = 
  { route : Route
  }


initialModel : Route -> Model 
initialModel route =
  { route = route
  }
