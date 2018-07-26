module App exposing (main)

import Navigation

import Models   exposing (Model, initialModel)
import Messages exposing (..)
import Update   exposing (update)
import Routing  exposing (Route, route)
import View     exposing (root)

init : Navigation.Location -> (Model, Cmd Msg)
init location =
  let 
    currentRoute = Routing.parseLocation location
  in 
    (initialModel currentRoute, Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

main =
  Navigation.program UrlChange
    { init          = init
    , view          = root
    , update        = update
    , subscriptions = subscriptions
    }

