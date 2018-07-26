module Update exposing (..)

import Navigation
import UrlParser as Url exposing (..)
import Models   exposing (Model)
import Messages exposing(Msg(..))
import Routing  exposing(..)

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    UrlChange location ->
      let 
        newRoute = parseLocation location
      in
        ({ model | route = newRoute }, Cmd.none)
