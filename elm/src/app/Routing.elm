module Routing exposing (..)

import Navigation
import UrlParser as Url exposing ((</>), (<?>), s, int, stringParam, top)

type Route
  = Home
  | About
  | Contact
  | NotFound


route : Url.Parser (Route -> a) a
route = 
  Url.oneOf
    [ Url.map Home    top
    , Url.map About   (s "about")
    , Url.map Contact (s "contact")
    ]
    
parseLocation : Navigation.Location -> Route
parseLocation location =
  case (Url.parseHash route location) of
    Just route ->
      route
    Nothing ->
      NotFound

routeToString : Route -> String
routeToString route =
  case route of
    Home ->
      "home"
    About ->
      "about"
    Contact ->
      "contact"
    NotFound ->
      "404"

